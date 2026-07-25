use std::collections::HashSet;

use async_trait::async_trait;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::cache;
use crate::errors::AppError;
use crate::models::User;
use crate::rbac::permissions::{self, Permission};
use crate::rbac::roles::Role;
use crate::repository::user_repository::PostgresUserRepository;
use crate::repository::UserRepository;
use crate::security::jwt;
use crate::state::AppState;

/// Authenticated user extracted from a valid JWT.
///
/// Carries the user record and the raw token string so downstream
/// handlers (e.g. logout) can access the original token.
pub struct AuthUser {
    pub user: User,
    pub token: String,
}

impl AuthUser {
    pub fn into_inner(self) -> User {
        self.user
    }
}

/// The user's RBAC roles, inserted into request extensions by `AuthUser`.
#[derive(Clone, Debug)]
pub struct UserRoles(pub HashSet<Role>);

#[async_trait]
impl axum::extract::FromRequestParts<AppState> for UserRoles {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UserRoles>()
            .cloned()
            .ok_or_else(|| AppError::Unauthorized("roles not available — did AuthUser run?".into()))
    }
}

/// Load the user's roles from the database and insert both role sets and a
/// typed `UserRoles` into the *original* request extensions.
async fn attach_roles_and_permissions(
    parts: &mut Parts,
    state: &AppState,
    user_id: uuid::Uuid,
) -> Result<(), AppError> {
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

    // Insert into the *original* parts so downstream extractors (e.g. UserRoles)
    // can read them.
    parts.extensions.insert(roles.clone());
    parts.extensions.insert(perms);
    parts.extensions.insert(UserRoles(roles));

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

        let raw_token = bearer.token().to_string();

        let claims = jwt::validate_token(bearer.token(), &state.security.decoding_key)
            .map_err(|e| AppError::InvalidToken(e.to_string()))?;

        // Check if this token has been blacklisted (logged out)
        if cache::is_token_blacklisted(&state.redis_client, &claims.jti)
            .await
            .unwrap_or(false)
        {
            return Err(AppError::InvalidToken("token has been revoked".into()));
        }

        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::InvalidToken(format!("invalid subject: {e}")))?;

        let repo = PostgresUserRepository;
        let user = repo
            .find_by_id(&state.postgres_pool, user_id)
            .await
            .map_err(AppError::Internal)?;

        match user {
            Some(user) => {
                let _ = attach_roles_and_permissions(parts, state, user.id).await;
                Ok(AuthUser {
                    user,
                    token: raw_token,
                })
            }
            None => Err(AppError::Unauthorized("user not found".into())),
        }
    }
}
