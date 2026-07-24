use std::sync::Arc;

use crate::ai::error::AIClientError;
use crate::backend::error::BackendClientError;
use crate::config::GatewayConfig;
use crate::models::Camera;
use crate::stream::reconnect::BackoffState;

use super::state::GatewayState;
use super::worker::CameraWorker;

/// Aggregate AI inference health metrics.
#[derive(Debug, Clone, Default)]
pub struct AiHealthMetrics {
    pub ai_reachable: bool,
    pub last_inference: Option<std::time::Instant>,
    pub successful_requests: u64,
    pub failed_requests: u64,
}

/// Aggregate backend incident publishing health metrics.
#[derive(Debug, Clone, Default)]
pub struct BackendHealthMetrics {
    pub backend_reachable: bool,
    pub last_publish: Option<std::time::Instant>,
    pub successful_publishes: u64,
    pub failed_publishes: u64,
}

/// Manages camera registration, worker lifecycle, and automatic reconnection.
pub struct GatewayManager {
    state: Arc<GatewayState>,
    config: GatewayConfig,
}

impl GatewayManager {
    pub fn new(state: Arc<GatewayState>, config: GatewayConfig) -> Self {
        Self { state, config }
    }

    /// Register a new camera and create a worker for it.
    pub async fn register_camera(&self, camera: Camera) -> Result<Arc<CameraWorker>, String> {
        let worker = Arc::new(
            CameraWorker::new(
                &camera,
                self.config.rtsp.clone(),
                self.config.ai.clone(),
                self.config.backend.clone(),
            )
            .map_err(|e| e.to_string())?,
        );
        self.state
            .cameras
            .write()
            .await
            .insert(camera.id, Arc::clone(&worker));
        self.state.increment_registrations();
        Ok(worker)
    }

    /// Remove a camera and stop its worker.
    pub async fn remove_camera(&self, camera_id: uuid::Uuid) -> bool {
        if let Some(worker) = self.state.cameras.write().await.remove(&camera_id) {
            worker.stop().await;
            self.state.increment_removals();
            true
        } else {
            false
        }
    }

    /// Start the worker for a given camera.
    pub async fn start_worker(&self, camera_id: uuid::Uuid) -> bool {
        let cameras = self.state.cameras.read().await;
        if let Some(worker) = cameras.get(&camera_id) {
            worker.start().await;
            true
        } else {
            false
        }
    }

    /// Stop the worker for a given camera.
    pub async fn stop_worker(&self, camera_id: uuid::Uuid) -> bool {
        let cameras = self.state.cameras.read().await;
        if let Some(worker) = cameras.get(&camera_id) {
            worker.stop().await;
            true
        } else {
            false
        }
    }

    /// Restart the worker for a given camera.
    pub async fn restart_worker(&self, camera_id: uuid::Uuid) -> bool {
        let cameras = self.state.cameras.read().await;
        if let Some(worker) = cameras.get(&camera_id) {
            worker.restart().await;
            true
        } else {
            false
        }
    }

    /// Start a worker with automatic reconnection on failure.
    ///
    /// Attempts to connect using exponential backoff. Returns once the
    /// worker is online or retries are exhausted.
    pub async fn start_worker_with_reconnect(&self, camera_id: uuid::Uuid) -> bool {
        let worker = {
            let cameras = self.state.cameras.read().await;
            match cameras.get(&camera_id) {
                Some(w) => Arc::clone(w),
                None => return false,
            }
        };

        let policy = self.config.reconnect.to_policy();
        let mut backoff = BackoffState::new(policy);

        // First attempt
        worker.start().await;
        if worker.is_running() {
            return true;
        }

        // Reconnect loop
        while let Some(delay) = backoff.next_delay() {
            tokio::time::sleep(delay).await;
            worker.restart().await;
            if worker.is_running() {
                backoff.reset();
                return true;
            }
        }

        false
    }

    /// Perform heartbeat checks on all running workers.
    ///
    /// Returns the number of workers that failed their heartbeat.
    pub async fn heartbeat_all(&self) -> usize {
        let cameras = self.state.cameras.read().await;
        let mut failures = 0;
        for worker in cameras.values() {
            if worker.is_running() {
                worker.heartbeat().await;
                if !worker.is_running() {
                    failures += 1;
                }
            }
        }
        failures
    }

    /// Check AI service health via any registered worker's AI client.
    pub async fn check_ai_health(&self) -> Result<(), AIClientError> {
        let client = {
            let cameras = self.state.cameras.read().await;
            cameras.values().next().map(|w| w.ai_client().clone())
        };

        match client {
            Some(c) => c.health().await,
            None => Err(AIClientError::Offline { url: String::new() }),
        }
    }

    /// Collect aggregated AI inference metrics across all workers.
    pub async fn ai_health_metrics(&self) -> AiHealthMetrics {
        let cameras = self.state.cameras.read().await;

        let mut successful: u64 = 0;
        let mut failed: u64 = 0;
        let mut last_inference: Option<std::time::Instant> = None;

        for worker in cameras.values() {
            successful += worker.successful_requests();
            failed += worker.failed_requests();

            if let Some(t) = worker.last_inference_time().await {
                match last_inference {
                    Some(prev) if t > prev => last_inference = Some(t),
                    None => last_inference = Some(t),
                    _ => {}
                }
            }
        }

        let ai_reachable = self.check_ai_health().await.is_ok();

        AiHealthMetrics {
            ai_reachable,
            last_inference,
            successful_requests: successful,
            failed_requests: failed,
        }
    }

    /// Check backend API health via any registered worker's backend client.
    pub async fn check_backend_health(&self) -> Result<(), BackendClientError> {
        let client = {
            let cameras = self.state.cameras.read().await;
            cameras.values().find_map(|w| w.backend_client().cloned())
        };

        match client {
            Some(c) => c.health().await,
            None => Err(BackendClientError::Offline { url: String::new() }),
        }
    }

    /// Collect aggregated backend publishing metrics across all workers.
    pub async fn backend_health_metrics(&self) -> BackendHealthMetrics {
        let cameras = self.state.cameras.read().await;

        let mut successful: u64 = 0;
        let mut failed: u64 = 0;
        let mut last_publish: Option<std::time::Instant> = None;

        for worker in cameras.values() {
            successful += worker.successful_publishes();
            failed += worker.failed_publishes();

            if let Some(t) = worker.last_publish_time().await {
                match last_publish {
                    Some(prev) if t > prev => last_publish = Some(t),
                    None => last_publish = Some(t),
                    _ => {}
                }
            }
        }

        let backend_reachable = self.check_backend_health().await.is_ok();

        BackendHealthMetrics {
            backend_reachable,
            last_publish,
            successful_publishes: successful,
            failed_publishes: failed,
        }
    }

    /// Get a reference to the shared state.
    pub fn state(&self) -> &Arc<GatewayState> {
        &self.state
    }

    /// Get a reference to the configuration.
    pub fn config(&self) -> &GatewayConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> GatewayConfig {
        GatewayConfig::default()
    }

    fn test_camera(name: &str) -> Camera {
        Camera {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            rtsp_url: "rtsp://10.0.0.1:554/stream".to_string(),
            location: Some("Lobby".to_string()),
            fps: Some(30),
            resolution: Some("1920x1080".to_string()),
            enabled: true,
        }
    }

    #[tokio::test]
    async fn test_register_and_count() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        let _worker = manager.register_camera(camera).await.unwrap();
        assert_eq!(state.camera_count().await, 1);
    }

    #[tokio::test]
    async fn test_register_multiple() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        for i in 0..5 {
            let camera = test_camera(&format!("cam-{i}"));
            manager.register_camera(camera).await.unwrap();
        }
        assert_eq!(state.camera_count().await, 5);
    }

    #[tokio::test]
    async fn test_remove_camera() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await.unwrap();
        assert_eq!(state.camera_count().await, 1);

        let removed = manager.remove_camera(camera.id).await;
        assert!(removed);
        assert_eq!(state.camera_count().await, 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_returns_false() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let removed = manager.remove_camera(uuid::Uuid::new_v4()).await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_start_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await.unwrap();

        let started = manager.start_worker(camera.id).await;
        assert!(started);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_stop_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await.unwrap();
        manager.start_worker(camera.id).await;

        let stopped = manager.stop_worker(camera.id).await;
        assert!(stopped);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_restart_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await.unwrap();

        let restarted = manager.restart_worker(camera.id).await;
        assert!(restarted);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_start_nonexistent_returns_false() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let started = manager.start_worker(uuid::Uuid::new_v4()).await;
        assert!(!started);
    }

    #[tokio::test]
    async fn test_remove_stops_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        let worker = manager.register_camera(camera.clone()).await.unwrap();
        manager.start_worker(camera.id).await;
        assert!(worker.is_running());

        manager.remove_camera(camera.id).await;
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_state_accessor() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let manager_state = manager.state();
        assert!(Arc::ptr_eq(&state, manager_state));
    }

    #[tokio::test]
    async fn test_config_accessor() {
        let state = Arc::new(GatewayState::new());
        let config = test_config();
        let manager = GatewayManager::new(Arc::clone(&state), config);

        assert_eq!(
            manager.config().heartbeat_interval,
            std::time::Duration::from_secs(10)
        );
    }

    #[tokio::test]
    async fn test_start_worker_with_reconnect_success() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await.unwrap();

        let started = manager.start_worker_with_reconnect(camera.id).await;
        assert!(started);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_start_worker_with_reconnect_nonexistent() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let started = manager
            .start_worker_with_reconnect(uuid::Uuid::new_v4())
            .await;
        assert!(!started);
    }

    #[tokio::test]
    async fn test_heartbeat_all() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let cam1 = test_camera("cam1");
        let cam2 = test_camera("cam2");
        manager.register_camera(cam1.clone()).await.unwrap();
        manager.register_camera(cam2.clone()).await.unwrap();

        manager.start_worker(cam1.id).await;
        manager.start_worker(cam2.id).await;

        let failures = manager.heartbeat_all().await;
        assert_eq!(failures, 0);
    }

    #[tokio::test]
    async fn test_register_invalid_url_returns_error() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let camera = Camera {
            id: uuid::Uuid::new_v4(),
            name: "Bad Camera".to_string(),
            rtsp_url: "http://10.0.0.1/stream".to_string(),
            location: None,
            fps: None,
            resolution: None,
            enabled: true,
        };

        let result = manager.register_camera(camera).await;
        assert!(result.is_err());
        assert_eq!(state.camera_count().await, 0);
    }

    #[tokio::test]
    async fn test_concurrent_registrations() {
        let state = Arc::new(GatewayState::new());
        let manager = Arc::new(GatewayManager::new(Arc::clone(&state), test_config()));
        let mut handles = vec![];

        for i in 0..20 {
            let manager = Arc::clone(&manager);
            handles.push(tokio::spawn(async move {
                let camera = test_camera(&format!("cam-{i}"));
                manager.register_camera(camera).await.unwrap();
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(state.camera_count().await, 20);
    }

    #[tokio::test]
    async fn test_ai_health_metrics_initial() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let metrics = manager.ai_health_metrics().await;
        assert_eq!(metrics.successful_requests, 0);
        assert_eq!(metrics.failed_requests, 0);
        assert!(metrics.last_inference.is_none());
    }

    #[tokio::test]
    async fn test_ai_health_check_no_workers() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let result = manager.check_ai_health().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ai_health_check_offline() {
        let state = Arc::new(GatewayState::new());
        let config = GatewayConfig {
            ai: crate::config::AiConfig {
                service_url: "http://127.0.0.1:19995".to_string(),
                ..crate::config::AiConfig::default()
            },
            ..test_config()
        };
        let manager = GatewayManager::new(Arc::clone(&state), config);

        let camera = test_camera("cam1");
        manager.register_camera(camera).await.unwrap();

        let result = manager.check_ai_health().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ai_health_metrics_with_worker() {
        let state = Arc::new(GatewayState::new());
        let config = GatewayConfig {
            ai: crate::config::AiConfig {
                service_url: "http://127.0.0.1:19994".to_string(),
                ..crate::config::AiConfig::default()
            },
            ..test_config()
        };
        let manager = GatewayManager::new(Arc::clone(&state), config);

        let camera = test_camera("cam1");
        manager.register_camera(camera).await.unwrap();

        let metrics = manager.ai_health_metrics().await;
        assert!(!metrics.ai_reachable);
        assert_eq!(metrics.successful_requests, 0);
    }

    #[tokio::test]
    async fn test_backend_health_check_no_workers() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let result = manager.check_backend_health().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_backend_health_check_offline() {
        let state = Arc::new(GatewayState::new());
        let config = GatewayConfig {
            backend: crate::config::BackendConfig {
                url: "http://127.0.0.1:19993".to_string(),
                auth_token: "test-token".to_string(),
                ..crate::config::BackendConfig::default()
            },
            ..test_config()
        };
        let manager = GatewayManager::new(Arc::clone(&state), config);

        let camera = test_camera("cam1");
        manager.register_camera(camera).await.unwrap();

        let result = manager.check_backend_health().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_backend_health_metrics_initial() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state), test_config());

        let metrics = manager.backend_health_metrics().await;
        assert_eq!(metrics.successful_publishes, 0);
        assert_eq!(metrics.failed_publishes, 0);
        assert!(metrics.last_publish.is_none());
    }

    #[tokio::test]
    async fn test_backend_health_metrics_with_worker() {
        let state = Arc::new(GatewayState::new());
        let config = GatewayConfig {
            backend: crate::config::BackendConfig {
                url: "http://127.0.0.1:19992".to_string(),
                auth_token: "test-token".to_string(),
                ..crate::config::BackendConfig::default()
            },
            ..test_config()
        };
        let manager = GatewayManager::new(Arc::clone(&state), config);

        let camera = test_camera("cam1");
        manager.register_camera(camera).await.unwrap();

        let metrics = manager.backend_health_metrics().await;
        assert!(!metrics.backend_reachable);
        assert_eq!(metrics.successful_publishes, 0);
    }
}
