use std::collections::HashSet;

use async_trait::async_trait;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::errors::AppError;
use crate::models::User;
use crate::rbac::permissions::{self, Permission};
use crate::rbac::roles::Role;
use crate::repository::user_repository::PostgresUserRepository;
use crate::repository::UserRepository;
use crate::security::jwt;
use crate::state::AppState;

/// Authenticated user extracted from a valid JWT.
pub struct AuthUser(pub User);

impl AuthUser {
    pub fn into_inner(self) -> User {
        self.0
    }
}

/// Load the user's roles from the database and insert both roles and
/// computed permissions into request extensions for downstream RBAC extractors.
async fn attach_roles_and_permissions(
    parts: &mut Parts,
    state: &AppState,
    user_id: uuid::Uuid,
) -> Result<(), AppError> {
    // Query role names via the user_roles + roles join
    let role_names: Vec<String> = sqlx::query_scalar(
        "SELECT r.name FROM user_roles ur JOIN roles r ON ur.role_id = r.id WHERE ur.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.postgres_pool)
    .await
    .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    let roles: HashSet<Role> = role_names
        .iter()
        .filter_map(|name| name.parse::<Role>().ok())
        .collect();

    let perms: HashSet<Permission> =
        permissions::permissions_for_roles(&roles.iter().copied().collect::<Vec<_>>());

    parts.extensions.insert(roles);
    parts.extensions.insert(perms);

    Ok(())
}

#[async_trait]
impl axum::extract::FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                AppError::Unauthorized("missing or malformed Authorization header".into())
            })?;

        let claims = jwt::validate_token(bearer.token(), &state.security.decoding_key)
            .map_err(|e| AppError::InvalidToken(e.to_string()))?;

        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::InvalidToken(format!("invalid subject: {e}")))?;

        let repo = PostgresUserRepository;
        let user = repo
            .find_by_id(&state.postgres_pool, user_id)
            .await
            .map_err(AppError::Internal)?;

        match user {
            Some(user) => {
                // Attach roles + permissions for downstream RBAC guards.
                // Errors here are non-fatal: the user is still authenticated
                // even if role lookup fails (they just get empty roles).
                let _ = attach_roles_and_permissions(parts, state, user.id).await;
                Ok(AuthUser(user))
            }
            None => Err(AppError::Unauthorized("user not found".into())),
        }
    }
}
