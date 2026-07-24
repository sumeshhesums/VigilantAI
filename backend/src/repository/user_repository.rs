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

    async fn list_paginated(
        &self,
        pool: &PgPool,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<User>> {
        let records = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC OFFSET $1 LIMIT $2",
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    async fn count(&self, pool: &PgPool) -> anyhow::Result<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
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

    async fn soft_delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>> {
        let record = sqlx::query_as::<_, User>(
            "UPDATE users SET is_active = FALSE, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn assign_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        role_name: &str,
    ) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO user_roles (user_id, role_id) \
             SELECT $1, r.id FROM roles r WHERE r.name = $2 \
             ON CONFLICT (user_id, role_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(role_name)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn remove_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        role_name: &str,
    ) -> anyhow::Result<bool> {
        let result = sqlx::query(
            "DELETE FROM user_roles \
             WHERE user_id = $1 AND role_id = (SELECT id FROM roles WHERE name = $2)",
        )
        .bind(user_id)
        .bind(role_name)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn find_roles_by_user_id(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> anyhow::Result<Vec<String>> {
        let roles = sqlx::query_scalar::<_, String>(
            "SELECT r.name FROM user_roles ur JOIN roles r ON ur.role_id = r.id \
             WHERE ur.user_id = $1 ORDER BY r.name",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(roles)
    }
}
