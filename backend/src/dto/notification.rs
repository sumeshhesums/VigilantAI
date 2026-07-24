use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{NotificationChannel, NotificationStatus};

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct SendNotificationRequest {
    pub incident_id: Uuid,
    pub channel: NotificationChannel,
    pub recipient: String,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub channel: NotificationChannel,
    pub recipient: String,
    pub status: NotificationStatus,
    pub attempts: i32,
    pub response_code: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct NotificationListResponse {
    pub notifications: Vec<NotificationResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct NotificationPaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub status: Option<NotificationStatus>,
    pub channel: Option<NotificationChannel>,
    pub incident_id: Option<Uuid>,
}

impl NotificationPaginationParams {
    pub fn offset_limit(&self) -> (u32, u32) {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;
        (offset, per_page)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_notification_request_deserialize() {
        let json = r#"{
            "incident_id": "550e8400-e29b-41d4-a716-446655440000",
            "channel": "webhook",
            "recipient": "https://example.com/hook"
        }"#;
        let req: SendNotificationRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.channel, NotificationChannel::Webhook);
        assert_eq!(req.recipient, "https://example.com/hook");
    }

    #[test]
    fn test_notification_response_serialization() {
        let response = NotificationResponse {
            id: Uuid::new_v4(),
            incident_id: Uuid::new_v4(),
            channel: NotificationChannel::Email,
            recipient: "admin@example.com".to_string(),
            status: NotificationStatus::Sent,
            attempts: 1,
            response_code: Some(200),
            error_message: None,
            created_at: Utc::now(),
            sent_at: Some(Utc::now()),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("email"));
        assert!(json.contains("sent"));
    }

    #[test]
    fn test_notification_list_response_serialization() {
        let list = NotificationListResponse {
            notifications: vec![],
            total: 0,
            page: 1,
            per_page: 20,
        };
        let json = serde_json::to_string(&list).unwrap();
        assert!(json.contains("total"));
        assert!(json.contains("page"));
    }

    #[test]
    fn test_pagination_offset_limit_default() {
        let params = NotificationPaginationParams {
            page: None,
            per_page: None,
            status: None,
            channel: None,
            incident_id: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_pagination_offset_limit_custom() {
        let params = NotificationPaginationParams {
            page: Some(3),
            per_page: Some(10),
            status: None,
            channel: None,
            incident_id: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 20);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_pagination_max_per_page() {
        let params = NotificationPaginationParams {
            page: Some(1),
            per_page: Some(200),
            status: None,
            channel: None,
            incident_id: None,
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }

    #[test]
    fn test_pagination_min_page() {
        let params = NotificationPaginationParams {
            page: Some(0),
            per_page: Some(10),
            status: None,
            channel: None,
            incident_id: None,
        };
        let (offset, _) = params.offset_limit();
        assert_eq!(offset, 0);
    }
}
