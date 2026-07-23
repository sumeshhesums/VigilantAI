use sqlx::postgres::PgPool;

use crate::models::User;
use crate::repository::UserRepository;

/// Authentication service scaffold.
///
/// JWT issuance and validation will be implemented here.
pub struct AuthService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> AuthService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Register a new user. Returns the created user.
    pub async fn register(
        &self,
        pool: &PgPool,
        email: String,
        password_hash: String,
        first_name: String,
        last_name: String,
    ) -> anyhow::Result<User> {
        let user = crate::models::CreateUser {
            email,
            password_hash,
            first_name,
            last_name,
        };
        self.repository.create(pool, &user).await
    }

    /// Look up a user by email for authentication.
    pub async fn find_user_by_email(
        &self,
        pool: &PgPool,
        email: &str,
    ) -> anyhow::Result<Option<User>> {
        self.repository.find_by_email(pool, email).await
    }
}
