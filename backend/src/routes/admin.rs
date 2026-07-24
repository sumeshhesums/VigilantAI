use axum::routing::get;
use axum::Router;

use crate::handlers::admin::admin_health;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(admin_health))
}
