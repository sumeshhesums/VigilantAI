use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{CreateUser, UpdateUser, User};
use crate::repository::UserRepository;

pub struct PostgresUserRepository;

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, pool: &PgPool, user: &CreateUser) -> anyhow::Result<User> {
        let record = sqlx::query_as::<_, User>(
            "INSERT INTO users (email, password_hash, first_name, last_name) \
             VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn find_by_email(&self, pool: &PgPool, email: &str) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<User>> {
        let records = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(records)
    }

    async fn update(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        user: &UpdateUser,
    ) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as::<_, User>(
            "UPDATE users \
             SET email = COALESCE($2, email), \
                 first_name = COALESCE($3, first_name), \
                 last_name = COALESCE($4, last_name), \
                 updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
