pub mod health;

use axum::Router;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/api/v1", health::routes())
}
