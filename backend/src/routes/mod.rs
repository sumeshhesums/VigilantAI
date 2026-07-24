pub mod admin;
pub mod auth;
pub mod camera;
pub mod evidence;
pub mod health;
pub mod incident;
pub mod user;

use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/api/v1/auth", auth::routes())
        .nest("/api/v1/admin", admin::routes())
        .nest("/api/v1/users", user::routes())
        .nest("/api/v1/cameras", camera::routes())
        .nest("/api/v1/incidents", incident::routes())
        .nest("/api/v1/evidence", evidence::routes())
        .nest("/api/v1", health::routes())
}
