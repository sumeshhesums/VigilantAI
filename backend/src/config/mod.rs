pub mod database;
pub mod redis;
pub mod server;

use anyhow::Result;

use self::database::DatabaseConfig;
use self::redis::RedisConfig;
use self::server::ServerConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let server = ServerConfig::from_env()?;
        let database = DatabaseConfig::from_env()?;
        let redis = RedisConfig::from_env()?;

        Ok(Self {
            server,
            database,
            redis,
        })
    }

    pub fn address(&self) -> String {
        self.server.address()
    }
}
