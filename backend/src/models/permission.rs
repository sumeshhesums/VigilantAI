use serde::{Deserialize, Serialize};

/// A permission that can be assigned to roles.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Permission {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Payload for creating a new permission.
#[derive(Debug, Clone)]
pub struct CreatePermission {
    pub name: String,
    pub description: Option<String>,
}
