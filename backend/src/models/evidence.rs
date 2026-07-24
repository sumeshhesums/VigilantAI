use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Evidence
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Evidence {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub content_type: String,
    pub file_size: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// CreateEvidence (internal payload)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateEvidence {
    pub incident_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub content_type: String,
    pub file_size: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_serialization_roundtrip() {
        let evidence = Evidence {
            id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            incident_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            file_name: "test.jpg".to_string(),
            file_path: "2024/01/15/incident_id/test.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            file_size: 1024,
            sha256: "abc123def456".to_string(),
            width: Some(1920),
            height: Some(1080),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&evidence).unwrap();
        let deserialized: Evidence = serde_json::from_str(&json).unwrap();
        assert_eq!(evidence.id, deserialized.id);
        assert_eq!(evidence.file_name, deserialized.file_name);
        assert_eq!(evidence.width, deserialized.width);
    }

    #[test]
    fn test_create_evidence_fields() {
        let create = CreateEvidence {
            incident_id: Uuid::new_v4(),
            file_name: "snapshot.jpg".to_string(),
            file_path: "2024/01/15/uuid/snapshot.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            file_size: 2048,
            sha256: "abc123".to_string(),
            width: Some(640),
            height: Some(480),
        };

        assert_eq!(create.content_type, "image/jpeg");
        assert_eq!(create.file_size, 2048);
    }

    #[test]
    fn test_evidence_optional_dimensions() {
        let json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "incident_id": "550e8400-e29b-41d4-a716-446655440001",
            "file_name": "test.jpg",
            "file_path": "2024/01/15/test.jpg",
            "content_type": "image/jpeg",
            "file_size": 1024,
            "sha256": "abc123",
            "width": null,
            "height": null,
            "created_at": "2024-01-15T10:00:00Z"
        }"#;

        let evidence: Evidence = serde_json::from_str(json).unwrap();
        assert!(evidence.width.is_none());
        assert!(evidence.height.is_none());
    }
}
