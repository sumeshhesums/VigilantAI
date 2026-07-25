use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{BoundingBox, IncidentSeverity, IncidentStatus};

// ---------------------------------------------------------------------------
// Requests
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct CreateIncidentRequest {
    pub camera_id: Uuid,
    pub timestamp: Option<DateTime<Utc>>,
    pub severity: IncidentSeverity,
    pub event_type: String,
    pub confidence: f64,
    pub bounding_box: Option<BoundingBox>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIncidentRequest {
    pub status: IncidentStatus,
}

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct IncidentResponse {
    pub id: Uuid,
    pub camera_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub event_type: String,
    pub confidence: f64,
    pub bounding_box: Option<BoundingBox>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct IncidentListResponse {
    pub incidents: Vec<IncidentResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub pages: u32,
}

// ---------------------------------------------------------------------------
// Pagination / Filtering
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct IncidentPaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub camera_id: Option<Uuid>,
    pub severity: Option<IncidentSeverity>,
    pub status: Option<IncidentStatus>,
    pub event_type: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
}

impl IncidentPaginationParams {
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
    fn test_pagination_offset_limit_default() {
        let params = IncidentPaginationParams {
            page: None,
            per_page: None,
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_pagination_offset_limit_custom() {
        let params = IncidentPaginationParams {
            page: Some(3),
            per_page: Some(10),
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 20);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_pagination_max_per_page() {
        let params = IncidentPaginationParams {
            page: Some(1),
            per_page: Some(200),
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }

    #[test]
    fn test_pagination_min_page() {
        let params = IncidentPaginationParams {
            page: Some(0),
            per_page: Some(10),
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (offset, _) = params.offset_limit();
        assert_eq!(offset, 0);
    }
}
