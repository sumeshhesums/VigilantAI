use std::sync::Arc;
use std::time::Instant;

use crate::gateway::manager::{AiHealthMetrics, BackendHealthMetrics};
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
            ai: AiHealthSnapshot::default(),
            backend: BackendHealthSnapshot::default(),
        }
    }

    pub async fn check_with_ai(&self, ai_metrics: AiHealthMetrics) -> HealthResponse {
        HealthResponse {
            uptime_secs: self.uptime().await.as_secs(),
            workers: self.worker_count().await,
            cameras: self.camera_count().await,
            online: self.online_cameras().await,
            offline: self.offline_cameras().await,
            ai: AiHealthSnapshot {
                ai_reachable: ai_metrics.ai_reachable,
                last_inference_secs_ago: ai_metrics.last_inference.map(|t| t.elapsed().as_secs()),
                successful_requests: ai_metrics.successful_requests,
                failed_requests: ai_metrics.failed_requests,
            },
            backend: BackendHealthSnapshot::default(),
        }
    }

    pub async fn check_full(
        &self,
        ai_metrics: AiHealthMetrics,
        backend_metrics: BackendHealthMetrics,
    ) -> HealthResponse {
        HealthResponse {
            uptime_secs: self.uptime().await.as_secs(),
            workers: self.worker_count().await,
            cameras: self.camera_count().await,
            online: self.online_cameras().await,
            offline: self.offline_cameras().await,
            ai: AiHealthSnapshot {
                ai_reachable: ai_metrics.ai_reachable,
                last_inference_secs_ago: ai_metrics.last_inference.map(|t| t.elapsed().as_secs()),
                successful_requests: ai_metrics.successful_requests,
                failed_requests: ai_metrics.failed_requests,
            },
            backend: BackendHealthSnapshot {
                backend_reachable: backend_metrics.backend_reachable,
                successful_publishes: backend_metrics.successful_publishes,
                failed_publishes: backend_metrics.failed_publishes,
                last_publish_secs_ago: backend_metrics.last_publish.map(|t| t.elapsed().as_secs()),
            },
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct AiHealthSnapshot {
    pub ai_reachable: bool,
    pub last_inference_secs_ago: Option<u64>,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BackendHealthSnapshot {
    pub backend_reachable: bool,
    pub successful_publishes: u64,
    pub failed_publishes: u64,
    pub last_publish_secs_ago: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub uptime_secs: u64,
    pub workers: usize,
    pub cameras: usize,
    pub online: usize,
    pub offline: usize,
    pub ai: AiHealthSnapshot,
    pub backend: BackendHealthSnapshot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GatewayConfig;
    use crate::gateway::manager::GatewayManager;
    use crate::models::Camera;

    fn test_config() -> GatewayConfig {
        GatewayConfig {
            rtsp: crate::config::RtspConfig {
                simulated: true,
                ..crate::config::RtspConfig::default()
            },
            ..GatewayConfig::default()
        }
    }

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
        assert!(!response.ai.ai_reachable);
    }

    #[tokio::test]
    async fn test_health_with_cameras() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let cam1 = test_camera("online-cam", true);
        let cam2 = test_camera("offline-cam", true);
        let cam3 = test_camera("disabled-cam", false);

        manager.register_camera(cam1.clone()).await.unwrap();
        manager.register_camera(cam2.clone()).await.unwrap();
        manager.register_camera(cam3.clone()).await.unwrap();

        manager.start_worker(cam1.id).await;

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
            ai: AiHealthSnapshot::default(),
            backend: BackendHealthSnapshot::default(),
        };
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: HealthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response, deserialized);
    }

    #[tokio::test]
    async fn test_health_with_ai_metrics() {
        let state = Arc::new(GatewayState::new());
        let health = GatewayHealth::new(Arc::clone(&state));

        let ai_metrics = AiHealthMetrics {
            ai_reachable: true,
            last_inference: Some(Instant::now() - std::time::Duration::from_secs(5)),
            successful_requests: 100,
            failed_requests: 3,
        };

        let response = health.check_with_ai(ai_metrics).await;
        assert!(response.ai.ai_reachable);
        assert_eq!(response.ai.successful_requests, 100);
        assert_eq!(response.ai.failed_requests, 3);
        assert!(response.ai.last_inference_secs_ago.is_some());
    }

    #[test]
    fn test_ai_health_snapshot_default() {
        let snap = AiHealthSnapshot::default();
        assert!(!snap.ai_reachable);
        assert!(snap.last_inference_secs_ago.is_none());
        assert_eq!(snap.successful_requests, 0);
        assert_eq!(snap.failed_requests, 0);
    }

    #[test]
    fn test_ai_health_snapshot_serialization() {
        let snap = AiHealthSnapshot {
            ai_reachable: true,
            last_inference_secs_ago: Some(10),
            successful_requests: 50,
            failed_requests: 2,
        };
        let json = serde_json::to_string(&snap).unwrap();
        let parsed: AiHealthSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(snap, parsed);
    }

    #[test]
    fn test_health_response_includes_ai_field() {
        let response = HealthResponse {
            uptime_secs: 0,
            workers: 0,
            cameras: 0,
            online: 0,
            offline: 0,
            ai: AiHealthSnapshot {
                ai_reachable: true,
                last_inference_secs_ago: None,
                successful_requests: 0,
                failed_requests: 0,
            },
            backend: BackendHealthSnapshot::default(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ai_reachable"));
        assert!(json.contains("backend_reachable"));
    }

    #[test]
    fn test_backend_health_snapshot_default() {
        let snap = BackendHealthSnapshot::default();
        assert!(!snap.backend_reachable);
        assert_eq!(snap.successful_publishes, 0);
        assert_eq!(snap.failed_publishes, 0);
        assert!(snap.last_publish_secs_ago.is_none());
    }

    #[test]
    fn test_backend_health_snapshot_serialization() {
        let snap = BackendHealthSnapshot {
            backend_reachable: true,
            successful_publishes: 25,
            failed_publishes: 1,
            last_publish_secs_ago: Some(3),
        };
        let json = serde_json::to_string(&snap).unwrap();
        let parsed: BackendHealthSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(snap, parsed);
    }

    #[tokio::test]
    async fn test_check_full_with_backend_metrics() {
        let state = Arc::new(GatewayState::new());
        let health = GatewayHealth::new(Arc::clone(&state));

        let ai_metrics = AiHealthMetrics {
            ai_reachable: true,
            last_inference: Some(Instant::now() - std::time::Duration::from_secs(2)),
            successful_requests: 50,
            failed_requests: 1,
        };

        let backend_metrics = BackendHealthMetrics {
            backend_reachable: true,
            last_publish: Some(Instant::now() - std::time::Duration::from_secs(5)),
            successful_publishes: 30,
            failed_publishes: 2,
        };

        let response = health.check_full(ai_metrics, backend_metrics).await;
        assert!(response.ai.ai_reachable);
        assert!(response.backend.backend_reachable);
        assert_eq!(response.backend.successful_publishes, 30);
        assert_eq!(response.backend.failed_publishes, 2);
        assert!(response.backend.last_publish_secs_ago.is_some());
    }
}
