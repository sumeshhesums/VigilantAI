use std::sync::Arc;

use crate::models::Camera;

use super::state::GatewayState;
use super::worker::CameraWorker;

/// Manages camera registration and worker lifecycle.
pub struct GatewayManager {
    state: Arc<GatewayState>,
}

impl GatewayManager {
    pub fn new(state: Arc<GatewayState>) -> Self {
        Self { state }
    }

    /// Register a new camera and create a worker for it.
    pub async fn register_camera(&self, camera: Camera) -> Arc<CameraWorker> {
        let worker = Arc::new(CameraWorker::new(&camera));
        self.state
            .cameras
            .write()
            .await
            .insert(camera.id, Arc::clone(&worker));
        self.state.increment_registrations();
        worker
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

    /// Get a reference to the shared state.
    pub fn state(&self) -> &Arc<GatewayState> {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        let _worker = manager.register_camera(camera).await;
        assert_eq!(state.camera_count().await, 1);
    }

    #[tokio::test]
    async fn test_register_multiple() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        for i in 0..5 {
            let camera = test_camera(&format!("cam-{i}"));
            manager.register_camera(camera).await;
        }
        assert_eq!(state.camera_count().await, 5);
    }

    #[tokio::test]
    async fn test_remove_camera() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await;
        assert_eq!(state.camera_count().await, 1);

        let removed = manager.remove_camera(camera.id).await;
        assert!(removed);
        assert_eq!(state.camera_count().await, 0);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_returns_false() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let removed = manager.remove_camera(uuid::Uuid::new_v4()).await;
        assert!(!removed);
    }

    #[tokio::test]
    async fn test_start_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await;

        let started = manager.start_worker(camera.id).await;
        assert!(started);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_stop_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await;
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
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        manager.register_camera(camera.clone()).await;

        let restarted = manager.restart_worker(camera.id).await;
        assert!(restarted);

        let cameras = state.cameras.read().await;
        let worker = cameras.get(&camera.id).unwrap();
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_start_nonexistent_returns_false() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let started = manager.start_worker(uuid::Uuid::new_v4()).await;
        assert!(!started);
    }

    #[tokio::test]
    async fn test_remove_stops_worker() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let camera = test_camera("cam1");
        let worker = manager.register_camera(camera.clone()).await;
        manager.start_worker(camera.id).await;
        assert!(worker.is_running());

        manager.remove_camera(camera.id).await;
        // Worker should be stopped after removal
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_state_accessor() {
        let state = Arc::new(GatewayState::new());
        let manager = GatewayManager::new(Arc::clone(&state));

        let manager_state = manager.state();
        assert!(Arc::ptr_eq(&state, manager_state));
    }

    #[tokio::test]
    async fn test_concurrent_registrations() {
        let state = Arc::new(GatewayState::new());
        let manager = Arc::new(GatewayManager::new(Arc::clone(&state)));
        let mut handles = vec![];

        for i in 0..20 {
            let manager = Arc::clone(&manager);
            handles.push(tokio::spawn(async move {
                let camera = test_camera(&format!("cam-{i}"));
                manager.register_camera(camera).await;
            }));
        }

        for h in handles {
            h.await.unwrap();
        }

        assert_eq!(state.camera_count().await, 20);
    }
}
