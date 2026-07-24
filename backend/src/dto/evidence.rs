use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Responses
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize)]
pub struct EvidenceListResponse {
    pub evidence: Vec<EvidenceResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
}

// ---------------------------------------------------------------------------
// Pagination
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct EvidencePaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl EvidencePaginationParams {
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
    fn test_evidence_response_serialization() {
        let response = EvidenceResponse {
            id: Uuid::new_v4(),
            incident_id: Uuid::new_v4(),
            file_name: "test.jpg".to_string(),
            content_type: "image/jpeg".to_string(),
            file_size: 1024,
            sha256: "abc123".to_string(),
            width: Some(1920),
            height: Some(1080),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("file_name"));
        assert!(json.contains("content_type"));
        assert!(json.contains("sha256"));
    }

    #[test]
    fn test_evidence_list_response_serialization() {
        let list = EvidenceListResponse {
            evidence: vec![],
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
        let params = EvidencePaginationParams {
            page: None,
            per_page: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_pagination_offset_limit_custom() {
        let params = EvidencePaginationParams {
            page: Some(3),
            per_page: Some(10),
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 20);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_pagination_max_per_page() {
        let params = EvidencePaginationParams {
            page: Some(1),
            per_page: Some(200),
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }

    #[test]
    fn test_pagination_min_page() {
        let params = EvidencePaginationParams {
            page: Some(0),
            per_page: Some(10),
        };
        let (offset, _) = params.offset_limit();
        assert_eq!(offset, 0);
    }
}
