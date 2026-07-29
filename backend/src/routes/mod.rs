pub mod admin;
pub mod auth;
pub mod camera;
pub mod dashboard;
pub mod evidence;
pub mod health;
pub mod incident;
pub mod metrics;
pub mod notification;
pub mod roles;
pub mod user;
use crate::state::AppState;
use crate::ws;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/admin", admin::routes())
        .nest("/api/v1/users", user::routes())
        .nest("/api/v1/cameras", camera::routes())
        .nest("/api/v1/incidents", incident::routes())
        .nest("/api/v1/evidence", evidence::routes())
        .nest("/api/v1/notifications", notification::routes())
        .nest("/api/v1/roles", roles::routes())
        .nest("/api/v1/dashboard", dashboard::routes())
        .route("/api/v1/health", axum::routing::get(health::health))
        .route("/metrics", axum::routing::get(metrics::metrics))
        .merge(ws::websocket_routes())
}
