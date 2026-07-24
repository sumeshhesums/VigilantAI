use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

use crate::dto::user::{AssignRoleRequest, CreateUserRequest, UpdateUserRequest};
use crate::models::{CreateUser, UpdateUser};
use crate::repository::UserRepository;
use crate::security::password;

pub struct UserService<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// List users with pagination. Returns (users, total_count).
    pub async fn list_users(
        &self,
        pool: &PgPool,
        offset: u32,
        limit: u32,
    ) -> Result<(Vec<crate::models::User>, i64)> {
        let users = self
            .repository
            .list_paginated(pool, offset as i64, limit as i64)
            .await?;
        let total = self.repository.count(pool).await?;
        Ok((users, total))
    }

    /// Get a user by ID.
    pub async fn get_user(&self, pool: &PgPool, id: uuid::Uuid) -> Result<crate::models::User> {
        self.repository
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("user not found"))
    }

    /// Create a new user. Validates email uniqueness, password length, and role existence.
    pub async fn create_user(
        &self,
        pool: &PgPool,
        req: &CreateUserRequest,
    ) -> Result<crate::models::User> {
        // Validate email format
        if !req.email.contains('@') || req.email.len() < 5 {
            return Err(anyhow!("invalid email format"));
        }

        // Validate password length
        if req.password.len() < 8 {
            return Err(anyhow!("password must be at least 8 characters"));
        }

        // Validate first/last name
        if req.first_name.trim().is_empty() {
            return Err(anyhow!("first name is required"));
        }
        if req.last_name.trim().is_empty() {
            return Err(anyhow!("last name is required"));
        }

        // Check duplicate email
        if self
            .repository
            .find_by_email(pool, &req.email)
            .await?
            .is_some()
        {
            return Err(anyhow!("email already registered"));
        }

        // Validate all requested roles exist
        for role_name in &req.roles {
            if crate::rbac::roles::Role::from_db_str(role_name).is_none() {
                return Err(anyhow!("unknown role: {role_name}"));
            }
        }

        let password_hash = password::hash_password(&req.password)?;

        let create = CreateUser {
            email: req.email.clone(),
            password_hash,
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
        };

        let user = self.repository.create(pool, &create).await?;

        // Assign roles
        for role_name in &req.roles {
            let role = crate::rbac::roles::Role::from_db_str(role_name).unwrap();
            self.repository
                .assign_role(pool, user.id, role.as_db_str())
                .await?;
        }

        Ok(user)
    }

    /// Update a user's profile fields.
    pub async fn update_user(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        req: &UpdateUserRequest,
    ) -> Result<crate::models::User> {
        // Validate email if being changed
        if let Some(ref email) = req.email {
            if !email.contains('@') || email.len() < 5 {
                return Err(anyhow!("invalid email format"));
            }
            // Check uniqueness
            if let Some(existing) = self.repository.find_by_email(pool, email).await? {
                if existing.id != id {
                    return Err(anyhow!("email already registered"));
                }
            }
        }

        if let Some(ref name) = req.first_name {
            if name.trim().is_empty() {
                return Err(anyhow!("first name cannot be empty"));
            }
        }
        if let Some(ref name) = req.last_name {
            if name.trim().is_empty() {
                return Err(anyhow!("last name cannot be empty"));
            }
        }

        let update = UpdateUser {
            email: req.email.clone(),
            first_name: req.first_name.clone(),
            last_name: req.last_name.clone(),
        };

        self.repository
            .update(pool, id, &update)
            .await?
            .ok_or_else(|| anyhow!("user not found"))
    }

    /// Soft-delete (deactivate) a user.
    pub async fn deactivate_user(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> Result<crate::models::User> {
        let user = self.get_user(pool, id).await?;
        if !user.is_active {
            return Err(anyhow!("user is already deactivated"));
        }
        self.repository
            .soft_delete(pool, id)
            .await?
            .ok_or_else(|| anyhow!("user not found"))
    }

    /// Assign a role to a user.
    pub async fn assign_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        req: &AssignRoleRequest,
    ) -> Result<()> {
        // Validate user exists
        self.get_user(pool, user_id).await?;

        // Validate role name
        let role = crate::rbac::roles::Role::from_db_str(&req.role)
            .ok_or_else(|| anyhow!("unknown role: {}", req.role))?;

        self.repository
            .assign_role(pool, user_id, role.as_db_str())
            .await?;

        Ok(())
    }

    /// Remove a role from a user.
    pub async fn remove_role(
        &self,
        pool: &PgPool,
        user_id: uuid::Uuid,
        req: &AssignRoleRequest,
    ) -> Result<()> {
        // Validate user exists
        self.get_user(pool, user_id).await?;

        let role = crate::rbac::roles::Role::from_db_str(&req.role)
            .ok_or_else(|| anyhow!("unknown role: {}", req.role))?;

        let removed = self
            .repository
            .remove_role(pool, user_id, role.as_db_str())
            .await?;

        if !removed {
            return Err(anyhow!("user does not have role: {}", req.role));
        }

        Ok(())
    }
}
