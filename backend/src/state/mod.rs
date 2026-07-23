use redis::Client;
use sqlx::postgres::PgPool;

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub postgres_pool: PgPool,
    pub redis_client: Client,
}
