use std::sync::Arc;
use std::time::Instant;

use crate::gateway::state::GatewayState;

/// Provides gateway health information.
pub struct GatewayHealth {
    started_at: Instant,
    state: Arc<GatewayState>,
}

impl GatewayHealth {
    pub fn new(state: Arc<GatewayState>) -> Self {
        Self {
            started_at: Instant::now(),
            state,
        }
    }

    pub async fn uptime(&self) -> std::time::Duration {
        self.started_at.elapsed()
    }

    pub async fn worker_count(&self) -> usize {
        self.state.camera_count().await
    }

    pub async fn camera_count(&self) -> usize {
        self.state.camera_count().await
    }

    pub async fn online_cameras(&self) -> usize {
        self.state.online_count().await
    }

    pub async fn offline_cameras(&self) -> usize {
        self.state.offline_count().await
    }

    pub async fn check(&self) -> HealthResponse {
        HealthResponse {
            uptime_secs: self.uptime().await.as_secs(),
            workers: self.worker_count().await,
            cameras: self.camera_count().await,
            online: self.online_cameras().await,
            offline: self.offline_cameras().await,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub uptime_secs: u64,
    pub workers: usize,
    pub cameras: usize,
    pub online: usize,
    pub offline: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::manager::GatewayManager;
    use crate::models::Camera;

    fn test_camera(name: &str, enabled: bool) -> Camera {
        Camera {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            rtsp_url: "rtsp://10.0.0.1:554/stream".to_string(),
            location: None,
            fps: None,
            resolution: None,
            enabled,
        }
    }

    #[tokio::test]
    async fn test_health_empty() {
        let state = Arc::new(GatewayState::new());
        let health = GatewayHealth::new(Arc::clone(&state));

        let response = health.check().await;
        assert_eq!(response.cameras, 0);
        assert_eq!(response.online, 0);
        assert_eq!(response.offline, 0);
        assert_eq!(response.workers, 0);
        assert!(response.uptime_secs < 1000);
    }

    #[tokio::test]
    async fn test_health_with_cameras() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let cam1 = test_camera("online-cam", true);
        let cam2 = test_camera("offline-cam", true);
        let cam3 = test_camera("disabled-cam", false);

        manager.register_camera(cam1.clone()).await;
        manager.register_camera(cam2.clone()).await;
        manager.register_camera(cam3.clone()).await;

        manager.start_worker(cam1.id).await;
        // cam2 stays offline, cam3 stays offline (disabled)

        let health = GatewayHealth::new(Arc::clone(&state));
        let response = health.check().await;

        assert_eq!(response.cameras, 3);
        assert_eq!(response.online, 1);
        assert_eq!(response.offline, 2);
    }

    #[tokio::test]
    async fn test_uptime_increases() {
        let state = Arc::new(GatewayState::new());
        let health = GatewayHealth::new(Arc::clone(&state));

        let u1 = health.uptime().await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let u2 = health.uptime().await;

        assert!(u2 > u1);
    }

    #[tokio::test]
    async fn test_health_response_serialization() {
        let response = HealthResponse {
            uptime_secs: 42,
            workers: 5,
            cameras: 5,
            online: 3,
            offline: 2,
        };
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: HealthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, deserialized);
    }
}
