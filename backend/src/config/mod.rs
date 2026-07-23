use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let host = std::env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        Ok(Self { host, port })
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
