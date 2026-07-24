use axum::routing::{get, post};
use axum::Router;

use crate::handlers::user;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(user::list_users).post(user::create_user))
        .route(
            "/{id}",
            get(user::get_user)
                .patch(user::update_user)
                .delete(user::delete_user),
        )
        .route(
            "/{id}/roles",
            post(user::assign_role).delete(user::remove_role),
        )
}
