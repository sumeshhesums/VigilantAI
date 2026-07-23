use sqlx::postgres::PgPool;

use crate::models::{CreateUser, UpdateUser, User};
use crate::repository::UserRepository;

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create_user(&self, pool: &PgPool, user: &CreateUser) -> anyhow::Result<User> {
        self.repository.create(pool, user).await
    }

    pub async fn get_user(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>> {
        self.repository.find_by_id(pool, id).await
    }

    pub async fn get_user_by_email(
        &self,
        pool: &PgPool,
        email: &str,
    ) -> anyhow::Result<Option<User>> {
        self.repository.find_by_email(pool, email).await
    }

    pub async fn list_users(&self, pool: &PgPool) -> anyhow::Result<Vec<User>> {
        self.repository.list(pool).await
    }

    pub async fn update_user(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        user: &UpdateUser,
    ) -> anyhow::Result<Option<User>> {
        self.repository.update(pool, id, user).await
    }

    pub async fn delete_user(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool> {
        self.repository.delete(pool, id).await
    }
}
