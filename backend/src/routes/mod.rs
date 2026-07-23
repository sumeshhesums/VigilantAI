pub mod health;

use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/api/v1", health::routes())
}
