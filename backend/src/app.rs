use axum::{middleware as axum_middleware, Router};
use tower_http::trace::TraceLayer;

use crate::middleware::http_metrics::http_metrics_layer;
use crate::routes;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(routes::routes())
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            http_metrics_layer,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
