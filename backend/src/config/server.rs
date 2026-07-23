use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn from_env() -> Result<Self> {
        let host = std::env::var("BACKEND_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = std::env::var("BACKEND_PORT")
            .context("BACKEND_PORT environment variable not set")?
            .parse::<u16>()
            .context("BACKEND_PORT must be a valid u16")?;

        Ok(Self { host, port })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
