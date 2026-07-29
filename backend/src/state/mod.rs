use redis::Client;
use sqlx::postgres::PgPool;

use crate::config::AppConfig;
use crate::metrics::AppMetrics;
use crate::security::Security;
use crate::ws::WsState;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub postgres_pool: PgPool,
    pub redis_client: Client,
    pub security: Security,
    pub metrics: AppMetrics,
    pub ws_state: WsState,
}
