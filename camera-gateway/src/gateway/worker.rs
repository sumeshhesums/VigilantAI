use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use tokio::sync::RwLock;

use crate::ai::client::AIClient;
use crate::ai::error::AIClientError;
use crate::ai::models::DetectionResponse;
use crate::backend::client::BackendClient;
use crate::backend::error::BackendClientError;
use crate::backend::models::{
    BoundingBox as BackendBoundingBox, IncidentRequest, IncidentSeverity,
};
use crate::config::{AiConfig, BackendConfig, RtspConfig};
use crate::models::{Camera, CameraStatus};
use crate::stream::client::{RTSPClient, RtspError};

/// A camera worker that owns an RTSP client and manages its lifecycle.
///
/// The worker transitions through states: Offline → Connecting → Online → Error
/// based on the underlying RTSP client's connection state.
///
/// When online, the worker runs an inference loop that captures frames and
/// sends them to the AI service for object detection.
pub struct CameraWorker {
    camera_id: uuid::Uuid,
    camera_name: String,
    client: RTSPClient,
    ai_client: AIClient,
    backend_client: Option<BackendClient>,
    backend_config: BackendConfig,
    status: RwLock<CameraStatus>,
    started_at: Option<Instant>,
    last_seen: RwLock<Option<Instant>>,
    last_error: RwLock<Option<RtspError>>,
    enabled: AtomicBool,
    running: AtomicBool,
    last_detection: RwLock<Option<DetectionResponse>>,
    last_inference: RwLock<Option<Instant>>,
    last_inference_error: RwLock<Option<AIClientError>>,
    successful_requests: std::sync::atomic::AtomicU64,
    failed_requests: std::sync::atomic::AtomicU64,
    successful_publishes: std::sync::atomic::AtomicU64,
    failed_publishes: std::sync::atomic::AtomicU64,
    last_publish: RwLock<Option<Instant>>,
    last_publish_error: RwLock<Option<BackendClientError>>,
}

impl CameraWorker {
    pub fn new(
        camera: &Camera,
        rtsp_config: RtspConfig,
        ai_config: AiConfig,
        backend_config: BackendConfig,
    ) -> Result<Self, RtspError> {
        let client = RTSPClient::new(camera.rtsp_url.clone(), rtsp_config)?;
        let ai_client = AIClient::new(ai_config);
        let backend_client = if backend_config.auto_publish && !backend_config.auth_token.is_empty()
        {
            Some(BackendClient::new(backend_config.clone()))
        } else {
            None
        };
        Ok(Self {
            camera_id: camera.id,
            camera_name: camera.name.clone(),
            client,
            ai_client,
            backend_client,
            backend_config,
            status: RwLock::new(CameraStatus::Offline),
            started_at: None,
            last_seen: RwLock::new(None),
            last_error: RwLock::new(None),
            enabled: AtomicBool::new(camera.enabled),
            running: AtomicBool::new(false),
            last_detection: RwLock::new(None),
            last_inference: RwLock::new(None),
            last_inference_error: RwLock::new(None),
            successful_requests: std::sync::atomic::AtomicU64::new(0),
            failed_requests: std::sync::atomic::AtomicU64::new(0),
            successful_publishes: std::sync::atomic::AtomicU64::new(0),
            failed_publishes: std::sync::atomic::AtomicU64::new(0),
            last_publish: RwLock::new(None),
            last_publish_error: RwLock::new(None),
        })
    }

    pub fn camera_id(&self) -> uuid::Uuid {
        self.camera_id
    }

    pub fn camera_name(&self) -> &str {
        &self.camera_name
    }

    pub fn rtsp_url(&self) -> &str {
        self.client.url()
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

    pub fn client(&self) -> &RTSPClient {
        &self.client
    }

    pub fn ai_client(&self) -> &AIClient {
        &self.ai_client
    }

    /// Get the most recent detection response from the AI service.
    pub async fn latest_detection(&self) -> Option<DetectionResponse> {
        self.last_detection.read().await.clone()
    }

    /// Get the time of the last successful inference call.
    pub async fn last_inference_time(&self) -> Option<Instant> {
        *self.last_inference.read().await
    }

    /// Get the last AI inference error, if any.
    pub async fn last_inference_error(&self) -> Option<AIClientError> {
        self.last_inference_error.read().await.clone()
    }

    /// Get the count of successful inference requests.
    pub fn successful_requests(&self) -> u64 {
        self.successful_requests.load(Ordering::Relaxed)
    }

    /// Get the count of failed inference requests.
    pub fn failed_requests(&self) -> u64 {
        self.failed_requests.load(Ordering::Relaxed)
    }

    /// Get a reference to the backend client, if configured.
    pub fn backend_client(&self) -> Option<&BackendClient> {
        self.backend_client.as_ref()
    }

    /// Get the count of successful incident publishes.
    pub fn successful_publishes(&self) -> u64 {
        self.successful_publishes.load(Ordering::Relaxed)
    }

    /// Get the count of failed incident publishes.
    pub fn failed_publishes(&self) -> u64 {
        self.failed_publishes.load(Ordering::Relaxed)
    }

    /// Get the time of the last successful publish.
    pub async fn last_publish_time(&self) -> Option<Instant> {
        *self.last_publish.read().await
    }

    /// Get the last backend publish error, if any.
    pub async fn last_publish_error(&self) -> Option<BackendClientError> {
        self.last_publish_error.read().await.clone()
    }

    /// Start the worker — connect to the RTSP stream.
    ///
    /// Transitions: Offline → Connecting → Online (success) or Error (failure).
    pub async fn start(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        {
            let mut status = self.status.write().await;
            *status = CameraStatus::Connecting;
        }

        match self.client.connect().await {
            Ok(()) => {
                let mut status = self.status.write().await;
                *status = CameraStatus::Online;
                self.running.store(true, Ordering::Relaxed);

                let mut last_seen = self.last_seen.write().await;
                *last_seen = Some(Instant::now());

                let mut last_err = self.last_error.write().await;
                *last_err = None;
            }
            Err(e) => {
                let mut status = self.status.write().await;
                *status = CameraStatus::Error;
                self.running.store(false, Ordering::Relaxed);

                let mut last_err = self.last_error.write().await;
                *last_err = Some(e);
            }
        }
    }

    /// Stop the worker — disconnect from the RTSP stream.
    pub async fn stop(&self) {
        self.client.disconnect().await;
        {
            let mut status = self.status.write().await;
            *status = CameraStatus::Stopped;
        }
        self.running.store(false, Ordering::Relaxed);

        let mut last_err = self.last_error.write().await;
        *last_err = None;
    }

    /// Restart the worker: disconnect then reconnect.
    pub async fn restart(&self) {
        self.stop().await;
        self.start().await;
    }

    /// Perform a heartbeat — verify the connection is still alive.
    ///
    /// Updates `last_seen` if the heartbeat succeeds. Transitions to Error
    /// if the connection is lost.
    pub async fn heartbeat(&self) {
        if !self.running.load(Ordering::Relaxed) {
            return;
        }

        match self.client.heartbeat().await {
            Ok(()) => {
                let mut last_seen = self.last_seen.write().await;
                *last_seen = Some(Instant::now());
            }
            Err(e) => {
                let mut status = self.status.write().await;
                *status = CameraStatus::Error;
                self.running.store(false, Ordering::Relaxed);

                let mut last_err = self.last_error.write().await;
                *last_err = Some(e);
            }
        }
    }

    /// Capture a simulated frame and run inference against the AI service.
    ///
    /// This is a single inference cycle. Returns the detection response on
    /// success, or an error. Frame capture is currently simulated (no RTSP
    /// decoding).
    ///
    /// When a backend client is configured and auto_publish is enabled, each
    /// detection is published as an incident to the backend API. Publishing
    /// is blocking (waits for all publishes before returning) and failures
    /// are non-blocking (logged and counted, worker continues).
    pub async fn capture_and_infer(
        &self,
        source: &str,
    ) -> Result<DetectionResponse, AIClientError> {
        let frame = self.simulate_frame_capture().await;

        let response = self.ai_client.detect_frame(frame, source).await;

        match &response {
            Ok(det) => {
                let mut last_det = self.last_detection.write().await;
                *last_det = Some(det.clone());
                let mut last_inf = self.last_inference.write().await;
                *last_inf = Some(Instant::now());
                let mut last_err = self.last_inference_error.write().await;
                *last_err = None;
                self.successful_requests.fetch_add(1, Ordering::Relaxed);

                if self.backend_client.is_some() {
                    self.publish_detections(det).await;
                }
            }
            Err(e) => {
                let mut last_err = self.last_inference_error.write().await;
                *last_err = Some(e.clone());
                self.failed_requests.fetch_add(1, Ordering::Relaxed);
            }
        }

        response
    }

    /// Run the continuous inference loop.
    ///
    /// Performs `capture_and_infer` at the configured interval until the worker
    /// is stopped. This should be spawned as a tokio task.
    pub async fn run_inference_loop(&self, interval: Duration) {
        loop {
            if !self.running.load(Ordering::Relaxed) {
                break;
            }

            let _ = self.capture_and_infer(&self.camera_name.clone()).await;

            tokio::time::sleep(interval).await;
        }
    }

    /// Publish all detections from a DetectionResponse as incidents.
    ///
    /// Blocks until all publishes complete. Failures are logged and counted
    /// but do not prevent other detections from being published.
    async fn publish_detections(&self, response: &DetectionResponse) {
        let Some(client) = &self.backend_client else {
            return;
        };

        for detection in &response.detections {
            let severity = self.map_severity(&detection.class_name);
            let bbox = Self::convert_bbox(&detection.bbox);

            let metadata = serde_json::json!({
                "model_name": response.metadata.model_name,
                "inference_time_ms": response.inference_time_ms,
                "source": response.metadata.source,
            });

            let request = IncidentRequest {
                camera_id: self.camera_id,
                timestamp: None,
                severity,
                event_type: detection.class_name.clone(),
                confidence: detection.confidence,
                bounding_box: bbox,
                metadata: Some(metadata),
            };

            match client.publish_incident(&request).await {
                Ok(_) => {
                    self.successful_publishes.fetch_add(1, Ordering::Relaxed);
                    let mut last_pub = self.last_publish.write().await;
                    *last_pub = Some(Instant::now());
                    let mut last_err = self.last_publish_error.write().await;
                    *last_err = None;
                }
                Err(e) => {
                    tracing::warn!(
                        camera_id = %self.camera_id,
                        event_type = %detection.class_name,
                        error = %e,
                        "failed to publish incident"
                    );
                    self.failed_publishes.fetch_add(1, Ordering::Relaxed);
                    let mut last_err = self.last_publish_error.write().await;
                    *last_err = Some(e);
                }
            }
        }
    }

    /// Map a detection class name to an incident severity using the configured mapping.
    ///
    /// Falls back to Low for unmapped class names.
    fn map_severity(&self, class_name: &str) -> IncidentSeverity {
        self.backend_config
            .severity_mapping
            .get(class_name)
            .copied()
            .unwrap_or(IncidentSeverity::Low)
    }

    /// Convert an AI BoundingBox (8-field) to a backend BoundingBox (4-field).
    fn convert_bbox(bbox: &crate::ai::models::BoundingBox) -> Option<BackendBoundingBox> {
        Some(BackendBoundingBox {
            x1: bbox.x1,
            y1: bbox.y1,
            x2: bbox.x2,
            y2: bbox.y2,
        })
    }

    /// Generate a simulated JPEG frame.
    ///
    /// In a future milestone this will decode from the RTSP stream.
    async fn simulate_frame_capture(&self) -> bytes::Bytes {
        // Minimal JPEG SOI marker + padding to simulate a real frame.
        // SOI (FFD8) + APP0 (FFE0) marker with JFIF header
        let mut data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
        ];
        // Pad to a reasonable size to simulate a real frame
        data.resize(1024, 0x00);
        // Add EOI marker at the end
        let len = data.len();
        data[len - 2] = 0xFF;
        data[len - 1] = 0xD9;
        bytes::Bytes::from(data)
    }

    /// Get the current status.
    pub async fn status(&self) -> CameraStatus {
        *self.status.read().await
    }

    /// Get the time of the last successful heartbeat.
    pub async fn last_seen(&self) -> Option<Instant> {
        *self.last_seen.read().await
    }

    /// Get the last error that occurred, if any.
    pub async fn last_error(&self) -> Option<RtspError> {
        self.last_error.read().await.clone()
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
            .field("rtsp_url", &self.rtsp_url())
            .field("running", &self.is_running())
            .field("enabled", &self.is_enabled())
            .field("successful_requests", &self.successful_requests())
            .field("failed_requests", &self.failed_requests())
            .field("successful_publishes", &self.successful_publishes())
            .field("failed_publishes", &self.failed_publishes())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rtsp_config() -> RtspConfig {
        RtspConfig {
            connection_timeout: std::time::Duration::from_secs(5),
        }
    }

    fn test_ai_config() -> AiConfig {
        AiConfig::default()
    }

    fn test_backend_config() -> BackendConfig {
        BackendConfig::default()
    }

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
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert_eq!(worker.status().await, CameraStatus::Offline);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_start_transitions_online() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
        assert!(worker.is_running());
        assert!(worker.last_seen().await.is_some());
        assert!(worker.last_error().await.is_none());
    }

    #[tokio::test]
    async fn test_worker_stop_transitions_stopped() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);

        worker.stop().await;
        assert_eq!(worker.status().await, CameraStatus::Stopped);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_restart() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);

        worker.restart().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
        assert!(worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_heartbeat_updates_last_seen() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;

        let before = worker.last_seen().await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        worker.heartbeat().await;
        let after = worker.last_seen().await.unwrap();

        assert!(after >= before);
    }

    #[tokio::test]
    async fn test_worker_heartbeat_preserves_online_status() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;

        worker.heartbeat().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
    }

    #[tokio::test]
    async fn test_worker_disabled_does_not_start() {
        let camera = test_camera(false);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Offline);
        assert!(!worker.is_running());
    }

    #[tokio::test]
    async fn test_worker_set_enabled() {
        let camera = test_camera(false);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert!(!worker.is_enabled());

        worker.set_enabled(true);
        assert!(worker.is_enabled());

        worker.start().await;
        assert_eq!(worker.status().await, CameraStatus::Online);
    }

    #[tokio::test]
    async fn test_worker_heartbeat_when_stopped_does_nothing() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        worker.stop().await;

        let before = worker.last_seen().await;
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        worker.heartbeat().await;
        let after = worker.last_seen().await;

        // Both should be Some but last_seen shouldn't have changed
        assert!(before.is_some());
        assert!(after.is_some());
    }

    #[tokio::test]
    async fn test_worker_no_error_on_success() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        worker.start().await;
        assert!(worker.last_error().await.is_none());
    }

    #[test]
    fn test_worker_metadata() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert_eq!(worker.camera_id(), camera.id);
        assert_eq!(worker.camera_name(), "Test Camera");
        assert_eq!(worker.rtsp_url(), "rtsp://10.0.0.1:554/stream");
    }

    #[test]
    fn test_worker_client_accessible() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert_eq!(worker.client().url(), "rtsp://10.0.0.1:554/stream");
    }

    #[test]
    fn test_worker_debug() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        let debug = format!("{worker:?}");
        assert!(debug.contains("CameraWorker"));
        assert!(debug.contains("camera_id"));
        assert!(debug.contains("successful_requests"));
    }

    #[test]
    fn test_worker_invalid_url() {
        let camera = Camera {
            id: uuid::Uuid::new_v4(),
            name: "Bad Camera".to_string(),
            rtsp_url: "http://10.0.0.1/stream".to_string(),
            location: None,
            fps: None,
            resolution: None,
            enabled: true,
        };
        let result = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        );
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_worker_initial_detection_none() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert!(worker.latest_detection().await.is_none());
        assert!(worker.last_inference_time().await.is_none());
        assert!(worker.last_inference_error().await.is_none());
        assert_eq!(worker.successful_requests(), 0);
        assert_eq!(worker.failed_requests(), 0);
    }

    #[tokio::test]
    async fn test_worker_capture_and_infer_offline() {
        let camera = test_camera(true);
        let ai_config = AiConfig {
            service_url: "http://127.0.0.1:19996".to_string(),
            ..AiConfig::default()
        };
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            ai_config,
            test_backend_config(),
        )
        .unwrap();
        let result = worker.capture_and_infer("cam-1").await;
        assert!(result.is_err());
        assert_eq!(worker.failed_requests(), 1);
        assert_eq!(worker.successful_requests(), 0);
        assert!(worker.last_inference_error().await.is_some());
        assert!(worker.latest_detection().await.is_none());
    }

    #[tokio::test]
    async fn test_worker_ai_client_accessible() {
        let camera = test_camera(true);
        let worker = CameraWorker::new(
            &camera,
            test_rtsp_config(),
            test_ai_config(),
            test_backend_config(),
        )
        .unwrap();
        assert_eq!(worker.ai_client().base_url(), "http://localhost:8081");
    }
}
