use axum::routing::get;
use axum::Router;

use crate::handlers::evidence;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/:id",
        get(evidence::get_evidence).delete(evidence::delete_evidence),
    )
}
