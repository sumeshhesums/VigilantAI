use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::dto::incident::{
    CreateIncidentRequest, IncidentListResponse, IncidentPaginationParams, IncidentResponse,
    UpdateIncidentRequest,
};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::models::Incident;
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::repository::incident_repository::PostgresIncidentRepository;
use crate::services::IncidentService;
use crate::state::AppState;

fn incident_response(incident: Incident) -> IncidentResponse {
    use crate::models::{IncidentSeverity, IncidentStatus};

    IncidentResponse {
        id: incident.id,
        camera_id: incident.camera_id,
        timestamp: incident.timestamp,
        severity: IncidentSeverity::from_db_str(&incident.severity)
            .unwrap_or(IncidentSeverity::Medium),
        status: IncidentStatus::from_db_str(&incident.status).unwrap_or(IncidentStatus::Open),
        event_type: incident.event_type,
        confidence: incident.confidence,
        bounding_box: incident
            .bounding_box
            .and_then(|v| serde_json::from_value(v).ok()),
        metadata: incident.metadata,
        created_at: incident.created_at,
        updated_at: incident.updated_at,
    }
}

// ---------------------------------------------------------------------------
// POST /api/v1/incidents
// Allowed: SecurityAdmin, SystemAdmin, ApiIntegration
// ---------------------------------------------------------------------------
pub async fn create_incident(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Json(body): Json<CreateIncidentRequest>,
) -> Result<(StatusCode, Json<IncidentResponse>), AppError> {
    require_any_role(
        &roles,
        &[Role::SecurityAdmin, Role::SystemAdmin, Role::ApiIntegration],
    )?;

    let repo = PostgresIncidentRepository;
    let service = IncidentService::new(repo);

    let incident = service
        .create(&state.postgres_pool, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("required")
                || msg.contains("must be between")
                || msg.contains("at most")
            {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok((StatusCode::CREATED, Json(incident_response(incident))))
}

// ---------------------------------------------------------------------------
// GET /api/v1/incidents
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn list_incidents(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Query(params): Query<IncidentPaginationParams>,
) -> Result<Json<IncidentListResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let repo = PostgresIncidentRepository;
    let service = IncidentService::new(repo);

    let response = service
        .list(&state.postgres_pool, &params)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(response))
}

// ---------------------------------------------------------------------------
// GET /api/v1/incidents/:id
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn get_incident(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<IncidentResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let repo = PostgresIncidentRepository;
    let service = IncidentService::new(repo);

    let incident = service.get(&state.postgres_pool, id).await.map_err(|e| {
        if e.to_string().contains("not found") {
            AppError::NotFound
        } else {
            AppError::Internal(e)
        }
    })?;

    Ok(Json(incident_response(incident)))
}

// ---------------------------------------------------------------------------
// PATCH /api/v1/incidents/:id
// Allowed: SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn update_incident(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateIncidentRequest>,
) -> Result<Json<IncidentResponse>, AppError> {
    require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin])?;

    let repo = PostgresIncidentRepository;
    let service = IncidentService::new(repo);

    let incident = service
        .update_status(&state.postgres_pool, id, &body)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(incident_response(incident)))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
