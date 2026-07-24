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
