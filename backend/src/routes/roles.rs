use axum::routing::get;
use axum::Router;

use crate::handlers::roles;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(roles::list_roles))
}
