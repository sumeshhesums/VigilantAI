use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

use crate::dto::evidence::{EvidenceListResponse, EvidencePaginationParams, EvidenceResponse};
use crate::models::{CreateEvidence, Evidence};
use crate::repository::evidence_repository::EvidenceRepository;
use crate::storage::Storage;

const MAX_FILE_SIZE: usize = 20 * 1024 * 1024; // 20 MB
const ALLOWED_CONTENT_TYPES: &[&str] = &["image/jpeg", "image/png"];

pub struct EvidenceService<R: EvidenceRepository, S: Storage> {
    repository: R,
    storage: S,
}

impl<R: EvidenceRepository, S: Storage> EvidenceService<R, S> {
    pub fn new(repository: R, storage: S) -> Self {
        Self {
            repository,
            storage,
        }
    }

    /// Save image evidence. Checks for SHA-256 deduplication within the incident.
    pub async fn save_image(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        file_name: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<Evidence> {
        Self::validate_content_type(content_type)?;
        Self::validate_file_size(data)?;

        let sha256 = Self::compute_sha256(data);

        // Check for duplicate within this incident
        if let Some(existing) = self
            .repository
            .find_by_sha256(pool, incident_id, &sha256)
            .await?
        {
            return Ok(existing);
        }

        // Verify incident exists
        let incident_exists = self.incident_exists(pool, incident_id).await?;
        if !incident_exists {
            return Err(anyhow!("incident not found"));
        }

        let stored = self
            .storage
            .save(incident_id, file_name, content_type, data)
            .await?;

        let create = CreateEvidence {
            incident_id,
            file_name: file_name.to_string(),
            file_path: stored.file_path,
            content_type: content_type.to_string(),
            file_size: stored.file_size,
            sha256: stored.sha256,
            width: stored.width,
            height: stored.height,
        };

        let evidence = self.repository.create(pool, &create).await?;
        Ok(evidence)
    }

    /// Get evidence by ID.
    pub async fn get(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Evidence> {
        self.repository
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("evidence not found"))
    }

    /// List evidence for an incident.
    pub async fn list_by_incident(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        params: &EvidencePaginationParams,
    ) -> Result<EvidenceListResponse> {
        let (offset, limit) = params.offset_limit();
        let evidence = self
            .repository
            .list_by_incident(pool, incident_id, params, offset as i64, limit as i64)
            .await?;
        let total = self.repository.count_by_incident(pool, incident_id).await?;
        let page = params.page.unwrap_or(1).max(1);

        let responses: Vec<EvidenceResponse> =
            evidence.into_iter().map(Self::evidence_response).collect();

        Ok(EvidenceListResponse {
            evidence: responses,
            total,
            page,
            per_page: limit,
            pages: if limit > 0 {
                (total as u32).div_ceil(limit).max(1)
            } else {
                1
            },
        })
    }

    /// Delete evidence and its file.
    pub async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> Result<()> {
        let evidence = self.get(pool, id).await?;

        self.storage.delete(&evidence.file_path).await?;
        self.repository.delete(pool, id).await?;

        Ok(())
    }

    /// Get file bytes for download.
    pub async fn get_file(&self, evidence: &Evidence) -> Result<bytes::Bytes> {
        self.storage.read(&evidence.file_path).await
    }

    /// Verify file integrity by recomputing SHA-256.
    pub async fn verify_checksum(&self, pool: &PgPool, id: uuid::Uuid) -> Result<bool> {
        let evidence = self.get(pool, id).await?;
        let data = self.storage.read(&evidence.file_path).await?;
        let computed = Self::compute_sha256(&data);
        Ok(computed == evidence.sha256)
    }

    fn validate_content_type(content_type: &str) -> Result<()> {
        if !ALLOWED_CONTENT_TYPES.contains(&content_type) {
            return Err(anyhow!(
                "unsupported content type: {content_type}. Allowed: {}",
                ALLOWED_CONTENT_TYPES.join(", ")
            ));
        }
        Ok(())
    }

    fn validate_file_size(data: &[u8]) -> Result<()> {
        if data.len() > MAX_FILE_SIZE {
            return Err(anyhow!(
                "file too large: {} bytes. Maximum: {} bytes",
                data.len(),
                MAX_FILE_SIZE
            ));
        }
        Ok(())
    }

    fn compute_sha256(data: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    async fn incident_exists(&self, pool: &PgPool, incident_id: uuid::Uuid) -> Result<bool> {
        use crate::repository::incident_repository::IncidentRepository;
        let repo = crate::repository::incident_repository::PostgresIncidentRepository;
        let exists = repo.find_by_id(pool, incident_id).await?.is_some();
        Ok(exists)
    }

    fn evidence_response(evidence: Evidence) -> EvidenceResponse {
        EvidenceResponse {
            id: evidence.id,
            incident_id: evidence.incident_id,
            file_name: evidence.file_name,
            content_type: evidence.content_type,
            file_size: evidence.file_size,
            sha256: evidence.sha256,
            width: evidence.width,
            height: evidence.height,
            created_at: evidence.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_content_type_valid() {
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_content_type("image/jpeg")
        .is_ok());
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_content_type("image/png")
        .is_ok());
    }

    #[test]
    fn test_validate_content_type_invalid() {
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_content_type("image/gif")
        .is_err());
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_content_type("video/mp4")
        .is_err());
    }

    #[test]
    fn test_validate_file_size_valid() {
        let data = vec![0u8; 1024];
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_file_size(&data)
        .is_ok());
    }

    #[test]
    fn test_validate_file_size_too_large() {
        let data = vec![0u8; MAX_FILE_SIZE + 1];
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_file_size(&data)
        .is_err());
    }

    #[test]
    fn test_validate_file_size_exact_limit() {
        let data = vec![0u8; MAX_FILE_SIZE];
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_file_size(&data)
        .is_ok());
    }

    #[test]
    fn test_compute_sha256_consistent() {
        let data = b"test data for hashing";
        let hash1 = EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::compute_sha256(data);
        let hash2 = EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::compute_sha256(data);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 hex is 64 chars
    }

    #[test]
    fn test_compute_sha256_different_data() {
        let data1 = b"data one";
        let data2 = b"data two";
        let hash1 = EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::compute_sha256(data1);
        let hash2 = EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::compute_sha256(data2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_validate_content_type_case_sensitive() {
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_content_type("Image/JPEG")
        .is_err());
    }

    #[test]
    fn test_validate_file_size_empty() {
        let data = vec![0u8; 0];
        assert!(EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::validate_file_size(&data)
        .is_ok());
    }

    #[test]
    fn test_evidence_response_conversion() {
        use chrono::Utc;
        use uuid::Uuid;

        let evidence = Evidence {
            id: Uuid::new_v4(),
            incident_id: Uuid::new_v4(),
            file_name: "test.jpg".to_string(),
            file_path: "2024/01/15/test.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            file_size: 1024,
            sha256: "abc123".to_string(),
            width: Some(1920),
            height: Some(1080),
            created_at: Utc::now(),
        };

        let response = EvidenceService::<
            crate::repository::evidence_repository::PostgresEvidenceRepository,
            crate::storage::filesystem::FilesystemStorage,
        >::evidence_response(evidence.clone());

        assert_eq!(response.id, evidence.id);
        assert_eq!(response.file_name, evidence.file_name);
        assert_eq!(response.width, Some(1920));
        assert_eq!(response.height, Some(1080));
    }
}
