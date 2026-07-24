use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use crate::models::CameraStatus;

use super::worker::CameraWorker;

/// Shared gateway state providing aggregate statistics.
pub struct GatewayState {
    pub cameras: Arc<tokio::sync::RwLock<HashMap<uuid::Uuid, Arc<CameraWorker>>>>,
    pub total_registrations: AtomicU64,
    pub total_removals: AtomicU64,
}

impl GatewayState {
    pub fn new() -> Self {
        Self {
            cameras: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            total_registrations: AtomicU64::new(0),
            total_removals: AtomicU64::new(0),
        }
    }

    pub async fn camera_count(&self) -> usize {
        self.cameras.read().await.len()
    }

    pub async fn online_count(&self) -> usize {
        let cameras = self.cameras.read().await;
        let mut count = 0;
        for worker in cameras.values() {
            if worker.status().await == CameraStatus::Online {
                count += 1;
            }
        }
        count
    }

    pub async fn offline_count(&self) -> usize {
        let cameras = self.cameras.read().await;
        let mut count = 0;
        for worker in cameras.values() {
            let status = worker.status().await;
            if status == CameraStatus::Offline || status == CameraStatus::Stopped {
                count += 1;
            }
        }
        count
    }

    pub fn worker_count(&self) -> usize {
        // This is an approximation; the real count requires async access.
        // Use camera_count() for the async variant.
        self.total_registrations.load(Ordering::Relaxed) as usize
            - self.total_removals.load(Ordering::Relaxed) as usize
    }

    pub fn increment_registrations(&self) {
        self.total_registrations.fetch_add(1, Ordering::Relaxed);
    }

    pub fn increment_removals(&self) {
        self.total_removals.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for GatewayState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AiConfig, BackendConfig, RtspConfig};
    use crate::models::Camera;

    fn test_rtsp_config() -> RtspConfig {
        RtspConfig {
            connection_timeout: std::time::Duration::from_secs(5),
        }
    }

    fn test_camera(name: &str) -> Camera {
        Camera {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            rtsp_url: "rtsp://10.0.0.1:554/stream".to_string(),
            location: None,
            fps: None,
            resolution: None,
            enabled: true,
        }
    }

    #[tokio::test]
    async fn test_empty_state() {
        let state = GatewayState::new();
        assert_eq!(state.camera_count().await, 0);
        assert_eq!(state.online_count().await, 0);
        assert_eq!(state.offline_count().await, 0);
    }

    #[tokio::test]
    async fn test_camera_count_after_insert() {
        let state = GatewayState::new();
        let camera = test_camera("cam1");
        let worker = Arc::new(
            CameraWorker::new(
                &camera,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );

        state.cameras.write().await.insert(camera.id, worker);
        state.increment_registrations();

        assert_eq!(state.camera_count().await, 1);
    }

    #[tokio::test]
    async fn test_online_count() {
        let state = GatewayState::new();
        let camera = test_camera("cam1");
        let worker = Arc::new(
            CameraWorker::new(
                &camera,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );
        worker.start().await;

        state.cameras.write().await.insert(camera.id, worker);
        state.increment_registrations();

        assert_eq!(state.online_count().await, 1);
    }

    #[tokio::test]
    async fn test_offline_count_stopped() {
        let state = GatewayState::new();
        let camera = test_camera("cam1");
        let worker = Arc::new(
            CameraWorker::new(
                &camera,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );
        worker.start().await;
        worker.stop().await;

        state.cameras.write().await.insert(camera.id, worker);
        state.increment_registrations();

        assert_eq!(state.offline_count().await, 1);
    }

    #[tokio::test]
    async fn test_counts_mixed() {
        let state = GatewayState::new();

        let cam1 = test_camera("online-cam");
        let w1 = Arc::new(
            CameraWorker::new(
                &cam1,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );
        w1.start().await;
        state.cameras.write().await.insert(cam1.id, w1);
        state.increment_registrations();

        let cam2 = test_camera("offline-cam");
        let w2 = Arc::new(
            CameraWorker::new(
                &cam2,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );
        state.cameras.write().await.insert(cam2.id, w2);
        state.increment_registrations();

        let cam3 = test_camera("stopped-cam");
        let w3 = Arc::new(
            CameraWorker::new(
                &cam3,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );
        w3.start().await;
        w3.stop().await;
        state.cameras.write().await.insert(cam3.id, w3);
        state.increment_registrations();

        assert_eq!(state.camera_count().await, 3);
        assert_eq!(state.online_count().await, 1);
        assert_eq!(state.offline_count().await, 2);
    }

    #[tokio::test]
    async fn test_removal() {
        let state = GatewayState::new();
        let camera = test_camera("cam1");
        let worker = Arc::new(
            CameraWorker::new(
                &camera,
                test_rtsp_config(),
                AiConfig::default(),
                BackendConfig::default(),
            )
            .unwrap(),
        );

        state.cameras.write().await.insert(camera.id, worker);
        state.increment_registrations();
        assert_eq!(state.camera_count().await, 1);

        state.cameras.write().await.remove(&camera.id);
        state.increment_removals();
        assert_eq!(state.camera_count().await, 0);
    }

    #[tokio::test]
    async fn test_thread_safe_concurrent_inserts() {
        let state = Arc::new(GatewayState::new());
        let mut handles = vec![];

        for i in 0..10 {
            let state = Arc::clone(&state);
            handles.push(tokio::spawn(async move {
                let camera = Camera {
                    id: uuid::Uuid::new_v4(),
                    name: format!("cam-{i}"),
                    rtsp_url: format!("rtsp://10.0.0.{i}:554/stream"),
                    location: None,
                    fps: None,
                    resolution: None,
                    enabled: true,
                };
                let worker = Arc::new(
                    CameraWorker::new(
                        &camera,
                        test_rtsp_config(),
                        AiConfig::default(),
                        BackendConfig::default(),
                    )
                    .unwrap(),
                );
                state.cameras.write().await.insert(camera.id, worker);
                state.increment_registrations();
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(state.camera_count().await, 10);
    }
}
