use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{CreatePermission, Permission};
use crate::repository::PermissionRepository;

pub struct PostgresPermissionRepository;

#[async_trait]
impl PermissionRepository for PostgresPermissionRepository {
    async fn create(
        &self,
        pool: &PgPool,
        permission: &CreatePermission,
    ) -> anyhow::Result<Permission> {
        let record = sqlx::query_as::<_, Permission>(
            "INSERT INTO permissions (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&permission.name)
        .bind(&permission.description)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Permission>> {
        let record = sqlx::query_as::<_, Permission>("SELECT * FROM permissions WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Permission>> {
        let records = sqlx::query_as::<_, Permission>("SELECT * FROM permissions ORDER BY name")
            .fetch_all(pool)
            .await?;

        Ok(records)
    }
}
