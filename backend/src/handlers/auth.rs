use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::dto::auth::{
    AuthResponse, LoginRequest, RefreshTokenRequest, RegisterRequest, UserResponse,
};
use crate::errors::AppError;
use crate::middleware::auth::AuthUser;
use crate::repository::user_repository::PostgresUserRepository;
use crate::services::AuthService;
use crate::state::AppState;

fn user_response(user: crate::models::User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: "user".to_string(),
        created_at: user.created_at,
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let repo = PostgresUserRepository;
    let service = AuthService::new(&repo);

    let user = service
        .register(
            &state.postgres_pool,
            body.email,
            body.password,
            body.first_name,
            body.last_name,
        )
        .await
        .map_err(|e| {
            if e.to_string().contains("already registered") {
                AppError::Conflict(e.to_string())
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok((StatusCode::CREATED, Json(user_response(user))))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let repo = PostgresUserRepository;
    let service = AuthService::new(&repo);

    let response = service
        .login(
            &state.postgres_pool,
            &body.email,
            &body.password,
            &state.security,
        )
        .await
        .map_err(|e| {
            if e.to_string().contains("invalid credentials") {
                AppError::Unauthorized(e.to_string())
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(response))
}

pub async fn refresh(
    State(state): State<AppState>,
    Json(body): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let repo = PostgresUserRepository;
    let service = AuthService::new(&repo);

    let response = service
        .refresh(&state.postgres_pool, &body.refresh_token, &state.security)
        .await
        .map_err(|e| {
            if e.to_string().contains("invalid") || e.to_string().contains("expired") {
                AppError::InvalidToken(e.to_string())
            } else {
                AppError::Internal(e)
            }
        })?;

    Ok(Json(response))
}

pub async fn logout(AuthUser(_user): AuthUser) -> Result<StatusCode, AppError> {
    let repo = PostgresUserRepository;
    let service = AuthService::new(&repo);

    service.logout().await.map_err(AppError::Internal)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(AuthUser(user): AuthUser) -> Json<UserResponse> {
    Json(user_response(user))
}
