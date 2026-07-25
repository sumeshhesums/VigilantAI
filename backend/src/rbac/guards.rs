use std::collections::HashSet;

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::errors::AppError;
use crate::state::AppState;

use super::permissions::Permission;
use super::roles::Role;

/// Check that the user holds at least one of the required roles.
/// Returns `Err(AppError::Forbidden)` on failure.
pub fn require_any_role(user_roles: &HashSet<Role>, allowed: &[Role]) -> Result<(), AppError> {
    for role in allowed {
        if user_roles.contains(role) {
            return Ok(());
        }
    }
    Err(AppError::Forbidden(format!(
        "one of the following roles is required: {}",
        allowed
            .iter()
            .map(|r| r.as_db_str())
            .collect::<Vec<_>>()
            .join(", ")
    )))
}

/// Extract the user's roles from request extensions.
pub fn user_roles(parts: &Parts) -> HashSet<Role> {
    parts
        .extensions
        .get::<HashSet<Role>>()
        .cloned()
        .unwrap_or_default()
}

/// Extract the user's permissions from request extensions.
pub fn user_permissions(parts: &Parts) -> HashSet<Permission> {
    parts
        .extensions
        .get::<HashSet<Permission>>()
        .cloned()
        .unwrap_or_default()
}

/// Check whether the user has a specific role.
pub fn has_role(parts: &Parts, role: Role) -> bool {
    user_roles(parts).contains(&role)
}

/// Check whether the user has a specific permission.
pub fn has_permission(parts: &Parts, permission: Permission) -> bool {
    user_permissions(parts).contains(&permission)
}

// ---------------------------------------------------------------------------
// Axum extractors
// ---------------------------------------------------------------------------

/// Marker inserted into request extensions by middleware to declare the
/// required role for the `RequireRole` extractor.
#[derive(Clone, Copy)]
pub struct RequiredRoleMarker(pub Role);

/// Axum extractor that rejects the request with 403 Forbidden
/// unless the authenticated user holds the required role.
pub struct RequireRole(pub Role);

#[async_trait]
impl FromRequestParts<AppState> for RequireRole {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let role = parts
            .extensions
            .get::<RequiredRoleMarker>()
            .map(|m| m.0)
            .ok_or_else(|| {
                AppError::Internal(anyhow::anyhow!(
                    "RequireRole used without RequiredRoleMarker in extensions"
                ))
            })?;

        if has_role(parts, role) {
            Ok(RequireRole(role))
        } else {
            Err(AppError::Forbidden(format!("required role: {role}")))
        }
    }
}

/// Marker inserted into request extensions by middleware to declare the
/// required permission for the `RequirePermission` extractor.
#[derive(Clone, Copy)]
pub struct RequiredPermissionMarker(pub Permission);

/// Axum extractor that rejects the request with 403 Forbidden
/// unless the authenticated user holds the required permission.
pub struct RequirePermission(pub Permission);

#[async_trait]
impl FromRequestParts<AppState> for RequirePermission {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let permission = parts
            .extensions
            .get::<RequiredPermissionMarker>()
            .map(|m| m.0)
            .ok_or_else(|| {
                AppError::Internal(anyhow::anyhow!(
                    "RequirePermission used without RequiredPermissionMarker in extensions"
                ))
            })?;

        if has_permission(parts, permission) {
            Ok(RequirePermission(permission))
        } else {
            Err(AppError::Forbidden(format!(
                "required permission: {permission}"
            )))
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_parts_with_roles(roles: HashSet<Role>, perms: HashSet<Permission>) -> Parts {
        let (mut parts, _) = axum::http::Request::builder()
            .body(())
            .unwrap()
            .into_parts();
        parts.extensions.insert(roles);
        parts.extensions.insert(perms);
        parts
    }

    #[test]
    fn test_has_role_present() {
        let mut roles = HashSet::new();
        roles.insert(Role::SystemAdmin);
        let parts = make_parts_with_roles(roles, HashSet::new());
        assert!(has_role(&parts, Role::SystemAdmin));
    }

    #[test]
    fn test_has_role_absent() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        let parts = make_parts_with_roles(roles, HashSet::new());
        assert!(!has_role(&parts, Role::SystemAdmin));
    }

    #[test]
    fn test_has_permission_present() {
        let mut perms = HashSet::new();
        perms.insert(Permission::CameraView);
        let parts = make_parts_with_roles(HashSet::new(), perms);
        assert!(has_permission(&parts, Permission::CameraView));
    }

    #[test]
    fn test_has_permission_absent() {
        let parts = make_parts_with_roles(HashSet::new(), HashSet::new());
        assert!(!has_permission(&parts, Permission::SystemAdmin));
    }

    #[test]
    fn test_empty_extensions_default() {
        let (parts, _) = axum::http::Request::builder()
            .body(())
            .unwrap()
            .into_parts();
        assert!(user_roles(&parts).is_empty());
        assert!(user_permissions(&parts).is_empty());
    }

    #[test]
    fn test_multiple_roles() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        roles.insert(Role::Operator);
        let parts = make_parts_with_roles(roles, HashSet::new());
        assert!(has_role(&parts, Role::Viewer));
        assert!(has_role(&parts, Role::Operator));
        assert!(!has_role(&parts, Role::SystemAdmin));
    }

    // -----------------------------------------------------------------------
    // require_any_role tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_require_any_role_allows_match() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        assert!(require_any_role(&roles, &[Role::Viewer, Role::Operator]).is_ok());
    }

    #[test]
    fn test_require_any_role_denies_no_match() {
        let mut roles = HashSet::new();
        roles.insert(Role::Viewer);
        let result = require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin]);
        assert!(result.is_err());
    }

    #[test]
    fn test_require_any_role_empty_user_roles() {
        let roles = HashSet::new();
        let result = require_any_role(&roles, &[Role::SystemAdmin]);
        assert!(result.is_err());
    }

    #[test]
    fn test_require_any_role_system_admin_allowed_all() {
        let mut roles = HashSet::new();
        roles.insert(Role::SystemAdmin);
        let all_roles = [
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ];
        assert!(require_any_role(&roles, &all_roles).is_ok());
    }

    #[test]
    fn test_require_any_role_operator_can_view() {
        let mut roles = HashSet::new();
        roles.insert(Role::Operator);
        let view_roles = [
            Role::Viewer,
            Role::Operator,
            Role::SecurityAnalyst,
            Role::SecurityAdmin,
            Role::SystemAdmin,
        ];
        assert!(require_any_role(&roles, &view_roles).is_ok());
    }

    #[test]
    fn test_require_any_role_operator_cannot_create() {
        let mut roles = HashSet::new();
        roles.insert(Role::Operator);
        let result = require_any_role(&roles, &[Role::SecurityAdmin, Role::SystemAdmin]);
        assert!(result.is_err());
    }
}
