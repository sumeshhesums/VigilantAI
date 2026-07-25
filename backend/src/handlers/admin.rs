use axum::Json;
use serde::Serialize;

use crate::middleware::auth::AuthUser;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AdminHealthResponse {
    status: &'static str,
    user_email: String,
    user_roles: Vec<String>,
}

pub async fn admin_health(
    AuthUser { user, .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<AdminHealthResponse>, crate::errors::AppError> {
    let role_names: Vec<String> = sqlx::query_scalar(
        "SELECT r.name FROM user_roles ur JOIN roles r ON ur.role_id = r.id WHERE ur.user_id = $1",
    )
    .bind(user.id)
    .fetch_all(&state.postgres_pool)
    .await
    .map_err(|e| crate::errors::AppError::Internal(anyhow::anyhow!(e)))?;

    Ok(Json(AdminHealthResponse {
        status: "ok",
        user_email: user.email,
        user_roles: role_names,
    }))
}
