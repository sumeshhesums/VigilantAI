use axum::{extract::State, http::StatusCode, response::IntoResponse, response::Response};

use crate::state::AppState;

pub async fn metrics(State(state): State<AppState>) -> Response {
    let body = state.metrics.encode_metrics();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        body,
    )
        .into_response()
}
