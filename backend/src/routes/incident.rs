use axum::routing::get;
use axum::Router;

use crate::handlers::incident;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(incident::list_incidents).post(incident::create_incident),
        )
        .route(
            "/{id}",
            get(incident::get_incident).patch(incident::update_incident),
        )
}
