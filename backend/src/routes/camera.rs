use axum::routing::{get, post};
use axum::Router;

use crate::handlers::camera;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(camera::list_cameras).post(camera::create_camera))
        .route(
            "/{id}",
            get(camera::get_camera)
                .patch(camera::update_camera)
                .delete(camera::delete_camera),
        )
        .route("/{id}/enable", post(camera::enable_camera))
        .route("/{id}/disable", post(camera::disable_camera))
}
