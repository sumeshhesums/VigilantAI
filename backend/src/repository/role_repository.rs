use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{CreateRole, Role};
use crate::repository::RoleRepository;

pub struct PostgresRoleRepository;

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn create(&self, pool: &PgPool, role: &CreateRole) -> anyhow::Result<Role> {
        let record = sqlx::query_as::<_, Role>(
            "INSERT INTO roles (name, description) VALUES ($1, $2) RETURNING *",
        )
        .bind(&role.name)
        .bind(&role.description)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Role>> {
        let record = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Role>> {
        let records = sqlx::query_as::<_, Role>("SELECT * FROM roles ORDER BY name")
            .fetch_all(pool)
            .await?;

        Ok(records)
    }
}
