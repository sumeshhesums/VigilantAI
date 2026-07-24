use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

use crate::dto::auth::AuthResponse;
use crate::models::{CreateUser, User};
use crate::repository::UserRepository;
use crate::security::{jwt, password, Security};

/// Authentication service.
///
/// Business logic for registration, login, token refresh, and logout.
pub struct AuthService<'a, R: UserRepository> {
    repository: &'a R,
}

impl<'a, R: UserRepository> AuthService<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    /// Register a new user.
    ///
    /// Checks for duplicate email, hashes the password, and creates the user.
    /// Returns the created user. Does NOT generate tokens.
    pub async fn register(
        &self,
        pool: &PgPool,
        email: String,
        password: String,
        first_name: String,
        last_name: String,
    ) -> Result<User> {
        if self.repository.find_by_email(pool, &email).await?.is_some() {
            return Err(anyhow!("email already registered"));
        }

        let password_hash = password::hash_password(&password)?;

        let user = CreateUser {
            email,
            password_hash,
            first_name,
            last_name,
        };

        self.repository.create(pool, &user).await
    }

    /// Authenticate a user by email and password.
    ///
    /// Returns an `AuthResponse` with access and refresh tokens on success.
    pub async fn login(
        &self,
        pool: &PgPool,
        email: &str,
        password_input: &str,
        security: &Security,
    ) -> Result<AuthResponse> {
        let user = self
            .repository
            .find_by_email(pool, email)
            .await?
            .ok_or_else(|| anyhow!("invalid credentials"))?;

        let valid = password::verify_password(password_input, &user.password_hash)?;
        if !valid {
            return Err(anyhow!("invalid credentials"));
        }

        let role = "user".to_string();
        let access_token = jwt::create_access_token(
            user.id,
            &user.email,
            &role,
            security.access_token_expiry_secs,
            &security.encoding_key,
        )?;

        let refresh_token = jwt::create_refresh_token(
            user.id,
            &user.email,
            &role,
            security.refresh_token_expiry_secs,
            &security.encoding_key,
        )?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            expires_in: security.access_token_expiry_secs,
            token_type: "Bearer".to_string(),
        })
    }

    /// Refresh tokens using a valid refresh token.
    ///
    /// Validates the refresh token, then issues new access and refresh tokens.
    pub async fn refresh(
        &self,
        pool: &PgPool,
        refresh_token: &str,
        security: &Security,
    ) -> Result<AuthResponse> {
        let claims = jwt::validate_token(refresh_token, &security.decoding_key)?;

        let user_id = uuid::Uuid::parse_str(&claims.sub)?;
        let user = self
            .repository
            .find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow!("user not found"))?;

        let role = "user".to_string();
        let access_token = jwt::create_access_token(
            user.id,
            &user.email,
            &role,
            security.access_token_expiry_secs,
            &security.encoding_key,
        )?;

        let refresh_token = jwt::create_refresh_token(
            user.id,
            &user.email,
            &role,
            security.refresh_token_expiry_secs,
            &security.encoding_key,
        )?;

        Ok(AuthResponse {
            access_token,
            refresh_token,
            expires_in: security.access_token_expiry_secs,
            token_type: "Bearer".to_string(),
        })
    }

    /// Logout a user.
    ///
    /// Placeholder — does not blacklist tokens yet.
    pub async fn logout(&self) -> Result<()> {
        Ok(())
    }
}
