use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// System roles with hierarchical permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    SystemAdmin,
    SecurityAdmin,
    SecurityAnalyst,
    Operator,
    Viewer,
    ApiIntegration,
}

impl Role {
    /// All variants in display order.
    pub const ALL: &'static [Role] = &[
        Role::SystemAdmin,
        Role::SecurityAdmin,
        Role::SecurityAnalyst,
        Role::Operator,
        Role::Viewer,
        Role::ApiIntegration,
    ];

    /// The canonical database string representation.
    pub fn as_db_str(&self) -> &'static str {
        match self {
            Role::SystemAdmin => "system_admin",
            Role::SecurityAdmin => "security_admin",
            Role::SecurityAnalyst => "security_analyst",
            Role::Operator => "operator",
            Role::Viewer => "viewer",
            Role::ApiIntegration => "api_integration",
        }
    }

    /// Parse from the database string representation.
    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "system_admin" => Some(Role::SystemAdmin),
            "security_admin" => Some(Role::SecurityAdmin),
            "security_analyst" => Some(Role::SecurityAnalyst),
            "operator" => Some(Role::Operator),
            "viewer" => Some(Role::Viewer),
            "api_integration" => Some(Role::ApiIntegration),
            _ => None,
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown role: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_display_roundtrip() {
        for role in Role::ALL {
            let s = role.to_string();
            let parsed: Role = s.parse().unwrap();
            assert_eq!(*role, parsed);
        }
    }

    #[test]
    fn test_role_db_str_roundtrip() {
        for role in Role::ALL {
            let db_str = role.as_db_str();
            let parsed = Role::from_db_str(db_str).unwrap();
            assert_eq!(*role, parsed);
        }
    }

    #[test]
    fn test_role_from_str_invalid() {
        assert!("invalid_role".parse::<Role>().is_err());
    }

    #[test]
    fn test_role_serialization() {
        let role = Role::SystemAdmin;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, "\"system_admin\"");

        let deserialized: Role = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Role::SystemAdmin);
    }

    #[test]
    fn test_role_count() {
        assert_eq!(Role::ALL.len(), 6);
    }
}
