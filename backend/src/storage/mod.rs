pub mod filesystem;

use async_trait::async_trait;

#[derive(Debug)]
pub struct StoredFile {
    pub file_path: String,
    pub file_size: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save(
        &self,
        incident_id: uuid::Uuid,
        file_name: &str,
        content_type: &str,
        data: &[u8],
    ) -> anyhow::Result<StoredFile>;
    async fn read(&self, file_path: &str) -> anyhow::Result<bytes::Bytes>;
    async fn delete(&self, file_path: &str) -> anyhow::Result<()>;
}
