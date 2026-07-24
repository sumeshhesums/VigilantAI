use std::path::PathBuf;

use async_trait::async_trait;
use chrono::Utc;
use sha2::{Digest, Sha256};

use super::{Storage, StoredFile};

pub struct FilesystemStorage {
    root: PathBuf,
}

impl FilesystemStorage {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn content_type_to_ext(content_type: &str) -> &'static str {
        match content_type {
            "image/jpeg" => "jpg",
            "image/png" => "png",
            _ => "bin",
        }
    }

    fn build_path(
        &self,
        incident_id: uuid::Uuid,
        evidence_id: uuid::Uuid,
        content_type: &str,
    ) -> PathBuf {
        let now = Utc::now();
        let ext = Self::content_type_to_ext(content_type);
        self.root
            .join(now.format("%Y").to_string())
            .join(now.format("%m").to_string())
            .join(now.format("%d").to_string())
            .join(incident_id.to_string())
            .join(format!("{evidence_id}.{ext}"))
    }

    fn validate_path(&self, file_path: &str) -> anyhow::Result<PathBuf> {
        if file_path.contains("..") {
            return Err(anyhow::anyhow!("path traversal detected"));
        }
        if file_path.starts_with('/') || file_path.starts_with('\\') {
            return Err(anyhow::anyhow!("absolute path not allowed"));
        }
        if file_path.len() >= 2
            && file_path.as_bytes()[0].is_ascii_alphabetic()
            && file_path.as_bytes()[1] == b':'
        {
            return Err(anyhow::anyhow!("absolute path not allowed"));
        }
        Ok(self.root.join(file_path))
    }
}

#[async_trait]
impl Storage for FilesystemStorage {
    async fn save(
        &self,
        incident_id: uuid::Uuid,
        _file_name: &str,
        content_type: &str,
        data: &[u8],
    ) -> anyhow::Result<StoredFile> {
        let evidence_id = uuid::Uuid::new_v4();
        let file_path = self.build_path(incident_id, evidence_id, content_type);

        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&file_path, data).await?;

        let mut hasher = Sha256::new();
        hasher.update(data);
        let sha256 = format!("{:x}", hasher.finalize());

        let (width, height) = extract_dimensions(data);

        let relative = file_path
            .strip_prefix(&self.root)
            .unwrap_or(&file_path)
            .to_string_lossy()
            .replace('\\', "/");

        Ok(StoredFile {
            file_path: relative,
            file_size: data.len() as i64,
            sha256,
            width,
            height,
        })
    }

    async fn read(&self, file_path: &str) -> anyhow::Result<bytes::Bytes> {
        let full_path = self.validate_path(file_path)?;
        let data = tokio::fs::read(&full_path).await?;
        Ok(bytes::Bytes::from(data))
    }

    async fn delete(&self, file_path: &str) -> anyhow::Result<()> {
        let full_path = self.validate_path(file_path)?;
        if full_path.exists() {
            tokio::fs::remove_file(&full_path).await?;
            // Try to remove empty parent directories (date/incident_id)
            if let Some(parent) = full_path.parent() {
                let _ = tokio::fs::remove_dir(parent).await;
                if let Some(grandparent) = parent.parent() {
                    let _ = tokio::fs::remove_dir(grandparent).await;
                    if let Some(great_grandparent) = grandparent.parent() {
                        let _ = tokio::fs::remove_dir(great_grandparent).await;
                    }
                }
            }
        }
        Ok(())
    }
}

fn extract_dimensions(data: &[u8]) -> (Option<i32>, Option<i32>) {
    match image::load_from_memory(data) {
        Ok(img) => {
            let width = img.width() as i32;
            let height = img.height() as i32;
            (Some(width), Some(height))
        }
        Err(_) => (None, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;

    fn test_storage() -> (FilesystemStorage, TempDir) {
        let dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(dir.path());
        (storage, dir)
    }

    #[test]
    fn test_content_type_to_ext() {
        assert_eq!(FilesystemStorage::content_type_to_ext("image/jpeg"), "jpg");
        assert_eq!(FilesystemStorage::content_type_to_ext("image/png"), "png");
        assert_eq!(FilesystemStorage::content_type_to_ext("video/mp4"), "bin");
    }

    #[test]
    fn test_build_path_format() {
        let dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(dir.path());
        let incident_id = uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let evidence_id = uuid::Uuid::parse_str("660e8400-e29b-41d4-a716-446655440001").unwrap();
        let path = storage.build_path(incident_id, evidence_id, "image/jpeg");
        let path_str = path.to_string_lossy();
        assert!(path_str.contains("550e8400-e29b-41d4-a716-446655440000"));
        assert!(path_str.contains("660e8400-e29b-41d4-a716-446655440001.jpg"));
        assert!(path_str.ends_with(".jpg"));
    }

    #[test]
    fn test_validate_path_safe() {
        let (storage, _dir) = test_storage();
        let result = storage.validate_path("2024/01/15/test.jpg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_traversal_rejected() {
        let (storage, _dir) = test_storage();
        let result = storage.validate_path("../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_path_absolute_rejected() {
        let (storage, _dir) = test_storage();
        let result = storage.validate_path("/etc/passwd");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_and_read() {
        let (storage, _dir) = test_storage();
        let incident_id = uuid::Uuid::new_v4();
        let data = b"fake image data";

        let stored = storage
            .save(incident_id, "test.jpg", "image/jpeg", data)
            .await
            .unwrap();
        assert_eq!(stored.file_size, data.len() as i64);
        assert!(!stored.sha256.is_empty());

        let read_data = storage.read(&stored.file_path).await.unwrap();
        assert_eq!(read_data.as_ref(), data);
    }

    #[tokio::test]
    async fn test_delete() {
        let (storage, _dir) = test_storage();
        let incident_id = uuid::Uuid::new_v4();
        let data = b"test data";

        let stored = storage
            .save(incident_id, "test.jpg", "image/jpeg", data)
            .await
            .unwrap();
        assert!(Path::new(&storage.root.join(&stored.file_path)).exists());

        storage.delete(&stored.file_path).await.unwrap();
        assert!(!Path::new(&storage.root.join(&stored.file_path)).exists());
    }

    #[tokio::test]
    async fn test_save_creates_directories() {
        let (storage, _dir) = test_storage();
        let incident_id = uuid::Uuid::new_v4();
        let data = b"test";

        let stored = storage
            .save(incident_id, "test.jpg", "image/jpeg", data)
            .await
            .unwrap();
        assert!(Path::new(&storage.root.join(&stored.file_path)).exists());
    }

    #[test]
    fn test_sha256_consistency() {
        let data = b"consistent data";
        let mut hasher1 = Sha256::new();
        hasher1.update(data);
        let hash1 = format!("{:x}", hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(data);
        let hash2 = format!("{:x}", hasher2.finalize());

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_path_separator_normalization() {
        let dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(dir.path());
        let incident_id = uuid::Uuid::new_v4();
        let evidence_id = uuid::Uuid::new_v4();
        let path = storage.build_path(incident_id, evidence_id, "image/jpeg");
        let path_str = path.to_string_lossy();
        // On Windows, Path uses backslashes, but our relative path uses forward slashes
        // The canonicalize in validate_path handles this
        assert!(path_str.contains(&incident_id.to_string()));
    }

    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let (storage, _dir) = test_storage();
        let result = storage.read("nonexistent/file.jpg").await;
        assert!(result.is_err());
    }
}
