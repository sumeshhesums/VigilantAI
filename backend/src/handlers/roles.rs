use axum::extract::State;
use axum::Json;

use crate::dto::role::{RoleListResponse, RoleResponse};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::repository::role_repository::PostgresRoleRepository;
use crate::repository::RoleRepository;
use crate::state::AppState;

// ---------------------------------------------------------------------------
// GET /api/v1/roles
// Allowed: SystemAdmin, SecurityAdmin
// ---------------------------------------------------------------------------
pub async fn list_roles(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
) -> Result<Json<RoleListResponse>, AppError> {
    require_any_role(&roles, &[Role::SystemAdmin, Role::SecurityAdmin])?;

    let repo = PostgresRoleRepository;
    let db_roles = repo
        .list(&state.postgres_pool)
        .await
        .map_err(AppError::Internal)?;

    let role_responses: Vec<RoleResponse> = db_roles
        .into_iter()
        .map(|r| RoleResponse {
            id: r.id,
            name: r.name,
            description: r.description,
        })
        .collect();

    Ok(Json(RoleListResponse {
        roles: role_responses,
    }))
}
