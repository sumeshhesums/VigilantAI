pub mod database;
pub mod evidence;
pub mod jwt;
pub mod redis;
pub mod server;

use anyhow::Result;

use self::database::DatabaseConfig;
use self::evidence::EvidenceConfig;
pub use self::jwt::JwtConfig;
use self::redis::RedisConfig;
use self::server::ServerConfig;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub evidence: EvidenceConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let server = ServerConfig::from_env()?;
        let database = DatabaseConfig::from_env()?;
        let redis = RedisConfig::from_env()?;
        let jwt = JwtConfig::from_env()?;
        let evidence = EvidenceConfig::from_env()?;

        Ok(Self {
            server,
            database,
            redis,
            jwt,
            evidence,
        })
    }

    pub fn address(&self) -> String {
        self.server.address()
    }
}
