use axum::routing::get;
use axum::Router;

use crate::handlers::dashboard;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/kpis", get(dashboard::get_kpis))
        .route("/live-stats", get(dashboard::get_live_stats))
        .route("/alert-trends", get(dashboard::get_alert_trends))
        .route("/incidents-summary", get(dashboard::get_incidents_summary))
}
