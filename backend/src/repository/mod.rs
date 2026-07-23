pub mod permission_repository;
pub mod role_repository;
pub mod user_repository;

use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{CreatePermission, CreateRole, CreateUser, Permission, Role, UpdateUser, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, user: &CreateUser) -> anyhow::Result<User>;
    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, pool: &PgPool, email: &str) -> anyhow::Result<Option<User>>;
    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<User>>;
    async fn update(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        user: &UpdateUser,
    ) -> anyhow::Result<Option<User>>;
    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool>;
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, role: &CreateRole) -> anyhow::Result<Role>;
    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Role>>;
    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Role>>;
}

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(
        &self,
        pool: &PgPool,
        permission: &CreatePermission,
    ) -> anyhow::Result<Permission>;
    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Permission>>;
    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Permission>>;
}
