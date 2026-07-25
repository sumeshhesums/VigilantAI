use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::dto::evidence::{EvidenceListResponse, EvidencePaginationParams, EvidenceResponse};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::repository::evidence_repository::PostgresEvidenceRepository;
use crate::services::EvidenceService;
use crate::state::AppState;
use crate::storage::filesystem::FilesystemStorage;

fn evidence_response(evidence: crate::models::Evidence) -> EvidenceResponse {
    EvidenceResponse {
        id: evidence.id,
        incident_id: evidence.incident_id,
        file_name: evidence.file_name,
        content_type: evidence.content_type,
        file_size: evidence.file_size,
        sha256: evidence.sha256,
        width: evidence.width,
        height: evidence.height,
        created_at: evidence.created_at,
    }
}

fn make_service(
    state: &AppState,
) -> EvidenceService<PostgresEvidenceRepository, FilesystemStorage> {
    let storage = FilesystemStorage::new(&state.config.evidence.storage_path);
    let repo = PostgresEvidenceRepository;
    EvidenceService::new(repo, storage)
}

// ---------------------------------------------------------------------------
// POST /api/v1/incidents/:id/evidence
// Allowed: Operator, SecurityAnalyst, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn upload_evidence(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(incident_id): Path<uuid::Uuid>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<EvidenceResponse>), AppError> {
    require_any_role(
        &roles,
        &[
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let service = make_service(&state);

    let mut file_name = String::new();
    let mut content_type = String::new();
    let mut file_data: Vec<u8> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("multipart error: {e}")))?
    {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "file" => {
                file_name = field.file_name().unwrap_or("unknown").to_string();
                content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();
                file_data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::Internal(anyhow::anyhow!("failed to read field: {e}")))?
                    .to_vec();
            }
            "content_type" => {
                let ct = field.text().await.map_err(|e| {
                    AppError::Internal(anyhow::anyhow!("failed to read field: {e}"))
                })?;
                if !ct.is_empty() {
                    content_type = ct;
                }
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return Err(AppError::Conflict("no file provided".to_string()));
    }

    let evidence = service
        .save_image(
            &state.postgres_pool,
            incident_id,
            &file_name,
            &content_type,
            &file_data,
        )
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("unsupported content type") || msg.contains("file too large") {
                AppError::Conflict(msg)
            } else if msg.contains("incident not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok((StatusCode::CREATED, Json(evidence_response(evidence))))
}

// ---------------------------------------------------------------------------
// GET /api/v1/incidents/:id/evidence
// Allowed: Viewer, Operator, SecurityAnalyst, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn list_evidence(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(incident_id): Path<uuid::Uuid>,
    Query(params): Query<EvidencePaginationParams>,
) -> Result<Json<EvidenceListResponse>, AppError> {
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

    let service = make_service(&state);

    let response = service
        .list_by_incident(&state.postgres_pool, incident_id, &params)
        .await
        .map_err(AppError::Internal)?;

    Ok(Json(response))
}

// ---------------------------------------------------------------------------
// GET /api/v1/evidence/:id
// Allowed: Viewer, Operator, SecurityAnalyst, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn get_evidence(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<EvidenceResponse>, AppError> {
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

    let service = make_service(&state);

    let evidence = service.get(&state.postgres_pool, id).await.map_err(|e| {
        if e.to_string().contains("not found") {
            AppError::NotFound
        } else {
            AppError::Internal(e)
        }
    })?;

    Ok(Json(evidence_response(evidence)))
}

// ---------------------------------------------------------------------------
// DELETE /api/v1/evidence/:id
// Allowed: SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn delete_evidence(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin])?;

    let service = make_service(&state);

    service
        .delete(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
