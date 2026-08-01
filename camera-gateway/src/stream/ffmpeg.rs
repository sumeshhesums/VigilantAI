use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Bytes;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};
use tokio::process::{Child, ChildStdout, Command};
use tokio::sync::{Mutex, RwLock};

use crate::config::RtspConfig;
use crate::stream::client::RtspError;

/// SOI and EOI markers that delimit individual JPEG frames in an MJPEG stream.
const SOI: &[u8; 2] = b"\xFF\xD8";
const EOI: &[u8; 2] = b"\xFF\xD9";

/// Maximum number of bytes buffered while scanning for a complete frame.
const MAX_BUFFER_SIZE: usize = 20 * 1024 * 1024;

/// Number of bytes read from the pipe per chunk.
const READ_CHUNK_SIZE: usize = 16 * 1024;

/// A period after which the ffmpeg pipe is considered stalled if no frame
/// has been parsed.
const STALL_TIMEOUT: Duration = Duration::from_secs(30);

/// The RTSP socket-I/O timeout flag supported by the installed ffmpeg.
///
/// ffmpeg renamed `-stimeout` to `-timeout` and eventually dropped the old
/// name entirely, so the supported flag depends on the installed version.
fn rtsp_timeout_flag() -> &'static str {
    static FLAG: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();
    FLAG.get_or_init(|| {
        let probe = std::process::Command::new("ffmpeg")
            .args(["-hide_banner", "-h", "demuxer=rtsp"])
            .output();
        match probe {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout);
                if text.contains("-timeout") {
                    "-timeout"
                } else {
                    "-stimeout"
                }
            }
            Err(_) => "-timeout",
        }
    })
}

/// Find the first occurrence of a two-byte marker at or after `start`.
fn find_marker(data: &[u8], marker: &[u8; 2], start: usize) -> Option<usize> {
    if start + 1 >= data.len() {
        return None;
    }
    let mut i = start;
    while i + 1 < data.len() {
        if data[i] == marker[0] && data[i + 1] == marker[1] {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Scan a buffer of raw MJPEG bytes for complete JPEG frames.
///
/// Returns:
/// - the newest complete frame (SOI..EOI inclusive), if any,
/// - the number of older complete frames that were superseded (dropped),
/// - the number of leading bytes that can be discarded.
///
/// A trailing partial frame (SOI with no EOI yet) is preserved in the buffer
/// for the next read.
fn scan_frames(data: &[u8]) -> (Option<Vec<u8>>, u64, usize) {
    let mut frame: Option<(usize, usize)> = None;
    let mut dropped = 0u64;
    let mut consumed = 0usize;

    let mut start = find_marker(data, SOI, 0);
    while let Some(s) = start {
        match find_marker(data, EOI, s + 2) {
            Some(e) => {
                if frame.is_some() {
                    dropped += 1;
                }
                frame = Some((s, e));
                consumed = e + 2;
                start = find_marker(data, SOI, e + 2);
            }
            None => break,
        }
    }

    match frame {
        Some((s, e)) => (Some(data[s..=e + 1].to_vec()), dropped, consumed),
        None => (None, dropped, consumed),
    }
}

/// A real frame reader backed by an `ffmpeg` subprocess.
///
/// ffmpeg performs the RTSP handshake and decodes the stream, re-encoding each
/// frame as JPEG and writing the concatenated MJPEG stream to stdout. This
/// reader splits stdout on SOI/EOI markers and hands out one frame at a time.
pub struct FfmpegFrameReader {
    url: String,
    config: RtspConfig,
    process: Arc<Mutex<Option<Child>>>,
    stdout: Arc<Mutex<Option<ChildStdout>>>,
    stderr_task: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    parse_buffer: std::sync::Mutex<Vec<u8>>,
    connected: Arc<AtomicU64>,
    bytes_received: AtomicU64,
    frames_dropped: AtomicU64,
    decode_errors: AtomicU64,
    last_frame_at: RwLock<Option<Instant>>,
    bitrate_window_bytes: AtomicU64,
    bitrate_window_start: RwLock<Instant>,
    current_bitrate: RwLock<f64>,
}

impl FfmpegFrameReader {
    pub fn new(url: String, config: RtspConfig) -> Self {
        Self {
            url,
            config,
            process: Arc::new(Mutex::new(None)),
            stdout: Arc::new(Mutex::new(None)),
            stderr_task: Arc::new(Mutex::new(None)),
            parse_buffer: std::sync::Mutex::new(Vec::new()),
            connected: Arc::new(AtomicU64::new(0)),
            bytes_received: AtomicU64::new(0),
            frames_dropped: AtomicU64::new(0),
            decode_errors: AtomicU64::new(0),
            last_frame_at: RwLock::new(None),
            bitrate_window_bytes: AtomicU64::new(0),
            bitrate_window_start: RwLock::new(Instant::now()),
            current_bitrate: RwLock::new(0.0),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Acquire) == 1
    }

    pub fn bytes_received(&self) -> u64 {
        self.bytes_received.load(Ordering::Relaxed)
    }

    pub fn frames_dropped(&self) -> u64 {
        self.frames_dropped.load(Ordering::Relaxed)
    }

    pub fn decode_errors(&self) -> u64 {
        self.decode_errors.load(Ordering::Relaxed)
    }

    pub async fn last_frame_at(&self) -> Option<Instant> {
        *self.last_frame_at.read().await
    }

    pub async fn bitrate_bps(&self) -> u64 {
        (*self.current_bitrate.read().await * 8.0) as u64
    }

    /// Spawn the ffmpeg subprocess and wait for the first frame to verify the
    /// connection actually works. Returns an error if the subprocess cannot be
    /// spawned or no frame arrives within the configured connection timeout.
    pub async fn connect(&self) -> Result<(), RtspError> {
        if self.is_connected() {
            return Ok(());
        }

        self.cleanup().await;

        let timeout_micros = self.config.connection_timeout.as_micros();

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-hide_banner")
            .arg("-loglevel")
            .arg("warning")
            .arg("-nostdin")
            .arg("-rtsp_transport")
            .arg("tcp")
            .arg(rtsp_timeout_flag())
            .arg(timeout_micros.to_string())
            .arg("-fflags")
            .arg("nobuffer")
            .arg("-i")
            .arg(&self.url)
            .arg("-an")
            .arg("-f")
            .arg("image2pipe")
            .arg("-vcodec")
            .arg("mjpeg")
            .arg("-q:v")
            .arg("3")
            .arg("pipe:1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(e) => {
                let detail = if e.kind() == std::io::ErrorKind::NotFound {
                    "ffmpeg binary not found; install ffmpeg or set GATEWAY_RTSP_SIMULATED=1".to_string()
                } else {
                    e.to_string()
                };
                self.decode_errors.fetch_add(1, Ordering::Relaxed);
                return Err(RtspError::SpawnFailed {
                    url: self.url.clone(),
                    detail,
                });
            }
        };

        let stdout = child.stdout.take().ok_or_else(|| RtspError::SpawnFailed {
            url: self.url.clone(),
            detail: "ffmpeg stdout was not piped".to_string(),
        })?;
        let stderr = child.stderr.take().ok_or_else(|| RtspError::SpawnFailed {
            url: self.url.clone(),
            detail: "ffmpeg stderr was not piped".to_string(),
        })?;

        let url_for_log = self.url.clone();
        let stderr_task = tokio::spawn(async move {
            let mut lines = tokio::io::BufReader::new(stderr).lines();
            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        tracing::debug!(url = %url_for_log, line, "ffmpeg stderr");
                    }
                    _ => break,
                }
            }
        });

        {
            let mut process = self.process.lock().await;
            *process = Some(child);
        }
        {
            let mut out = self.stdout.lock().await;
            *out = Some(stdout);
        }
        {
            let mut task = self.stderr_task.lock().await;
            *task = Some(stderr_task);
        }
        {
            let mut start = self.bitrate_window_start.write().await;
            *start = Instant::now();
        }
        self.bitrate_window_bytes.store(0, Ordering::Relaxed);

        match tokio::time::timeout(self.config.connection_timeout, self.next_frame()).await {
            Ok(Ok(_frame)) => {
                self.connected.store(1, Ordering::Release);
                {
                    let mut last = self.last_frame_at.write().await;
                    *last = Some(Instant::now());
                }
                Ok(())
            }
            Ok(Err(e)) => {
                self.cleanup().await;
                Err(e)
            }
            Err(_elapsed) => {
                self.decode_errors.fetch_add(1, Ordering::Relaxed);
                let err = RtspError::ConnectionTimeout {
                    url: self.url.clone(),
                    timeout_ms: self.config.connection_timeout.as_millis() as u64,
                };
                self.cleanup().await;
                Err(err)
            }
        }
    }

    /// Read the next complete JPEG frame from the ffmpeg pipe.
    pub async fn next_frame(&self) -> Result<Bytes, RtspError> {
        let mut buf = {
            let mut guard = self.parse_buffer.lock().expect("parse buffer poisoned");
            std::mem::take(&mut *guard)
        };

        loop {
            let (frame, dropped, consumed) = scan_frames(&buf);
            if let Some(frame) = frame {
                self.frames_dropped.fetch_add(dropped, Ordering::Relaxed);
                let leftover = buf.split_off(consumed);
                {
                    let mut guard = self.parse_buffer.lock().expect("parse buffer poisoned");
                    *guard = leftover;
                }
                {
                    let mut last = self.last_frame_at.write().await;
                    *last = Some(Instant::now());
                }
                return Ok(Bytes::from(frame));
            }

            if buf.len() > MAX_BUFFER_SIZE {
                self.decode_errors.fetch_add(1, Ordering::Relaxed);
                return Err(RtspError::DecodeFailed {
                    url: self.url.clone(),
                    detail: "no complete JPEG frame found within buffer limit".to_string(),
                });
            }

            let mut chunk = vec![0u8; READ_CHUNK_SIZE];
            let mut out = self.stdout.lock().await;
            let stdout = out.as_mut().ok_or_else(|| RtspError::DecodeFailed {
                url: self.url.clone(),
                detail: "ffmpeg stdout closed".to_string(),
            })?;

            let n = stdout.read(&mut chunk).await.map_err(|e| {
                self.decode_errors.fetch_add(1, Ordering::Relaxed);
                RtspError::DecodeFailed {
                    url: self.url.clone(),
                    detail: format!("failed to read ffmpeg output: {e}"),
                }
            })?;

            if n == 0 {
                self.decode_errors.fetch_add(1, Ordering::Relaxed);
                return Err(RtspError::Disconnected {
                    url: self.url.clone(),
                });
            }

            buf.extend_from_slice(&chunk[..n]);
            self.bytes_received.fetch_add(n as u64, Ordering::Relaxed);
            self.bitrate_window_bytes
                .fetch_add(n as u64, Ordering::Relaxed);
            self.update_bitrate().await;
        }
    }

    /// Check that the subprocess is still alive and that frames are flowing.
    pub async fn heartbeat(&self) -> Result<(), RtspError> {
        if !self.is_connected() {
            return Err(RtspError::Disconnected {
                url: self.url.clone(),
            });
        }

        let mut process = self.process.lock().await;
        if let Some(child) = process.as_mut() {
            match child.try_wait() {
                Ok(Some(_status)) => {
                    drop(process);
                    self.connected.store(0, Ordering::Release);
                    return Err(RtspError::Disconnected {
                        url: self.url.clone(),
                    });
                }
                Ok(None) => {}
                Err(e) => {
                    drop(process);
                    self.connected.store(0, Ordering::Release);
                    return Err(RtspError::DecodeFailed {
                        url: self.url.clone(),
                        detail: format!("failed to poll ffmpeg: {e}"),
                    });
                }
            }
        }
        drop(process);

        if let Some(last) = *self.last_frame_at.read().await {
            if last.elapsed() > STALL_TIMEOUT {
                self.connected.store(0, Ordering::Release);
                return Err(RtspError::Disconnected {
                    url: self.url.clone(),
                });
            }
        }

        Ok(())
    }

    /// Kill the subprocess, abort the stderr drain task, and reset all state.
    pub async fn cleanup(&self) {
        {
            let mut process = self.process.lock().await;
            if let Some(child) = process.as_mut() {
                let _ = child.kill().await;
                let _ = child.wait().await;
            }
            *process = None;
        }
        {
            let mut out = self.stdout.lock().await;
            *out = None;
        }
        {
            let mut task = self.stderr_task.lock().await;
            if let Some(handle) = task.take() {
                handle.abort();
            }
        }
        {
            let mut guard = self.parse_buffer.lock().expect("parse buffer poisoned");
            *guard = Vec::new();
        }
        self.connected.store(0, Ordering::Release);
        {
            let mut last = self.last_frame_at.write().await;
            *last = None;
        }
    }

    async fn update_bitrate(&self) {
        let now = Instant::now();
        let mut start = self.bitrate_window_start.write().await;
        let elapsed = now.duration_since(*start);
        if elapsed >= Duration::from_secs(2) {
            let bytes = self.bitrate_window_bytes.swap(0, Ordering::Relaxed);
            let mut rate = self.current_bitrate.write().await;
            *rate = bytes as f64 / elapsed.as_secs_f64();
            *start = now;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(n: u8) -> Vec<u8> {
        vec![0xFF, 0xD8, n, 0x00, 0xFF, 0xD9]
    }

    #[test]
    fn test_scan_empty_buffer() {
        let (frame, dropped, consumed) = scan_frames(&[]);
        assert!(frame.is_none());
        assert_eq!(dropped, 0);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn test_scan_single_frame() {
        let data = frame(1);
        let (parsed, dropped, consumed) = scan_frames(&data);
        let parsed = parsed.unwrap();
        assert_eq!(parsed, data);
        assert_eq!(dropped, 0);
        assert_eq!(consumed, data.len());
    }

    #[test]
    fn test_scan_two_frames_keeps_newest() {
        let mut data = frame(1);
        data.extend_from_slice(&frame(2));
        let (parsed, dropped, consumed) = scan_frames(&data);
        let parsed = parsed.unwrap();
        assert_eq!(parsed, frame(2));
        assert_eq!(dropped, 1);
        assert_eq!(consumed, data.len());
    }

    #[test]
    fn test_scan_partial_trailing_frame_is_preserved() {
        let mut data = frame(1);
        data.extend_from_slice(&[0xFF, 0xD8, 0xAA]);
        let (parsed, dropped, consumed) = scan_frames(&data);
        let parsed = parsed.unwrap();
        assert_eq!(parsed, frame(1));
        assert_eq!(dropped, 0);
        // Consumes only the complete frame; the partial tail is preserved.
        assert_eq!(consumed, 6);
    }

    #[test]
    fn test_scan_only_partial_frame() {
        let data = vec![0xFF, 0xD8, 0xAA];
        let (parsed, dropped, consumed) = scan_frames(&data);
        assert!(parsed.is_none());
        assert_eq!(dropped, 0);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn test_scan_garbage_prefix_is_consumed() {
        let mut data = vec![0x00, 0x01, 0x02];
        data.extend_from_slice(&frame(1));
        let (parsed, _dropped, consumed) = scan_frames(&data);
        let parsed = parsed.unwrap();
        assert_eq!(parsed, frame(1));
        assert_eq!(consumed, data.len());
    }

    fn ffmpeg_available() -> bool {
        std::process::Command::new("ffmpeg")
            .arg("-version")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    #[tokio::test]
    async fn test_ffmpeg_pipe_yields_mjpeg_frames() {
        if !ffmpeg_available() {
            return;
        }

        let mut child = Command::new("ffmpeg")
            .args([
                "-hide_banner",
                "-loglevel",
                "error",
                "-f",
                "lavfi",
                "-i",
                "testsrc2=duration=1:size=160x120:rate=5",
                "-an",
                "-f",
                "image2pipe",
                "-vcodec",
                "mjpeg",
                "pipe:1",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn ffmpeg");

        let mut out = child.stdout.take().unwrap();
        let mut buf = Vec::new();
        let mut frames = 0u32;

        for _ in 0..64 {
            if frames >= 2 {
                break;
            }
            let mut chunk = [0u8; 8192];
            let n = out.read(&mut chunk).await.unwrap();
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
            loop {
                match scan_frames(&buf) {
                    (Some(_), _, consumed) => {
                        buf.drain(..consumed);
                        frames += 1;
                    }
                    (None, _, _) => break,
                }
                if frames >= 2 {
                    break;
                }
            }
        }

        let _ = child.kill().await;
        let _ = child.wait().await;

        assert!(frames >= 2, "expected >=2 frames, got {frames}");
    }
}
