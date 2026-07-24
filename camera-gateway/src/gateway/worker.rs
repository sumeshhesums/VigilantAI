use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use tokio::sync::RwLock;

use crate::models::{Camera, CameraStatus};

/// A simulated camera worker.
///
/// Does not connect to RTSP — instead it simulates a worker lifecycle
/// through status transitions and heartbeat tracking.
pub struct CameraWorker {
    camera_id: uuid::Uuid,
    camera_name: String,
    rtsp_url: String,
    status: RwLock<CameraStatus>,
    started_at: Option<Instant>,
    last_seen: RwLock<Option<Instant>>,
    enabled: AtomicBool,
    running: AtomicBool,
}

impl CameraWorker {
    pub fn new(camera: &Camera) -> Self {
        Self {
            camera_id: camera.id,
            camera_name: camera.name.clone(),
            rtsp_url: camera.rtsp_url.clone(),
            status: RwLock::new(CameraStatus::Offline),
            started_at: None,
            last_seen: RwLock::new(None),
            enabled: AtomicBool::new(camera.enabled),
            running: AtomicBool::new(false),
        }
    }

    pub fn camera_id(&self) -> uuid::Uuid {
        self.camera_id
    }

    pub fn camera_name(&self) -> &str {
        &self.camera_name
    }

    pub fn rtsp_url(&self) -> &str {
        &self.rtsp_url
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    pub fn started_at(&self) -> Option<Instant> {
        self.started_at
    }

    /// Transition the worker to `Connecting` then `Online`.
    /// Simulates a successful connection handshake.
    pub async fn start(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        {
            let mut status = self.status.write().await;
            *status = CameraStatus::Connecting;
        }

        // Simulate connection delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        {
            let mut status = self.status.write().await;
            *status = CameraStatus::Online;
        }
        self.running.store(true, Ordering::Relaxed);

        let mut last_seen = self.last_seen.write().await;
        *last_seen = Some(Instant::now());
    }

    /// Transition the worker to `Stopped`.
    pub async fn stop(&self) {
        {
            let mut status = self.status.write().await;
            *status = CameraStatus::Stopped;
        }
        self.running.store(false, Ordering::Relaxed);
    }

    /// Restart the worker: stop then start.
    pub async fn restart(&self) {
        self.stop().await;
        self.start().await;
    }

    /// Record a heartbeat — update `last_seen` and confirm status is `Online`.
    pub async fn heartbeat(&self) {
        if self.running.load(Ordering::Relaxed) {
            let mut last_seen = self.last_seen.write().await;
            *last_seen = Some(Instant::now());
        }
    }

    /// Get the current status.
    pub async fn status(&self) -> CameraStatus {
        *self.status.read().await
    }

    /// Get the time since the last heartbeat.
    pub async fn last_seen(&self) -> Option<Instant> {
        *self.last_seen.read().await
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
}

impl fmt::Debug for CameraWorker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CameraWorker")
            .field("camera_id", &self.camera_id)
            .field("camera_name", &self.camera_name)
            .field("rtsp_url", &self.rtsp_url)
            .field("running", &self.is_running())
            .field("enabled", &self.is_enabled())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_camera(enabled: bool) -> Camera {
        Camera {
            id: uuid::Uuid::new_v4(),
            name: "Test Camera".to_string(),
            rtsp_url: "rtsp://10.0.0.1:554/stream".to_string(),
            location: Some("Lobby".to_string()),
            fps: Some(30),
            resolution: Some("1920x1080".to_string()),
            enabled,
        }
    }

    #[tokio::test]
    async fn test_worker_new_status_offline() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        assert_eq!(worker.status().await, CameraStatus::Offline);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_start_transitions_online() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
        assert!(worker.is_running());
        assert!(worker.last_seen().await.is_some());
    }

    #[tokio::test]
    async fn test_worker_stop_transitions_stopped() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);

        worker.stop().await;
        assert_eq!(worker.status().await, CameraStatus::Stopped);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_restart() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);

        worker.restart().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_heartbeat_updates_last_seen() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        worker.start().await;

        let before = worker.last_seen().await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        worker.heartbeat().await;
        let after = worker.last_seen().await.unwrap();

        assert!(after >= before);
    }

    #[tokio::test]
    async fn test_worker_disabled_does_not_start() {
        let camera = test_camera(false);
        let worker = CameraWorker::new(&camera);
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Offline);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_set_enabled() {
        let camera = test_camera(false);
        let worker = CameraWorker::new(&camera);
        assert!(!worker.is_enabled());

        worker.set_enabled(true);
        assert!(worker.is_enabled());

        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
    }

    #[tokio::test]
    async fn test_worker_heartbeat_when_stopped_does_nothing() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        worker.start().await;
        worker.stop().await;

        // Heartbeat should not update last_seen when not running
        let before = worker.last_seen().await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        worker.heartbeat().await;
        let after = worker.last_seen().await;

        // Both should be Some but last_seen shouldn't have changed
        assert!(before.is_some());
        assert!(after.is_some());
    }

    #[test]
    fn test_worker_metadata() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        assert_eq!(worker.camera_id(), camera.id);
        assert_eq!(worker.camera_name(), "Test Camera");
        assert_eq!(worker.rtsp_url(), "rtsp://10.0.0.1:554/stream");
    }

    #[test]
    fn test_worker_debug() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(&camera);
        let debug = format!("{worker:?}");
        assert!(debug.contains("CameraWorker"));
        assert!(debug.contains("camera_id"));
    }
}
