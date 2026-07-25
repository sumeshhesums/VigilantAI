use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

use crate::dto::camera::{CreateCameraRequest, UpdateCameraRequest};
use crate::models::{Camera, CreateCamera, UpdateCamera};
use crate::repository::CameraRepository;

pub struct CameraService<R: CameraRepository> {
    repository: R,
}

impl<R: CameraRepository> CameraService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn list_cameras(
        &self,
        pool: &PgPool,
        offset: u32,
        limit: u32,
    ) -> Result<(Vec<Camera>, i64)> {
        let cameras = self
            .repository
            .list_paginated(pool, offset as i64, limit as i64)
            .await?;
        let total = self.repository.count(pool).await?;
        Ok((cameras, total))
    }

    pub async fn get_camera(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Camera> {
        self.repository
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("camera not found"))
    }

    pub async fn create_camera(&self, pool: &PgPool, req: &CreateCameraRequest) -> Result<Camera> {
        Self::validate_create(req)?;

        if let Some(existing) = self.repository.find_by_name(pool, &req.name).await? {
            if !existing.id.is_nil() {
                return Err(anyhow!("camera name already exists"));
            }
        }

        if self
            .repository
            .find_by_rtsp_url(pool, &req.rtsp_url)
            .await?
            .is_some()
        {
            return Err(anyhow!("RTSP URL already registered"));
        }

        let create = CreateCamera {
            name: req.name.clone(),
            location: req.location.clone(),
            rtsp_url: req.rtsp_url.clone(),
            fps: req.fps,
            resolution: req.resolution.clone(),
        };

        self.repository.create(pool, &create).await
    }

    pub async fn update_camera(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        req: &UpdateCameraRequest,
    ) -> Result<Camera> {
        if let Some(ref name) = req.name {
            Self::validate_name(name)?;
            if let Some(existing) = self.repository.find_by_name(pool, name).await? {
                if existing.id != id {
                    return Err(anyhow!("camera name already exists"));
                }
            }
        }

        if let Some(ref url) = req.rtsp_url {
            Self::validate_rtsp_url(url)?;
            if let Some(existing) = self.repository.find_by_rtsp_url(pool, url).await? {
                if existing.id != id {
                    return Err(anyhow!("RTSP URL already registered"));
                }
            }
        }

        if let Some(Some(f)) = req.fps {
            Self::validate_fps(f)?;
        }

        if let Some(Some(ref r)) = req.resolution {
            Self::validate_resolution(r)?;
        }

        let update = UpdateCamera {
            name: req.name.clone(),
            location: req.location.clone(),
            rtsp_url: req.rtsp_url.clone(),
            fps: req.fps,
            resolution: req.resolution.clone(),
            enabled: req.enabled,
        };

        self.repository
            .update(pool, id, &update)
            .await?
            .ok_or_else(|| anyhow!("camera not found"))
    }

    pub async fn delete_camera(&self, pool: &PgPool, id: uuid::Uuid) -> Result<()> {
        let deleted = self.repository.delete(pool, id).await?;
        if !deleted {
            return Err(anyhow!("camera not found"));
        }
        Ok(())
    }

    pub async fn enable_camera(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Camera> {
        self.repository
            .enable(pool, id)
            .await?
            .ok_or_else(|| anyhow!("camera not found"))
    }

    pub async fn disable_camera(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Camera> {
        self.repository
            .disable(pool, id)
            .await?
            .ok_or_else(|| anyhow!("camera not found"))
    }

    // -----------------------------------------------------------------------
    // Validation
    // -----------------------------------------------------------------------

    fn validate_create(req: &CreateCameraRequest) -> Result<()> {
        Self::validate_name(&req.name)?;
        Self::validate_rtsp_url(&req.rtsp_url)?;
        if let Some(fps) = req.fps {
            Self::validate_fps(fps)?;
        }
        if let Some(ref resolution) = req.resolution {
            Self::validate_resolution(resolution)?;
        }
        Ok(())
    }

    fn validate_name(name: &str) -> Result<()> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("camera name is required"));
        }
        if trimmed.len() < 3 {
            return Err(anyhow!(
                "camera name must be at least 3 characters, got {}",
                trimmed.len()
            ));
        }
        if trimmed.len() > 150 {
            return Err(anyhow!(
                "camera name must be at most 150 characters, got {}",
                trimmed.len()
            ));
        }
        Ok(())
    }

    fn validate_rtsp_url(url: &str) -> Result<()> {
        let trimmed = url.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("RTSP URL is required"));
        }
        if !trimmed.starts_with("rtsp://") && !trimmed.starts_with("rtsps://") {
            return Err(anyhow!("RTSP URL must start with rtsp:// or rtsps://"));
        }
        Ok(())
    }

    fn validate_fps(fps: i32) -> Result<()> {
        if !(1..=120).contains(&fps) {
            return Err(anyhow!("FPS must be between 1 and 120, got {fps}"));
        }
        Ok(())
    }

    fn validate_resolution(resolution: &str) -> Result<()> {
        let parts: Vec<&str> = resolution.split('x').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "resolution must be in WIDTHxHEIGHT format, e.g. 1920x1080"
            ));
        }
        let width: i32 = parts[0]
            .parse()
            .map_err(|_| anyhow!("invalid resolution width: {}", parts[0]))?;
        let height: i32 = parts[1]
            .parse()
            .map_err(|_| anyhow!("invalid resolution height: {}", parts[1]))?;
        if width < 1 || height < 1 {
            return Err(anyhow!("resolution dimensions must be positive"));
        }
        if width > 7680 || height > 4320 {
            return Err(anyhow!("resolution exceeds maximum 7680x4320"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_too_short() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_name("ab");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("at least 3 characters"));
    }

    #[test]
    fn test_validate_name_too_long() {
        let name = "a".repeat(151);
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_name(&name);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at most 150"));
    }

    #[test]
    fn test_validate_name_empty() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_name("   ");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("required"));
    }

    #[test]
    fn test_validate_name_valid() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_name("Lobby Camera");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rtsp_url_missing_prefix() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_rtsp_url("http://192.168.1.1/stream");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must start with rtsp:// or rtsps://"));
    }

    #[test]
    fn test_validate_rtsp_url_empty() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_rtsp_url("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("required"));
    }

    #[test]
    fn test_validate_rtsp_url_valid() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_rtsp_url("rtsp://192.168.1.100:554/stream1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rtsp_url_valid_rtsps() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_rtsp_url("rtsps://192.168.1.100:554/stream1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_fps_too_low() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_fps(0);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("between 1 and 120"));
    }

    #[test]
    fn test_validate_fps_too_high() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_fps(121);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("between 1 and 120"));
    }

    #[test]
    fn test_validate_fps_valid() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_fps(30);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_fps_boundaries() {
        assert!(CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_fps(1).is_ok());
        assert!(CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_fps(120).is_ok());
    }

    #[test]
    fn test_validate_resolution_bad_format() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("1920");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("WIDTHxHEIGHT"));
    }

    #[test]
    fn test_validate_resolution_non_numeric() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("abx1080");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_resolution_too_large() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("8000x5000");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("exceeds maximum"));
    }

    #[test]
    fn test_validate_resolution_zero() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("0x0");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be positive"));
    }

    #[test]
    fn test_validate_resolution_valid() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("1920x1080");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_resolution_4k() {
        let result = CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("3840x2160");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_resolution_boundary() {
        assert!(CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("1x1").is_ok());
        assert!(CameraService::<crate::repository::camera_repository::PostgresCameraRepository>::validate_resolution("7680x4320").is_ok());
    }
}
