use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

impl RedisConfig {
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("REDIS_URL")
            .context("REDIS_URL environment variable not set. Example: redis://127.0.0.1:6379")?;

        Ok(Self { url })
    }
}
