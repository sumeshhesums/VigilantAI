pub mod camera_repository;
pub mod incident_repository;
pub mod permission_repository;
pub mod role_repository;
pub mod user_repository;

use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{
    Camera, CreateCamera, CreatePermission, CreateRole, CreateUser, Permission, Role, UpdateCamera,
    UpdateUser, User,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, user: &CreateUser) -> anyhow::Result<User>;
    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>>;
    async fn find_by_email(&self, pool: &PgPool, email: &str) -> anyhow::Result<Option<User>>;
    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<User>>;
    async fn list_paginated(
        &self,
        pool: &PgPool,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<User>>;
    async fn count(&self, pool: &PgPool) -> anyhow::Result<i64>;
    async fn update(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        user: &UpdateUser,
    ) -> anyhow::Result<Option<User>>;
    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool>;
    async fn soft_delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<User>>;
    async fn assign_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        role_name: &str,
    ) -> anyhow::Result<()>;
    async fn remove_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        role_name: &str,
    ) -> anyhow::Result<bool>;
    async fn find_roles_by_user_id(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
    ) -> anyhow::Result<Vec<String>>;
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

#[async_trait]
pub trait CameraRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, camera: &CreateCamera) -> anyhow::Result<Camera>;
    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>>;
    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Camera>>;
    async fn find_by_rtsp_url(
        &self,
        pool: &PgPool,
        rtsp_url: &str,
    ) -> anyhow::Result<Option<Camera>>;
    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Camera>>;
    async fn update(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        camera: &UpdateCamera,
    ) -> anyhow::Result<Option<Camera>>;
    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool>;
    async fn enable(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>>;
    async fn disable(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>>;
    async fn update_last_seen(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> anyhow::Result<Option<Camera>>;
    async fn count(&self, pool: &PgPool) -> anyhow::Result<i64>;
}
