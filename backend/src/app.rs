use axum::{middleware as axum_middleware, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::middleware::http_metrics::http_metrics_layer;
use crate::routes;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::routes())
        .layer(cors)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            http_metrics_layer,
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
