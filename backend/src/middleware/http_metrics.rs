use axum::body::Body;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

use crate::state::AppState;

pub async fn http_metrics_layer(
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = Instant::now();

    state.metrics.active_connections.inc();
    let response = next.run(request).await;
    state.metrics.active_connections.dec();

    let elapsed = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    state
        .metrics
        .http_requests_total
        .with_label_values(&[&method, &path, &status])
        .inc();
    state
        .metrics
        .http_request_duration_seconds
        .with_label_values(&[&method, &path])
        .observe(elapsed);

    response
}
