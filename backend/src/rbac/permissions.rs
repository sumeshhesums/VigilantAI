use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::roles::Role;

/// Granular permissions for resource access control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    // Users
    UserView,
    UserCreate,
    UserUpdate,
    UserDelete,

    // Roles
    RoleView,
    RoleUpdate,

    // Camera
    CameraView,
    CameraCreate,
    CameraUpdate,
    CameraDelete,

    // Incident
    IncidentView,
    IncidentCreate,
    IncidentUpdate,
    IncidentClose,

    // Evidence
    EvidenceView,
    EvidenceDownload,
    EvidenceUpload,
    EvidenceDelete,

    // Dashboard
    DashboardView,

    // System
    SystemAdmin,
}

impl Permission {
    /// All variants in display order.
    pub const ALL: &'static [Permission] = &[
        Permission::UserView,
        Permission::UserCreate,
        Permission::UserUpdate,
        Permission::UserDelete,
        Permission::RoleView,
        Permission::RoleUpdate,
        Permission::CameraView,
        Permission::CameraCreate,
        Permission::CameraUpdate,
        Permission::CameraDelete,
        Permission::IncidentView,
        Permission::IncidentCreate,
        Permission::IncidentUpdate,
        Permission::IncidentClose,
        Permission::EvidenceView,
        Permission::EvidenceDownload,
        Permission::EvidenceUpload,
        Permission::EvidenceDelete,
        Permission::DashboardView,
        Permission::SystemAdmin,
    ];

    /// Canonical `resource:action` string used in the database.
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::UserView => "user:view",
            Permission::UserCreate => "user:create",
            Permission::UserUpdate => "user:update",
            Permission::UserDelete => "user:delete",
            Permission::RoleView => "role:view",
            Permission::RoleUpdate => "role:update",
            Permission::CameraView => "camera:view",
            Permission::CameraCreate => "camera:create",
            Permission::CameraUpdate => "camera:update",
            Permission::CameraDelete => "camera:delete",
            Permission::IncidentView => "incident:view",
            Permission::IncidentCreate => "incident:create",
            Permission::IncidentUpdate => "incident:update",
            Permission::IncidentClose => "incident:close",
            Permission::EvidenceView => "evidence:view",
            Permission::EvidenceDownload => "evidence:download",
            Permission::EvidenceUpload => "evidence:upload",
            Permission::EvidenceDelete => "evidence:delete",
            Permission::DashboardView => "dashboard:view",
            Permission::SystemAdmin => "system:admin",
        }
    }

    /// Parse from a `resource:action` string.
    pub fn from_str_ref(s: &str) -> Option<Self> {
        match s {
            "user:view" => Some(Permission::UserView),
            "user:create" => Some(Permission::UserCreate),
            "user:update" => Some(Permission::UserUpdate),
            "user:delete" => Some(Permission::UserDelete),
            "role:view" => Some(Permission::RoleView),
            "role:update" => Some(Permission::RoleUpdate),
            "camera:view" => Some(Permission::CameraView),
            "camera:create" => Some(Permission::CameraCreate),
            "camera:update" => Some(Permission::CameraUpdate),
            "camera:delete" => Some(Permission::CameraDelete),
            "incident:view" => Some(Permission::IncidentView),
            "incident:create" => Some(Permission::IncidentCreate),
            "incident:update" => Some(Permission::IncidentUpdate),
            "incident:close" => Some(Permission::IncidentClose),
            "evidence:view" => Some(Permission::EvidenceView),
            "evidence:download" => Some(Permission::EvidenceDownload),
            "evidence:upload" => Some(Permission::EvidenceUpload),
            "evidence:delete" => Some(Permission::EvidenceDelete),
            "dashboard:view" => Some(Permission::DashboardView),
            "system:admin" => Some(Permission::SystemAdmin),
            _ => None,
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_ref(s).ok_or_else(|| format!("unknown permission: {s}"))
    }
}

impl Serialize for Permission {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Permission {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_str_ref(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown permission: {s}")))
    }
}

/// Returns the default set of permissions for a given role.
///
/// Higher-privilege roles include all permissions of lower-privilege roles.
pub fn default_permissions_for_role(role: Role) -> HashSet<Permission> {
    match role {
        Role::SystemAdmin => Permission::ALL.iter().copied().collect(),

        Role::SecurityAdmin => {
            let mut perms = default_permissions_for_role(Role::SecurityAnalyst);
            perms.extend([
                Permission::UserView,
                Permission::UserCreate,
                Permission::UserUpdate,
                Permission::UserDelete,
                Permission::RoleView,
                Permission::RoleUpdate,
                Permission::EvidenceDelete,
            ]);
            perms
        }

        Role::SecurityAnalyst => {
            let mut perms = default_permissions_for_role(Role::Operator);
            perms.extend([
                Permission::IncidentCreate,
                Permission::IncidentUpdate,
                Permission::IncidentClose,
                Permission::EvidenceView,
                Permission::EvidenceDownload,
            ]);
            perms
        }

        Role::Operator => {
            let mut perms = default_permissions_for_role(Role::Viewer);
            perms.extend([
                Permission::IncidentUpdate,
                Permission::CameraUpdate,
                Permission::EvidenceUpload,
            ]);
            perms
        }

        Role::Viewer => HashSet::from([
            Permission::DashboardView,
            Permission::CameraView,
            Permission::IncidentView,
            Permission::EvidenceView,
        ]),

        Role::ApiIntegration => HashSet::from([
            Permission::CameraView,
            Permission::CameraCreate,
            Permission::IncidentView,
            Permission::IncidentCreate,
            Permission::EvidenceView,
            Permission::EvidenceUpload,
        ]),
    }
}

/// Collect all permissions a user has across multiple roles.
pub fn permissions_for_roles(roles: &[Role]) -> HashSet<Permission> {
    roles
        .iter()
        .flat_map(|r| default_permissions_for_role(*r))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permission_display_roundtrip() {
        for perm in Permission::ALL {
            let s = perm.to_string();
            let parsed: Permission = s.parse().unwrap();
            assert_eq!(*perm, parsed);
        }
    }

    #[test]
    fn test_permission_from_str_invalid() {
        assert!("unknown:perm".parse::<Permission>().is_err());
    }

    #[test]
    fn test_permission_serialization() {
        let perm = Permission::UserView;
        let json = serde_json::to_string(&perm).unwrap();
        assert_eq!(json, "\"user:view\"");

        let deserialized: Permission = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Permission::UserView);
    }

    #[test]
    fn test_system_admin_has_all_permissions() {
        let perms = default_permissions_for_role(Role::SystemAdmin);
        for perm in Permission::ALL {
            assert!(perms.contains(perm), "SystemAdmin missing {perm}");
        }
    }

    #[test]
    fn test_viewer_has_minimal_permissions() {
        let perms = default_permissions_for_role(Role::Viewer);
        assert_eq!(perms.len(), 4);
        assert!(perms.contains(&Permission::DashboardView));
        assert!(perms.contains(&Permission::CameraView));
        assert!(perms.contains(&Permission::IncidentView));
        assert!(perms.contains(&Permission::EvidenceView));
        // Viewer should NOT have write permissions
        assert!(!perms.contains(&Permission::IncidentUpdate));
        assert!(!perms.contains(&Permission::CameraCreate));
        assert!(!perms.contains(&Permission::UserDelete));
    }

    #[test]
    fn test_operator_extends_viewer() {
        let viewer = default_permissions_for_role(Role::Viewer);
        let operator = default_permissions_for_role(Role::Operator);
        assert!(operator.is_superset(&viewer));
        assert!(operator.contains(&Permission::IncidentUpdate));
        assert!(operator.contains(&Permission::CameraUpdate));
        assert!(operator.contains(&Permission::EvidenceUpload));
    }

    #[test]
    fn test_security_analyst_extends_operator() {
        let operator = default_permissions_for_role(Role::Operator);
        let analyst = default_permissions_for_role(Role::SecurityAnalyst);
        assert!(analyst.is_superset(&operator));
        assert!(analyst.contains(&Permission::IncidentCreate));
        assert!(analyst.contains(&Permission::IncidentClose));
        assert!(analyst.contains(&Permission::EvidenceDownload));
    }

    #[test]
    fn test_security_admin_extends_analyst() {
        let analyst = default_permissions_for_role(Role::SecurityAnalyst);
        let sec_admin = default_permissions_for_role(Role::SecurityAdmin);
        assert!(sec_admin.is_superset(&analyst));
        assert!(sec_admin.contains(&Permission::UserCreate));
        assert!(sec_admin.contains(&Permission::RoleUpdate));
        assert!(sec_admin.contains(&Permission::EvidenceDelete));
    }

    #[test]
    fn test_api_integration_independent() {
        let api = default_permissions_for_role(Role::ApiIntegration);
        assert_eq!(api.len(), 6);
        assert!(api.contains(&Permission::CameraView));
        assert!(api.contains(&Permission::CameraCreate));
        assert!(api.contains(&Permission::IncidentView));
        assert!(api.contains(&Permission::IncidentCreate));
        assert!(api.contains(&Permission::EvidenceView));
        assert!(api.contains(&Permission::EvidenceUpload));
    }

    #[test]
    fn test_permissions_for_roles_union() {
        let roles = vec![Role::Viewer, Role::ApiIntegration];
        let perms = permissions_for_roles(&roles);
        // Should be union of both
        assert!(perms.contains(&Permission::DashboardView)); // from Viewer
        assert!(perms.contains(&Permission::CameraCreate)); // from ApiIntegration
        assert!(perms.contains(&Permission::IncidentCreate)); // from ApiIntegration
    }

    #[test]
    fn test_permission_count() {
        assert_eq!(Permission::ALL.len(), 20);
    }
}
