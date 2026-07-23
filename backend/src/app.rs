use axum::Router;
use tower_http::trace::TraceLayer;

use crate::routes;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
