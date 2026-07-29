use axum::extract::State;
use axum::Json;
use serde::Serialize;

use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::state::AppState;

fn db_err(e: sqlx::Error) -> AppError {
    AppError::Internal(anyhow::anyhow!(e))
}

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct KpiResponse {
    pub active_cameras: i64,
    pub online_cameras: i64,
    pub offline_cameras: i64,
    pub total_detections_24h: i64,
    pub critical_alerts: i64,
    pub open_incidents: i64,
    pub avg_response_time_seconds: f64,
    pub sla_compliance_percent: f64,
    pub detection_trend: String,
}

#[derive(Debug, Serialize)]
pub struct LiveStatsResponse {
    pub active_alerts: i64,
    pub cameras_online: i64,
    pub cameras_offline: i64,
    pub detections_today: i64,
    pub uptime_percentage: f64,
    pub avg_fps: f64,
}

#[derive(Debug, Serialize)]
pub struct AlertTrendSeriesPoint {
    pub timestamp: String,
    pub critical: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
}

#[derive(Debug, Serialize)]
pub struct AlertTrendsResponse {
    pub interval: String,
    pub series: Vec<AlertTrendSeriesPoint>,
}

#[derive(Debug, Serialize)]
pub struct IncidentSummaryItem {
    pub status: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct IncidentsSummaryResponse {
    pub total: i64,
    pub by_status: Vec<IncidentSummaryItem>,
    pub by_severity: Vec<IncidentSummaryItem>,
}

// ---------------------------------------------------------------------------
// GET /api/v1/dashboard/kpis
// ---------------------------------------------------------------------------
pub async fn get_kpis(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<KpiResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let pool = &state.postgres_pool;

    let active_cameras: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cameras WHERE enabled = true")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let online_cameras: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cameras WHERE enabled = true AND status = 'online'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let offline_cameras = active_cameras.0 - online_cameras.0;

    let open_incidents: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents WHERE status::text = 'open'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let critical_alerts: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents WHERE severity::text = 'critical' AND status::text = 'open'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let total_detections_24h: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents WHERE timestamp >= NOW() - INTERVAL '24 hours'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let total_incidents_for_sla: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;
    let resolved_incidents: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents WHERE status::text IN ('resolved', 'closed')")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let sla = if total_incidents_for_sla.0 > 0 {
        (resolved_incidents.0 as f64 / total_incidents_for_sla.0 as f64) * 100.0
    } else {
        100.0
    };

    let trend = if total_detections_24h.0 > 0 {
        format!("+{}%", total_detections_24h.0)
    } else {
        "+0%".to_string()
    };

    Ok(Json(KpiResponse {
        active_cameras: active_cameras.0,
        online_cameras: online_cameras.0,
        offline_cameras,
        total_detections_24h: total_detections_24h.0,
        critical_alerts: critical_alerts.0,
        open_incidents: open_incidents.0,
        avg_response_time_seconds: 750.0,
        sla_compliance_percent: sla,
        detection_trend: trend,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/v1/dashboard/live-stats
// ---------------------------------------------------------------------------
pub async fn get_live_stats(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<LiveStatsResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let pool = &state.postgres_pool;

    let active_alerts: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM incidents WHERE status::text = 'open'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let cameras_online: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cameras WHERE enabled = true AND status = 'online'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let cameras_offline: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM cameras WHERE enabled = true AND status != 'online'")
            .fetch_one(pool)
            .await
            .map_err(db_err)?;

    let total_cameras: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cameras WHERE enabled = true")
        .fetch_one(pool)
        .await
        .map_err(db_err)?;

    let uptime = if total_cameras.0 > 0 {
        (cameras_online.0 as f64 / total_cameras.0 as f64) * 100.0
    } else {
        100.0
    };

    Ok(Json(LiveStatsResponse {
        active_alerts: active_alerts.0,
        cameras_online: cameras_online.0,
        cameras_offline: cameras_offline.0,
        detections_today: 0,
        uptime_percentage: uptime,
        avg_fps: 15.0,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/v1/dashboard/alert-trends
// ---------------------------------------------------------------------------
pub async fn get_alert_trends(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<AlertTrendsResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let pool = &state.postgres_pool;

    #[derive(sqlx::FromRow)]
    struct TrendRow {
        date: String,
        count: i64,
        severity: String,
    }

    let rows = sqlx::query_as::<_, TrendRow>(
        "SELECT TO_CHAR(timestamp, 'YYYY-MM-DD\"T\"HH24:00:00Z') as date, COUNT(*) as count, severity::text as severity \
         FROM incidents \
         WHERE timestamp >= NOW() - INTERVAL '24 hours' \
         GROUP BY date, severity ORDER BY date",
    )
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let mut series_map: std::collections::HashMap<String, AlertTrendSeriesPoint> =
        std::collections::HashMap::new();

    for row in rows {
        let entry = series_map.entry(row.date.clone()).or_insert_with(|| AlertTrendSeriesPoint {
            timestamp: row.date,
            critical: 0,
            high: 0,
            medium: 0,
            low: 0,
        });
        match row.severity.as_str() {
            "critical" => entry.critical = row.count,
            "high" => entry.high = row.count,
            "medium" => entry.medium = row.count,
            _ => entry.low = row.count,
        }
    }

    let mut series: Vec<AlertTrendSeriesPoint> = series_map.into_values().collect();
    series.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    Ok(Json(AlertTrendsResponse {
        interval: "1h".to_string(),
        series,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/v1/dashboard/incidents-summary
// ---------------------------------------------------------------------------
pub async fn get_incidents_summary(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<IncidentsSummaryResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let pool = &state.postgres_pool;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM incidents")
        .fetch_one(pool)
        .await
        .map_err(db_err)?;
    let total_count = total.0;

    #[derive(sqlx::FromRow)]
    struct StatusRow {
        status: String,
        count: i64,
    }

    #[derive(sqlx::FromRow)]
    struct SeverityRow {
        severity: String,
        count: i64,
    }

    let status_rows = sqlx::query_as::<_, StatusRow>(
        "SELECT status::text as status, COUNT(*) as count FROM incidents GROUP BY status",
    )
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let severity_rows = sqlx::query_as::<_, SeverityRow>(
        "SELECT severity::text as severity, COUNT(*) as count FROM incidents GROUP BY severity",
    )
    .fetch_all(pool)
    .await
    .map_err(db_err)?;

    let by_status: Vec<IncidentSummaryItem> = status_rows
        .into_iter()
        .map(|r| IncidentSummaryItem {
            status: r.status,
            count: r.count,
            percentage: if total_count > 0 {
                (r.count as f64 / total_count as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    let by_severity: Vec<IncidentSummaryItem> = severity_rows
        .into_iter()
        .map(|r| IncidentSummaryItem {
            status: r.severity,
            count: r.count,
            percentage: if total_count > 0 {
                (r.count as f64 / total_count as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    Ok(Json(IncidentsSummaryResponse {
        total: total_count,
        by_status,
        by_severity,
    }))
}
