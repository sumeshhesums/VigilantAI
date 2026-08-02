use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl IncidentSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            IncidentSeverity::Low => "low",
            IncidentSeverity::Medium => "medium",
            IncidentSeverity::High => "high",
            IncidentSeverity::Critical => "critical",
        }
    }

    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "low" => Some(IncidentSeverity::Low),
            "medium" => Some(IncidentSeverity::Medium),
            "high" => Some(IncidentSeverity::High),
            "critical" => Some(IncidentSeverity::Critical),
            _ => None,
        }
    }
}

impl std::fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct IncidentRequest {
    pub camera_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
    pub severity: IncidentSeverity,
    pub event_type: String,
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IncidentResponse {
    pub id: Uuid,
    pub camera_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub severity: IncidentSeverity,
    pub status: String,
    pub event_type: String,
    pub confidence: f64,
    pub bounding_box: Option<BoundingBox>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotificationRequest {
    pub incident_id: Uuid,
    pub channel: NotificationChannel,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub recipient: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Webhook,
    Email,
}

impl std::fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            NotificationChannel::Webhook => "webhook",
            NotificationChannel::Email => "email",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub channel: NotificationChannel,
    pub recipient: String,
    pub status: String,
    pub attempts: i32,
    pub response_code: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EvidenceResponse {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub file_size: i64,
    pub sha256: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_channel_display() {
        assert_eq!(NotificationChannel::Webhook.to_string(), "webhook");
        assert_eq!(NotificationChannel::Email.to_string(), "email");
    }

    #[test]
    fn test_notification_request_serialization() {
        let req = NotificationRequest {
            incident_id: Uuid::nil(),
            channel: NotificationChannel::Webhook,
            recipient: String::new(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"incident_id\""));
        assert!(json.contains("\"channel\":\"webhook\""));
        assert!(!json.contains("\"recipient\""));
    }

    #[test]
    fn test_notification_request_with_recipient() {
        let req = NotificationRequest {
            incident_id: Uuid::nil(),
            channel: NotificationChannel::Email,
            recipient: "ops@example.com".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"recipient\":\"ops@example.com\""));
    }

    #[test]
    fn test_notification_response_deserialization() {
        let json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "incident_id": "550e8400-e29b-41d4-a716-446655440001",
            "channel": "webhook",
            "recipient": "https://example.com/hook",
            "status": "sent",
            "attempts": 1,
            "response_code": 200,
            "error_message": null,
            "created_at": "2024-01-01T00:00:00Z",
            "sent_at": "2024-01-01T00:00:00Z"
        }"#;
        let resp: NotificationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.channel, NotificationChannel::Webhook);
        assert_eq!(resp.status, "sent");
        assert_eq!(resp.response_code, Some(200));
    }

    #[test]
    fn test_evidence_response_deserialization() {
        let json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "incident_id": "550e8400-e29b-41d4-a716-446655440001",
            "file_name": "frame.jpg",
            "content_type": "image/jpeg",
            "file_size": 1024,
            "sha256": "abc123",
            "width": 640,
            "height": 480,
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let resp: EvidenceResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.sha256, "abc123");
        assert_eq!(resp.width, Some(640));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_serialization_roundtrip() {
        let severities = [
            IncidentSeverity::Low,
            IncidentSeverity::Medium,
            IncidentSeverity::High,
            IncidentSeverity::Critical,
        ];
        for sev in &severities {
            let json = serde_json::to_string(sev).unwrap();
            let parsed: IncidentSeverity = serde_json::from_str(&json).unwrap();
            assert_eq!(*sev, parsed);
        }
    }

    #[test]
    fn test_severity_as_str() {
        assert_eq!(IncidentSeverity::Low.as_str(), "low");
        assert_eq!(IncidentSeverity::Medium.as_str(), "medium");
        assert_eq!(IncidentSeverity::High.as_str(), "high");
        assert_eq!(IncidentSeverity::Critical.as_str(), "critical");
    }

    #[test]
    fn test_severity_from_name() {
        assert_eq!(
            IncidentSeverity::from_name("low"),
            Some(IncidentSeverity::Low)
        );
        assert_eq!(
            IncidentSeverity::from_name("medium"),
            Some(IncidentSeverity::Medium)
        );
        assert_eq!(
            IncidentSeverity::from_name("high"),
            Some(IncidentSeverity::High)
        );
        assert_eq!(
            IncidentSeverity::from_name("critical"),
            Some(IncidentSeverity::Critical)
        );
        assert_eq!(IncidentSeverity::from_name("invalid"), None);
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(IncidentSeverity::Low.to_string(), "low");
        assert_eq!(IncidentSeverity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_bounding_box_serialization() {
        let bb = BoundingBox {
            x1: 10.0,
            y1: 20.0,
            x2: 100.0,
            y2: 200.0,
        };
        let json = serde_json::to_string(&bb).unwrap();
        let parsed: BoundingBox = serde_json::from_str(&json).unwrap();
        assert_eq!(bb, parsed);
    }

    #[test]
    fn test_incident_request_serialization() {
        let req = IncidentRequest {
            camera_id: Uuid::nil(),
            timestamp: None,
            severity: IncidentSeverity::High,
            event_type: "fire".to_string(),
            confidence: 0.95,
            bounding_box: Some(BoundingBox {
                x1: 0.0,
                y1: 0.0,
                x2: 100.0,
                y2: 100.0,
            }),
            metadata: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"severity\":\"high\""));
        assert!(json.contains("\"event_type\":\"fire\""));
        assert!(json.contains("\"confidence\":0.95"));
        assert!(!json.contains("\"timestamp\""));
        assert!(!json.contains("\"bounding_box\":null"));
    }

    #[test]
    fn test_incident_request_with_metadata() {
        let req = IncidentRequest {
            camera_id: Uuid::nil(),
            timestamp: None,
            severity: IncidentSeverity::Medium,
            event_type: "person".to_string(),
            confidence: 0.8,
            bounding_box: None,
            metadata: Some(serde_json::json!({"model": "yolov8n"})),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"model\""));
        assert!(!json.contains("\"bounding_box\""));
    }

    #[test]
    fn test_incident_response_deserialization() {
        let json = r#"{
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "camera_id": "550e8400-e29b-41d4-a716-446655440001",
            "timestamp": "2024-01-01T00:00:00Z",
            "severity": "critical",
            "status": "open",
            "event_type": "fire",
            "confidence": 0.99,
            "bounding_box": {"x1": 10.0, "y1": 20.0, "x2": 100.0, "y2": 200.0},
            "metadata": null,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let resp: IncidentResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.severity, IncidentSeverity::Critical);
        assert_eq!(resp.status, "open");
        assert_eq!(resp.event_type, "fire");
        assert!(resp.bounding_box.is_some());
    }

    #[test]
    fn test_error_response_deserialization() {
        let json = r#"{"error": "unauthorized: missing token", "status": 401}"#;
        let resp: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.status, 401);
        assert!(resp.error.contains("unauthorized"));
    }
}
