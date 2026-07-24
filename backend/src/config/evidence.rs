use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct EvidenceConfig {
    pub storage_path: String,
    pub max_file_size: usize,
}

impl EvidenceConfig {
    pub fn from_env() -> Result<Self> {
        let storage_path =
            std::env::var("EVIDENCE_STORAGE_PATH").unwrap_or_else(|_| "./evidence".to_string());

        let max_file_size = std::env::var("EVIDENCE_MAX_FILE_SIZE")
            .unwrap_or_else(|_| "20971520".to_string())
            .parse::<usize>()
            .context("EVIDENCE_MAX_FILE_SIZE must be a number")?;

        Ok(Self {
            storage_path,
            max_file_size,
        })
    }
}
