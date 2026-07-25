use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateCameraRequest {
    pub name: String,
    pub location: Option<String>,
    pub rtsp_url: String,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCameraRequest {
    pub name: Option<String>,
    pub location: Option<Option<String>>,
    pub rtsp_url: Option<String>,
    pub fps: Option<Option<i32>>,
    pub resolution: Option<Option<String>>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct CameraResponse {
    pub id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub rtsp_url: String,
    pub status: String,
    pub enabled: bool,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CameraListResponse {
    pub cameras: Vec<CameraResponse>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub pages: u32,
}

#[derive(Debug, Deserialize)]
pub struct CameraPaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

impl CameraPaginationParams {
    pub fn offset_limit(&self) -> (u32, u32) {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(20).min(100);
        let offset = (page - 1) * per_page;
        (offset, per_page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_defaults() {
        let params = CameraPaginationParams {
            page: None,
            per_page: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_pagination_page_2() {
        let params = CameraPaginationParams {
            page: Some(2),
            per_page: Some(10),
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 10);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_pagination_clamps_max_per_page() {
        let params = CameraPaginationParams {
            page: Some(1),
            per_page: Some(200),
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }

    #[test]
    fn test_pagination_clamps_min_page() {
        let params = CameraPaginationParams {
            page: Some(0),
            per_page: Some(10),
        };
        let (offset, _) = params.offset_limit();
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_pagination_large_page() {
        let params = CameraPaginationParams {
            page: Some(100),
            per_page: Some(10),
        };
        let (offset, _) = params.offset_limit();
        assert_eq!(offset, 990);
    }

    #[test]
    fn test_camera_list_response_has_pages() {
        let response = CameraListResponse {
            cameras: vec![],
            total: 0,
            page: 1,
            per_page: 20,
            pages: 0,
        };
        assert_eq!(response.pages, 0);
        assert_eq!(response.total, 0);
    }
}
