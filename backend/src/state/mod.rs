use redis::Client;
use sqlx::postgres::PgPool;

use crate::config::AppConfig;
use crate::security::Security;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub postgres_pool: PgPool,
    pub redis_client: Client,
    pub security: Security,
}
