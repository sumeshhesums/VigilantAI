use serde::{Deserialize, Serialize};

/// A role that can be assigned to users.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
}

/// Payload for creating a new role.
#[derive(Debug, Clone)]
pub struct CreateRole {
    pub name: String,
    pub description: Option<String>,
}
