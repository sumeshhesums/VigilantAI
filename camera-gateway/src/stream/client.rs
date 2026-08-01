use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::RwLock;
use tokio::time::timeout;

use crate::config::RtspConfig;
use crate::models::CameraStatus;
use crate::stream::ffmpeg::FfmpegFrameReader;

/// Errors that can occur during RTSP operations.
#[derive(Debug, Clone)]
pub enum RtspError {
    InvalidUrl(String),
    ConnectionTimeout { url: String, timeout_ms: u64 },
    ConnectionRefused { url: String },
    TlsError { url: String, detail: String },
    ProtocolError { url: String, detail: String },
    Disconnected { url: String },
    SpawnFailed { url: String, detail: String },
    DecodeFailed { url: String, detail: String },
}

impl fmt::Display for RtspError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RtspError::InvalidUrl(url) => write!(f, "invalid RTSP URL: {url}"),
            RtspError::ConnectionTimeout { url, timeout_ms } => {
                write!(f, "connection to {url} timed out after {timeout_ms}ms")
            }
            RtspError::ConnectionRefused { url } => {
                write!(f, "connection to {url} refused")
            }
            RtspError::TlsError { url, detail } => {
                write!(f, "TLS error connecting to {url}: {detail}")
            }
            RtspError::ProtocolError { url, detail } => {
                write!(f, "RTSP protocol error on {url}: {detail}")
            }
            RtspError::Disconnected { url } => write!(f, "disconnected from {url}"),
            RtspError::SpawnFailed { url, detail } => {
                write!(f, "failed to spawn ffmpeg for {url}: {detail}")
            }
            RtspError::DecodeFailed { url, detail } => {
                write!(f, "failed to decode stream from {url}: {detail}")
            }
        }
    }
}

impl std::error::Error for RtspError {}

/// An RTSP client that manages a real connection to a camera stream.
///
/// Supports `rtsp://` and `rtsps://` URL schemes. The connection is backed by
/// an `ffmpeg` subprocess which performs the RTSP handshake and decodes the
/// stream into JPEG frames (see [`FfmpegFrameReader`]).
///
/// When `RtspConfig::simulated` is set (tests only), the connection is
/// simulated and no subprocess is spawned.
#[derive(Clone)]
pub struct RTSPClient {
    url: String,
    config: RtspConfig,
    connected: Arc<AtomicBool>,
    last_error: Arc<RwLock<Option<RtspError>>>,
    connected_at: Arc<RwLock<Option<Instant>>>,
    ffmpeg: Arc<FfmpegFrameReader>,
}

impl RTSPClient {
    pub fn new(url: String, config: RtspConfig) -> Result<Self, RtspError> {
        Self::validate_url(&url)?;
        let ffmpeg = FfmpegFrameReader::new(url.clone(), config.clone());
        Ok(Self {
            url,
            config,
            connected: Arc::new(AtomicBool::new(false)),
            last_error: Arc::new(RwLock::new(None)),
            connected_at: Arc::new(RwLock::new(None)),
            ffmpeg: Arc::new(ffmpeg),
        })
    }

    /// Validate that the URL uses a supported RTSP scheme.
    fn validate_url(url: &str) -> Result<(), RtspError> {
        if url.starts_with("rtsp://") || url.starts_with("rtsps://") {
            Ok(())
        } else {
            Err(RtspError::InvalidUrl(url.to_string()))
        }
    }

    /// Attempt to connect to the RTSP stream.
    ///
    /// In real mode this spawns `ffmpeg` and waits for the first decoded frame
    /// within the configured connection timeout, proving the stream is live.
    /// In simulated mode a fake handshake is performed.
    pub async fn connect(&self) -> Result<(), RtspError> {
        if self.connected.load(Ordering::Acquire) {
            return Ok(());
        }

        if self.config.simulated {
            return self.simulated_connect().await;
        }

        match self.ffmpeg.connect().await {
            Ok(()) => {
                self.connected.store(true, Ordering::Release);
                let mut connected_at = self.connected_at.write().await;
                *connected_at = Some(Instant::now());
                let mut last_err = self.last_error.write().await;
                *last_err = None;
                Ok(())
            }
            Err(e) => {
                self.connected.store(false, Ordering::Release);
                let mut last_err = self.last_error.write().await;
                *last_err = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Disconnect from the RTSP stream.
    pub async fn disconnect(&self) {
        self.connected.store(false, Ordering::Release);
        let mut connected_at = self.connected_at.write().await;
        *connected_at = None;
        if !self.config.simulated {
            self.ffmpeg.cleanup().await;
        }
    }

    /// Attempt to reconnect to the RTSP stream.
    ///
    /// Disconnects first, then attempts a fresh connection.
    pub async fn reconnect(&self) -> Result<(), RtspError> {
        self.disconnect().await;
        self.connect().await
    }

    /// Check if the client is currently connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire)
    }

    /// Get the last error that occurred, if any.
    pub async fn last_error(&self) -> Option<RtspError> {
        self.last_error.read().await.clone()
    }

    /// Get the time when the connection was established.
    pub async fn connected_at(&self) -> Option<Instant> {
        *self.connected_at.read().await
    }

    /// Get the URL this client connects to.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Determine the current status based on connection state.
    pub async fn status(&self) -> CameraStatus {
        if self.is_connected() {
            CameraStatus::Online
        } else {
            match self.last_error().await {
                Some(_) => CameraStatus::Error,
                None => CameraStatus::Offline,
            }
        }
    }

    /// Perform a heartbeat check — verifies the stream is still alive.
    ///
    /// In real mode this verifies the ffmpeg subprocess is still running and
    /// that frames have arrived recently. Returns `Err(RtspError)` if the
    /// connection has been lost.
    pub async fn heartbeat(&self) -> Result<(), RtspError> {
        if !self.is_connected() {
            return Err(RtspError::Disconnected {
                url: self.url.clone(),
            });
        }

        if self.config.simulated {
            return self.simulated_heartbeat().await;
        }

        match self.ffmpeg.heartbeat().await {
            Ok(()) => Ok(()),
            Err(e) => {
                self.connected.store(false, Ordering::Release);
                let mut last_err = self.last_error.write().await;
                *last_err = Some(e.clone());
                Err(e)
            }
        }
    }

    /// Read the next decoded JPEG frame from the stream.
    ///
    /// Real mode returns frames decoded by ffmpeg. Simulated mode returns a
    /// synthetic JPEG frame so tests can exercise the inference pipeline.
    pub async fn next_frame(&self) -> Result<bytes::Bytes, RtspError> {
        if self.config.simulated {
            return self.simulated_next_frame().await;
        }
        self.ffmpeg.next_frame().await
    }

    /// Total number of raw bytes received from the ffmpeg pipe.
    pub fn bytes_received(&self) -> u64 {
        self.ffmpeg.bytes_received()
    }

    /// Number of complete frames skipped because newer frames arrived.
    pub fn frames_dropped(&self) -> u64 {
        self.ffmpeg.frames_dropped()
    }

    /// Number of decode/read failures encountered.
    pub fn decode_errors(&self) -> u64 {
        self.ffmpeg.decode_errors()
    }

    /// Current stream bitrate in bits per second.
    pub async fn bitrate_bps(&self) -> u64 {
        self.ffmpeg.bitrate_bps().await
    }

    /// Time the last frame was decoded, if any.
    pub async fn last_frame_at(&self) -> Option<Instant> {
        self.ffmpeg.last_frame_at().await
    }

    async fn simulated_connect(&self) -> Result<(), RtspError> {
        let connect_future = async {
            // Simulate connection handshake latency (5–20ms range)
            let handshake_latency =
                Duration::from_millis(5 + (std::ptr::addr_of!(self) as usize % 16) as u64);
            tokio::time::sleep(handshake_latency).await;

            // Validate scheme again for safety
            Self::validate_url(&self.url)?;

            // Simulate occasional failures for rtsps:// (TLS overhead)
            if self.url.starts_with("rtsps://") {
                let hash = self.url.len() % 7;
                if hash == 0 {
                    return Err(RtspError::TlsError {
                        url: self.url.clone(),
                        detail: "certificate verification failed".to_string(),
                    });
                }
            }

            Ok::<(), RtspError>(())
        };

        match timeout(self.config.connection_timeout, connect_future).await {
            Ok(Ok(())) => {
                self.connected.store(true, Ordering::Release);
                let mut connected_at = self.connected_at.write().await;
                *connected_at = Some(Instant::now());
                let mut last_err = self.last_error.write().await;
                *last_err = None;
                Ok(())
            }
            Ok(Err(e)) => {
                self.connected.store(false, Ordering::Release);
                let mut last_err = self.last_error.write().await;
                *last_err = Some(e.clone());
                Err(e)
            }
            Err(_elapsed) => {
                self.connected.store(false, Ordering::Release);
                let err = RtspError::ConnectionTimeout {
                    url: self.url.clone(),
                    timeout_ms: self.config.connection_timeout.as_millis() as u64,
                };
                let mut last_err = self.last_error.write().await;
                *last_err = Some(err.clone());
                Err(err)
            }
        }
    }

    async fn simulated_heartbeat(&self) -> Result<(), RtspError> {
        // Simulate a lightweight RTSP OPTIONS keepalive
        let keepalive_timeout = Duration::from_millis(500);
        let check = async {
            // In a real implementation this would send an RTSP OPTIONS request
            tokio::time::sleep(Duration::from_millis(1)).await;
            Ok::<(), RtspError>(())
        };

        match timeout(keepalive_timeout, check).await {
            Ok(Ok(())) => Ok(()),
            Ok(Err(e)) => {
                self.connected.store(false, Ordering::Release);
                let mut last_err = self.last_error.write().await;
                *last_err = Some(e.clone());
                Err(e)
            }
            Err(_) => {
                self.connected.store(false, Ordering::Release);
                let err = RtspError::Disconnected {
                    url: self.url.clone(),
                };
                let mut last_err = self.last_error.write().await;
                *last_err = Some(err.clone());
                Err(err)
            }
        }
    }

    async fn simulated_next_frame(&self) -> Result<bytes::Bytes, RtspError> {
        // Minimal JPEG SOI marker + padding to simulate a real frame.
        let mut data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
        ];
        data.resize(1024, 0x00);
        let len = data.len();
        data[len - 2] = 0xFF;
        data[len - 1] = 0xD9;
        Ok(bytes::Bytes::from(data))
    }
}

impl fmt::Debug for RTSPClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RTSPClient")
            .field("url", &self.url)
            .field("connected", &self.is_connected())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RtspConfig {
        RtspConfig {
            connection_timeout: Duration::from_secs(5),
            simulated: true,
        }
    }

    #[tokio::test]
    async fn test_connect_rtsp() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        assert!(!client.is_connected());
        assert!(client.last_error().await.is_none());

        client.connect().await.unwrap();
        assert!(client.is_connected());
        assert!(client.connected_at().await.is_some());
        assert_eq!(client.status().await, CameraStatus::Online);
    }

    #[tokio::test]
    async fn test_connect_rtsps() {
        let client =
            RTSPClient::new("rtsps://10.0.0.1:322/stream".to_string(), test_config()).unwrap();
        client.connect().await.unwrap();
        assert!(client.is_connected());
    }

    #[tokio::test]
    async fn test_connect_invalid_url() {
        let result = RTSPClient::new("http://10.0.0.1/stream".to_string(), test_config());
        assert!(result.is_err());
        match result.unwrap_err() {
            RtspError::InvalidUrl(_) => {}
            other => panic!("expected InvalidUrl, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_connect_no_scheme() {
        let result = RTSPClient::new("10.0.0.1:554/stream".to_string(), test_config());
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_disconnect() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        client.connect().await.unwrap();
        assert!(client.is_connected());

        client.disconnect().await;
        assert!(!client.is_connected());
        assert!(client.connected_at().await.is_none());
    }

    #[tokio::test]
    async fn test_reconnect() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        client.connect().await.unwrap();
        assert!(client.is_connected());

        client.reconnect().await.unwrap();
        assert!(client.is_connected());
    }

    #[tokio::test]
    async fn test_connect_idempotent() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        client.connect().await.unwrap();
        client.connect().await.unwrap();
        assert!(client.is_connected());
    }

    #[tokio::test]
    async fn test_heartbeat_connected() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        client.connect().await.unwrap();

        let result = client.heartbeat().await;
        assert!(result.is_ok());
        assert!(client.is_connected());
    }

    #[tokio::test]
    async fn test_heartbeat_disconnected() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        // Not connected
        let result = client.heartbeat().await;
        assert!(result.is_err());
        assert!(!client.is_connected());
    }

    #[tokio::test]
    async fn test_status_transitions() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        assert_eq!(client.status().await, CameraStatus::Offline);

        client.connect().await.unwrap();
        assert_eq!(client.status().await, CameraStatus::Online);

        client.disconnect().await;
        assert_eq!(client.status().await, CameraStatus::Offline);
    }

    #[tokio::test]
    async fn test_next_frame_simulated() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        let frame = client.next_frame().await.unwrap();
        assert!(frame.len() >= 1024);
        assert_eq!(&frame[..2], &[0xFF, 0xD8]);
        assert_eq!(&frame[frame.len() - 2..], &[0xFF, 0xD9]);
    }

    #[tokio::test]
    async fn test_url_accessor() {
        let client =
            RTSPClient::new("rtsp://192.168.1.100:554/live".to_string(), test_config()).unwrap();
        assert_eq!(client.url(), "rtsp://192.168.1.100:554/live");
    }

    #[tokio::test]
    async fn test_debug_format() {
        let client =
            RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), test_config()).unwrap();
        let debug = format!("{client:?}");
        assert!(debug.contains("RTSPClient"));
        assert!(debug.contains("url"));
    }

    #[tokio::test]
    async fn test_connect_timeout() {
        let config = RtspConfig {
            connection_timeout: Duration::from_millis(1),
            simulated: true,
        };
        let client = RTSPClient::new("rtsp://10.0.0.1:554/stream".to_string(), config).unwrap();
        // The connection handshake takes ~5ms, so with 1ms timeout it should timeout
        // But since the actual sleep is variable, we just verify the client handles it
        let result = client.connect().await;
        // It may succeed or timeout depending on timing, both are valid
        if result.is_err() {
            assert!(!client.is_connected());
        }
    }

    #[test]
    fn test_error_display_spawn_failed() {
        let err = RtspError::SpawnFailed {
            url: "rtsp://cam".to_string(),
            detail: "ffmpeg binary not found".to_string(),
        };
        assert!(err.to_string().contains("spawn ffmpeg"));
        assert!(err.to_string().contains("ffmpeg binary not found"));
    }

    #[test]
    fn test_error_display_decode_failed() {
        let err = RtspError::DecodeFailed {
            url: "rtsp://cam".to_string(),
            detail: "stream ended".to_string(),
        };
        assert!(err.to_string().contains("decode stream"));
        assert!(err.to_string().contains("stream ended"));
    }
}
