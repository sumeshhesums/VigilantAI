use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;

use crate::dto::user::{
    AssignRoleRequest, CreateUserRequest, UpdateUserRequest, UserListResponse,
    UserPaginationParams, UserResponse,
};
use crate::errors::AppError;
use crate::middleware::auth::{AuthUser, UserRoles};
use crate::rbac::guards::require_any_role;
use crate::rbac::roles::Role;
use crate::repository::user_repository::PostgresUserRepository;
use crate::repository::UserRepository;
use crate::services::UserService;
use crate::state::AppState;

fn user_response(user: crate::models::User, roles: Vec<String>) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        is_active: user.is_active,
        roles,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

async fn fetch_user_roles(state: &AppState, user_id: uuid::Uuid) -> Vec<String> {
    let repo = PostgresUserRepository;
    repo.find_roles_by_user_id(&state.postgres_pool, user_id)
        .await
        .unwrap_or_default()
}

fn require_admin_or_security_admin(roles: &UserRoles) -> Result<(), AppError> {
    require_any_role(&roles.0, &[Role::SystemAdmin, Role::SecurityAdmin])
}

fn require_system_admin(roles: &UserRoles) -> Result<(), AppError> {
    require_any_role(&roles.0, &[Role::SystemAdmin])
}

// ---------------------------------------------------------------------------
// GET /api/v1/users
// ---------------------------------------------------------------------------
pub async fn list_users(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Query(params): Query<UserPaginationParams>,
) -> Result<Json<UserListResponse>, AppError> {
    require_admin_or_security_admin(&UserRoles(roles.clone()))?;

    let (offset, limit) = params.offset_limit();
    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    let (users, total) = service
        .list_users(&state.postgres_pool, offset, limit)
        .await
        .map_err(AppError::Internal)?;

    let page = params.page.unwrap_or(1).max(1);

    let mut user_responses = Vec::with_capacity(users.len());
    for u in users {
        let r = fetch_user_roles(&state, u.id).await;
        user_responses.push(user_response(u, r));
    }

    Ok(Json(UserListResponse {
        users: user_responses,
        total,
        page,
        per_page: limit,
        pages: if limit > 0 {
            (total as u32).div_ceil(limit).max(1)
        } else {
            1
        },
    }))
}

// ---------------------------------------------------------------------------
// GET /api/v1/users/:id
// ---------------------------------------------------------------------------
pub async fn get_user(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    require_admin_or_security_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    let target = service
        .get_user(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            if e.to_string().contains("not found") {
                AppError::NotFound
            } else {
                AppError::Internal(e)
            }
        })?;

    let r = fetch_user_roles(&state, id).await;
    Ok(Json(user_response(target, r)))
}

// ---------------------------------------------------------------------------
// POST /api/v1/users
// ---------------------------------------------------------------------------
pub async fn create_user(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    require_system_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    let user = service
        .create_user(&state.postgres_pool, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("already registered")
                || msg.contains("invalid")
                || msg.contains("required")
                || msg.contains("at least")
            {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    let r = fetch_user_roles(&state, user.id).await;
    Ok((StatusCode::CREATED, Json(user_response(user, r))))
}

// ---------------------------------------------------------------------------
// PATCH /api/v1/users/:id
// ---------------------------------------------------------------------------
pub async fn update_user(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    require_system_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    let user = service
        .update_user(&state.postgres_pool, id, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") {
                AppError::NotFound
            } else if msg.contains("already registered") || msg.contains("cannot be empty") {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    let r = fetch_user_roles(&state, id).await;
    Ok(Json(user_response(user, r)))
}

// ---------------------------------------------------------------------------
// DELETE /api/v1/users/:id
// ---------------------------------------------------------------------------
pub async fn delete_user(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, AppError> {
    require_system_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    service
        .deactivate_user(&state.postgres_pool, id)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") {
                AppError::NotFound
            } else if msg.contains("already deactivated") {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// POST /api/v1/users/:id/roles
// ---------------------------------------------------------------------------
pub async fn assign_role(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<AssignRoleRequest>,
) -> Result<StatusCode, AppError> {
    require_system_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    service
        .assign_role(&state.postgres_pool, id, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") {
                AppError::NotFound
            } else if msg.contains("unknown role") {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// DELETE /api/v1/users/:id/roles
// ---------------------------------------------------------------------------
pub async fn remove_role(
    AuthUser { .. }: AuthUser,
    UserRoles(roles): UserRoles,
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(body): Json<AssignRoleRequest>,
) -> Result<StatusCode, AppError> {
    require_system_admin(&UserRoles(roles))?;

    let repo = PostgresUserRepository;
    let service = UserService::new(repo);

    service
        .remove_role(&state.postgres_pool, id, &body)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("not found") {
                AppError::NotFound
            } else if msg.contains("does not have role") {
                AppError::Conflict(msg)
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}
