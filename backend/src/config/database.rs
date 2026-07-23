use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL environment variable not set. Example: postgres://user:pass@localhost:5432/vigilantai")?;

        Ok(Self { url })
    }
}
