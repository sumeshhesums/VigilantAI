use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::dto::evidence::EvidencePaginationParams;
use crate::models::{CreateEvidence, Evidence};

#[async_trait]
pub trait EvidenceRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, evidence: &CreateEvidence) -> anyhow::Result<Evidence>;
    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Evidence>>;
    async fn list_by_incident(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        params: &EvidencePaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Evidence>>;
    async fn count_by_incident(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
    ) -> anyhow::Result<i64>;
    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool>;
    async fn find_by_sha256(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        sha256: &str,
    ) -> anyhow::Result<Option<Evidence>>;
}

pub struct PostgresEvidenceRepository;

#[async_trait]
impl EvidenceRepository for PostgresEvidenceRepository {
    async fn create(&self, pool: &PgPool, evidence: &CreateEvidence) -> anyhow::Result<Evidence> {
        let record = sqlx::query_as::<_, Evidence>(
            "INSERT INTO evidence (incident_id, file_name, file_path, content_type, file_size, sha256, width, height) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        )
        .bind(evidence.incident_id)
        .bind(&evidence.file_name)
        .bind(&evidence.file_path)
        .bind(&evidence.content_type)
        .bind(evidence.file_size)
        .bind(&evidence.sha256)
        .bind(evidence.width)
        .bind(evidence.height)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Evidence>> {
        let record = sqlx::query_as::<_, Evidence>("SELECT * FROM evidence WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list_by_incident(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        _params: &EvidencePaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Evidence>> {
        let records = sqlx::query_as::<_, Evidence>(
            "SELECT * FROM evidence WHERE incident_id = $1 ORDER BY created_at DESC OFFSET $2 LIMIT $3",
        )
        .bind(incident_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    async fn count_by_incident(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
    ) -> anyhow::Result<i64> {
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM evidence WHERE incident_id = $1")
                .bind(incident_id)
                .fetch_one(pool)
                .await?;

        Ok(count)
    }

    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM evidence WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_by_sha256(
        &self,
        pool: &PgPool,
        incident_id: uuid::Uuid,
        sha256: &str,
    ) -> anyhow::Result<Option<Evidence>> {
        let record = sqlx::query_as::<_, Evidence>(
            "SELECT * FROM evidence WHERE incident_id = $1 AND sha256 = $2 LIMIT 1",
        )
        .bind(incident_id)
        .bind(sha256)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::evidence::EvidencePaginationParams;

    #[test]
    fn test_default_pagination_params() {
        let params = EvidencePaginationParams {
            page: None,
            per_page: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_custom_pagination_params() {
        let params = EvidencePaginationParams {
            page: Some(5),
            per_page: Some(10),
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 40);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_max_per_page() {
        let params = EvidencePaginationParams {
            page: Some(1),
            per_page: Some(500),
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }
}
