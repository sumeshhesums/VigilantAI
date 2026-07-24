use async_trait::async_trait;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::errors::AppError;
use crate::models::User;
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
            Some(user) => Ok(AuthUser(user)),
            None => Err(AppError::Unauthorized("user not found".into())),
        }
    }
}
