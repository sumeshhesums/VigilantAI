use axum::routing::{get, post};
use axum::Router;

use crate::handlers::notification;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/send", post(notification::send_notification))
        .route("/retry", post(notification::retry_notifications))
        .route("/", get(notification::list_notifications))
        .route("/{id}", get(notification::get_notification))
}
