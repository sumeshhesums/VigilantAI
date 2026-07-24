use std::collections::HashSet;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::dto::camera::{
    CameraListResponse, CameraPaginationParams, CameraResponse, CreateCameraRequest,
    UpdateCameraRequest,
};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::models::Camera;
use crate::rbac::roles::Role;
use crate::repository::camera_repository::PostgresCameraRepository;
use crate::services::CameraService;
use crate::state::AppState;

/// Check that the user holds at least one of the required roles.
/// Returns `Err(AppError::Forbidden)` on failure.
fn require_any_role(user_roles: &HashSet<Role>, allowed: &[Role]) -> Result<(), AppError> {
    for role in allowed {
        if user_roles.contains(role) {
            return Ok(());
        }
    }
    Err(AppError::Forbidden(format!(
        "one of the following roles is required: {}",
        allowed
            .iter()
            .map(|r| r.as_db_str())
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

fn camera_response(camera: Camera) -> CameraResponse {
    CameraResponse {
        id: camera.id,
        name: camera.name,
        location: camera.location,
        rtsp_url: camera.rtsp_url,
        status: camera.status,
        enabled: camera.enabled,
        fps: camera.fps,
        resolution: camera.resolution,
        last_seen: camera.last_seen,
        created_at: camera.created_at,
        updated_at: camera.updated_at,
    }
}

// ---------------------------------------------------------------------------
// GET /api/v1/cameras
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn list_cameras(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Query(params): Query<CameraPaginationParams>,
) -> Result<Json<CameraListResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let (offset, limit) = params.offset_limit();
    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let (cameras, total) = service
        .list_cameras(&state.postgres_pool, offset, limit)
        .await
        .map_err(AppError::Internal)?;

    let page = params.page.unwrap_or(1).max(1);

    Ok(Json(CameraListResponse {
        cameras: cameras.into_iter().map(camera_response).collect(),
        total,
        page,
        per_page: limit,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/v1/cameras/:id
// Allowed: Viewer, Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn get_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CameraResponse>, AppError> {
    require_any_role(
        &roles,
        &[
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ],
    )?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let camera = service
        .get_camera(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(camera_response(camera)))
}

// ---------------------------------------------------------------------------
// POST /api/v1/cameras
// Allowed: SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn create_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Json(body): Json<CreateCameraRequest>,
) -> Result<(StatusCode, Json<CameraResponse>), AppError> {
    require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin])?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let camera = service
        .create_camera(&state.postgres_pool, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("already exists")
                || msg.contains("already registered")
                || msg.contains("must be")
                || msg.contains("required")
                || msg.contains("must start with")
                || msg.contains("between")
                || msg.contains("WIDTHxHEIGHT")
                || msg.contains("exceeds")
                || msg.contains("must be positive")
            {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok((StatusCode::CREATED, Json(camera_response(camera))))
}

// ---------------------------------------------------------------------------
// PATCH /api/v1/cameras/:id
// Allowed: SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn update_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateCameraRequest>,
) -> Result<Json<CameraResponse>, AppError> {
    require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin])?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let camera = service
        .update_camera(&state.postgres_pool, id, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") {
                AppError::NotFound
            } else if msg.contains("already exists")
                || msg.contains("already registered")
                || msg.contains("must be")
                || msg.contains("must start with")
                || msg.contains("between")
                || msg.contains("WIDTHxHEIGHT")
                || msg.contains("exceeds")
                || msg.contains("must be positive")
            {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(camera_response(camera)))
}

// ---------------------------------------------------------------------------
// DELETE /api/v1/cameras/:id
// Allowed: SystemAdmin
// ---------------------------------------------------------------------------
pub async fn delete_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    require_any_role(&roles, &[Role::SystemAdmin])?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    service
        .delete_camera(&state.postgres_pool, id)
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
// POST /api/v1/cameras/:id/enable
// Allowed: Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn enable_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CameraResponse>, AppError> {
    require_any_role(
        &roles,
        &[Role::Operator, Role::SecurityAdmin, Role::SystemAdmin],
    )?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let camera = service
        .enable_camera(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(camera_response(camera)))
}

// ---------------------------------------------------------------------------
// POST /api/v1/cameras/:id/disable
// Allowed: Operator, SecurityAdmin, SystemAdmin
// ---------------------------------------------------------------------------
pub async fn disable_camera(
    AuthUser(_user): AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CameraResponse>, AppError> {
    require_any_role(
        &roles,
        &[Role::Operator, Role::SecurityAdmin, Role::SystemAdmin],
    )?;

    let repo = PostgresCameraRepository;
    let service = CameraService::new(repo);

    let camera = service
        .disable_camera(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(camera_response(camera)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_any_role_allows_match() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        assert!(require_any_role(&roles, &[Role::Viewer, Role::Operator]).is_ok());
    }

    #[test]
    fn test_require_any_role_denies_no_match() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        let result = require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin]);
        assert!(result.is_err());
    }

    #[test]
    fn test_require_any_role_empty_user_roles() {
        let roles = HashSet::new();
        let result = require_any_role(&roles, &[Role::SystemAdmin]);
        assert!(result.is_err());
    }

    #[test]
    fn test_require_any_role_system_admin_allowed_all() {
        let mut roles = HashSet::new();
        roles.insert(Role::SystemAdmin);
        let all_read = [
            Role::Viewer,
            Role::Operator,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ];
        assert!(require_any_role(&roles, &all_read).is_ok());
    }
}
