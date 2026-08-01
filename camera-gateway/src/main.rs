use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Path, State as AxumState};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use prometheus::{Encoder, IntGauge, Registry, TextEncoder};
use serde::Serialize;
use tokio::net::TcpListener;
use tracing::info;

use camera_gateway::config::GatewayConfig;
use camera_gateway::gateway::manager::GatewayManager;
use camera_gateway::gateway::state::GatewayState;
use camera_gateway::models::{Camera, CameraStatus};
use camera_gateway::services::health::{GatewayHealth, HealthResponse};

#[derive(Clone)]
#[allow(dead_code)]
struct GatewayMetrics {
    registry: Registry,
    cameras_connected: IntGauge,
    cameras_online: IntGauge,
    cameras_offline: IntGauge,
    reconnect_attempts_total: IntGauge,
    frames_processed_total: IntGauge,
    frames_dropped_total: IntGauge,
    decode_errors_total: IntGauge,
    current_bitrate_bps: IntGauge,
    ai_requests_total: IntGauge,
    ai_failures_total: IntGauge,
    backend_publishes_total: IntGauge,
    backend_publish_failures_total: IntGauge,
}

impl GatewayMetrics {
    fn new() -> Self {
        let registry = Registry::new();
        let cameras_connected = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_cameras_connected",
            "Number of connected cameras"
        ))
        .unwrap();
        let cameras_online = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_cameras_online",
            "Number of online cameras"
        ))
        .unwrap();
        let cameras_offline = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_cameras_offline",
            "Number of offline cameras"
        ))
        .unwrap();
        let reconnect_attempts_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_reconnect_attempts_total",
            "Total reconnect attempts"
        ))
        .unwrap();
        let frames_processed_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_frames_processed_total",
            "Total frames processed"
        ))
        .unwrap();
        let frames_dropped_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_frames_dropped_total",
            "Total frames dropped because newer frames arrived"
        ))
        .unwrap();
        let decode_errors_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_decode_errors_total",
            "Total stream decode errors"
        ))
        .unwrap();
        let current_bitrate_bps = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_current_bitrate_bps",
            "Aggregate current stream bitrate in bits per second"
        ))
        .unwrap();
        let ai_requests_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_ai_requests_total",
            "Total AI inference requests"
        ))
        .unwrap();
        let ai_failures_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_ai_failures_total",
            "Total AI inference failures"
        ))
        .unwrap();
        let backend_publishes_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_backend_publishes_total",
            "Total backend publish attempts"
        ))
        .unwrap();
        let backend_publish_failures_total = IntGauge::with_opts(prometheus::opts!(
            "vigilantai_gateway_backend_publish_failures_total",
            "Total backend publish failures"
        ))
        .unwrap();

        registry
            .register(Box::new(cameras_connected.clone()))
            .unwrap();
        registry.register(Box::new(cameras_online.clone())).unwrap();
        registry
            .register(Box::new(cameras_offline.clone()))
            .unwrap();
        registry
            .register(Box::new(reconnect_attempts_total.clone()))
            .unwrap();
        registry
            .register(Box::new(frames_processed_total.clone()))
            .unwrap();
        registry
            .register(Box::new(frames_dropped_total.clone()))
            .unwrap();
        registry
            .register(Box::new(decode_errors_total.clone()))
            .unwrap();
        registry
            .register(Box::new(current_bitrate_bps.clone()))
            .unwrap();
        registry
            .register(Box::new(ai_requests_total.clone()))
            .unwrap();
        registry
            .register(Box::new(ai_failures_total.clone()))
            .unwrap();
        registry
            .register(Box::new(backend_publishes_total.clone()))
            .unwrap();
        registry
            .register(Box::new(backend_publish_failures_total.clone()))
            .unwrap();

        Self {
            registry,
            cameras_connected,
            cameras_online,
            cameras_offline,
            reconnect_attempts_total,
            frames_processed_total,
            frames_dropped_total,
            decode_errors_total,
            current_bitrate_bps,
            ai_requests_total,
            ai_failures_total,
            backend_publishes_total,
            backend_publish_failures_total,
        }
    }

    async fn update_from_state_full(&self, state: &GatewayState) {
        let total = state.worker_count() as i64;
        self.cameras_connected.set(total);

        let cameras = state.cameras.read().await;
        let mut online = 0i64;
        let mut offline = 0i64;
        let mut frames_total: i64 = 0;
        let mut frames_dropped_total: i64 = 0;
        let mut decode_errors_total: i64 = 0;
        let mut bitrate_total: i64 = 0;
        let mut reconnect_total: i64 = 0;
        let mut ai_ok: i64 = 0;
        let mut ai_fail: i64 = 0;
        let mut pub_ok: i64 = 0;
        let mut pub_fail: i64 = 0;
        for worker in cameras.values() {
            match worker.status().await {
                camera_gateway::models::CameraStatus::Online => online += 1,
                _ => offline += 1,
            }
            frames_total += worker.frames_processed() as i64;
            frames_dropped_total += worker.frames_dropped() as i64;
            decode_errors_total += worker.decode_errors() as i64;
            bitrate_total += worker.bitrate_bps().await as i64;
            reconnect_total += worker.reconnect_count() as i64;
            ai_ok += worker.successful_requests() as i64;
            ai_fail += worker.failed_requests() as i64;
            pub_ok += worker.successful_publishes() as i64;
            pub_fail += worker.failed_publishes() as i64;
        }
        self.cameras_online.set(online);
        self.cameras_offline.set(offline);
        self.frames_processed_total.set(frames_total);
        self.frames_dropped_total.set(frames_dropped_total);
        self.decode_errors_total.set(decode_errors_total);
        self.current_bitrate_bps.set(bitrate_total);
        self.reconnect_attempts_total.set(reconnect_total);
        self.ai_requests_total.set(ai_ok);
        self.ai_failures_total.set(ai_fail);
        self.backend_publishes_total.set(pub_ok);
        self.backend_publish_failures_total.set(pub_fail);
    }

    fn encode(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap()
    }
}

#[derive(Clone)]
struct HttpState {
    health: Arc<GatewayHealth>,
    manager: Arc<GatewayManager>,
    metrics: Arc<GatewayMetrics>,
}

async fn health_handler(AxumState(state): AxumState<HttpState>) -> Json<HealthResponse> {
    let ai_metrics = state.manager.ai_health_metrics().await;
    let backend_metrics = state.manager.backend_health_metrics().await;
    let response = state.health.check_full(ai_metrics, backend_metrics).await;
    Json(response)
}

async fn metrics_handler(AxumState(state): AxumState<HttpState>) -> impl IntoResponse {
    state.metrics.update_from_state_full(state.manager.state()).await;
    let body = state.metrics.encode();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        body,
    )
}

fn build_config_from_env() -> GatewayConfig {
    let ai_url = std::env::var("GATEWAY_AI_SERVICE_URL")
        .unwrap_or_else(|_| "http://ai-service:8081".to_string());
    let backend_url =
        std::env::var("GATEWAY_BACKEND_URL").unwrap_or_else(|_| "http://backend:8080".to_string());
    let auth_token = std::env::var("GATEWAY_AUTH_TOKEN").unwrap_or_default();

    GatewayConfig {
        ai: camera_gateway::config::AiConfig {
            service_url: ai_url,
            ..Default::default()
        },
        backend: camera_gateway::config::BackendConfig {
            url: backend_url,
            auth_token,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[derive(Debug, Clone, Serialize)]
struct CameraStatusResponse {
    id: uuid::Uuid,
    name: String,
    rtsp_url: String,
    status: CameraStatus,
    enabled: bool,
    location: Option<String>,
    fps: Option<i32>,
    resolution: Option<String>,
    last_seen_secs_ago: Option<u64>,
    frames_processed: u64,
    current_fps: f64,
    frames_dropped: u64,
    decode_errors: u64,
    reconnect_count: u64,
    bitrate_bps: u64,
    successful_inferences: u64,
    failed_inferences: u64,
}

async fn to_camera_status_response(worker: &camera_gateway::gateway::worker::CameraWorker) -> CameraStatusResponse {
    let last_seen = worker
        .last_seen()
        .await
        .map(|t| t.elapsed().as_secs());
    CameraStatusResponse {
        id: worker.camera_id(),
        name: worker.camera_name().to_string(),
        rtsp_url: worker.rtsp_url().to_string(),
        status: worker.status().await,
        enabled: worker.is_enabled(),
        location: None,
        fps: None,
        resolution: None,
        last_seen_secs_ago: last_seen,
        frames_processed: worker.frames_processed(),
        current_fps: worker.fps().await,
        frames_dropped: worker.frames_dropped(),
        decode_errors: worker.decode_errors(),
        reconnect_count: worker.reconnect_count(),
        bitrate_bps: worker.bitrate_bps().await,
        successful_inferences: worker.successful_requests(),
        failed_inferences: worker.failed_requests(),
    }
}

async fn cameras_handler(AxumState(state): AxumState<HttpState>) -> Json<Vec<CameraStatusResponse>> {
    let cameras = state.manager.state().cameras.read().await;
    let mut list = Vec::with_capacity(cameras.len());
    for (_, worker) in cameras.iter() {
        list.push(to_camera_status_response(worker).await);
    }
    Json(list)
}

async fn camera_handler(
    AxumState(state): AxumState<HttpState>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<CameraStatusResponse>, StatusCode> {
    let cameras = state.manager.state().cameras.read().await;
    let worker = cameras.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(to_camera_status_response(worker).await))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "camera_gateway=info,tower_http=info".into()),
        )
        .init();

    info!("camera-gateway starting");

    let config = build_config_from_env();
    let state = Arc::new(GatewayState::new());
    let manager = Arc::new(GatewayManager::new(Arc::clone(&state), config.clone()));
    let health = Arc::new(GatewayHealth::new(Arc::clone(&state)));
    let metrics = Arc::new(GatewayMetrics::new());

    let http_state = HttpState {
        health: Arc::clone(&health),
        manager: Arc::clone(&manager),
        metrics: Arc::clone(&metrics),
    };

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/cameras", get(cameras_handler))
        .route("/cameras/{id}", get(camera_handler))
        .with_state(http_state);

    let port = std::env::var("GATEWAY_PORT").unwrap_or_else(|_| "8082".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    info!(address = %addr, "health server listening");

    seed_cameras_from_env(&manager).await;

    let heartbeat_interval = config.heartbeat_interval;
    let inference_interval = config.ai.inference_interval;

    tokio::select! {
        _ = axum::serve(listener, app) => {
            tracing::error!("health server exited");
        }
        _ = run_supervisor(
            Arc::clone(&manager),
            heartbeat_interval,
            inference_interval,
        ) => {}
        _ = shutdown_signal() => {
            info!("shutdown signal received");
        }
    }

    info!("camera-gateway stopped");
    Ok(())
}

/// Register cameras from the `GATEWAY_CAMERAS` environment variable.
///
/// Format: a comma-separated list of `name=rtsp://...` entries, e.g.
/// `GATEWAY_CAMERAS="lobby=rtsp://cam1/live,parking=rtsp://cam2/live"`.
/// This is a bootstrap mechanism until camera configuration is synced from the
/// backend API.
async fn seed_cameras_from_env(manager: &GatewayManager) {
    let raw = std::env::var("GATEWAY_CAMERAS").unwrap_or_default();
    let mut seeded = 0usize;
    for entry in raw.split(',').filter(|s| !s.trim().is_empty()) {
        let mut parts = entry.splitn(2, '=');
        let name = parts.next().unwrap_or("Camera").trim().to_string();
        let url = parts.next().unwrap_or("").trim().to_string();
        if !(url.starts_with("rtsp://") || url.starts_with("rtsps://")) {
            tracing::warn!(entry, "skipping invalid camera entry (expected name=rtsp://...)");
            continue;
        }
        let camera = Camera {
            id: uuid::Uuid::new_v4(),
            name,
            rtsp_url: url,
            location: None,
            fps: None,
            resolution: None,
            enabled: true,
        };
        match manager.register_camera(camera).await {
            Ok(worker) => {
                tracing::info!(camera = %worker.camera_name(), "seeded camera from env");
                seeded += 1;
            }
            Err(e) => tracing::warn!(error = %e, "failed to seed camera from env"),
        }
    }
    if seeded == 0 && !raw.trim().is_empty() {
        tracing::warn!("no cameras were seeded from GATEWAY_CAMERAS");
    }
}

/// Supervise camera workers: run inference loops for online workers, reconnect
/// failed workers with backoff, and detect stalled streams via heartbeats.
async fn run_supervisor(
    manager: Arc<GatewayManager>,
    heartbeat_interval: Duration,
    inference_interval: Duration,
) {
    let mut loops: HashMap<uuid::Uuid, tokio::task::JoinHandle<()>> = HashMap::new();
    let reconnecting: Arc<tokio::sync::Mutex<HashSet<uuid::Uuid>>> =
        Arc::new(tokio::sync::Mutex::new(HashSet::new()));

    loop {
        let worker_ids: Vec<uuid::Uuid> = {
            let cameras = manager.state().cameras.read().await;
            cameras
                .values()
                .filter(|w| w.is_enabled())
                .map(|w| w.camera_id())
                .collect()
        };

        for id in worker_ids {
            let worker = {
                let cameras = manager.state().cameras.read().await;
                match cameras.get(&id) {
                    Some(w) => Arc::clone(w),
                    None => continue,
                }
            };

            if worker.is_running() {
                worker.heartbeat().await;
                if !worker.is_running() {
                    tracing::warn!(camera = %worker.camera_name(), "camera heartbeat failed");
                }
            }

            if worker.is_running() {
                if !loops.contains_key(&id) {
                    let w = Arc::clone(&worker);
                    loops.insert(
                        id,
                        tokio::spawn(async move {
                            w.run_inference_loop(inference_interval).await;
                        }),
                    );
                }
            } else {
                if let Some(handle) = loops.remove(&id) {
                    handle.abort();
                }
                if reconnecting.lock().await.contains(&id) {
                    continue;
                }
                reconnecting.lock().await.insert(id);
                let mgr = Arc::clone(&manager);
                let w = Arc::clone(&worker);
                let reconnecting = Arc::clone(&reconnecting);
                tokio::spawn(async move {
                    tracing::warn!(
                        camera = %w.camera_name(),
                        "camera offline, attempting to reconnect"
                    );
                    let ok = mgr.start_worker_with_reconnect(id).await;
                    reconnecting.lock().await.remove(&id);
                    if ok {
                        tracing::info!(camera = %w.camera_name(), "camera reconnected");
                    } else {
                        tracing::warn!(
                            camera = %w.camera_name(),
                            "camera reconnect failed after retries"
                        );
                    }
                });
            }
        }

        tokio::time::sleep(heartbeat_interval).await;
    }
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();
    #[cfg(unix)]
    {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => { info!("received SIGINT"); }
            _ = sigterm.recv() => { info!("received SIGTERM"); }
        }
    }
    #[cfg(not(unix))]
    {
        match ctrl_c.await {
            Ok(()) => info!("received SIGINT"),
            Err(e) => tracing::error!(error = %e, "failed to listen for Ctrl+C"),
        }
    }
}
