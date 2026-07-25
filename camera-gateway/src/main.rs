use std::sync::Arc;
use std::time::Duration;

use axum::extract::State as AxumState;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use prometheus::{Encoder, IntCounter, IntGauge, Registry, TextEncoder};
use tokio::net::TcpListener;
use tracing::info;

use camera_gateway::config::GatewayConfig;
use camera_gateway::gateway::manager::GatewayManager;
use camera_gateway::gateway::state::GatewayState;
use camera_gateway::services::health::{GatewayHealth, HealthResponse};

#[derive(Clone)]
#[allow(dead_code)]
struct GatewayMetrics {
    registry: Registry,
    cameras_connected: IntGauge,
    cameras_online: IntGauge,
    cameras_offline: IntGauge,
    reconnect_attempts_total: IntCounter,
    frames_processed_total: IntCounter,
    ai_requests_total: IntCounter,
    ai_failures_total: IntCounter,
    backend_publishes_total: IntCounter,
    backend_publish_failures_total: IntCounter,
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
        let reconnect_attempts_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_gateway_reconnect_attempts_total",
            "Total reconnect attempts"
        ))
        .unwrap();
        let frames_processed_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_gateway_frames_processed_total",
            "Total frames processed"
        ))
        .unwrap();
        let ai_requests_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_gateway_ai_requests_total",
            "Total AI inference requests"
        ))
        .unwrap();
        let ai_failures_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_gateway_ai_failures_total",
            "Total AI inference failures"
        ))
        .unwrap();
        let backend_publishes_total = IntCounter::with_opts(prometheus::opts!(
            "vigilantai_gateway_backend_publishes_total",
            "Total backend publish attempts"
        ))
        .unwrap();
        let backend_publish_failures_total = IntCounter::with_opts(prometheus::opts!(
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
            ai_requests_total,
            ai_failures_total,
            backend_publishes_total,
            backend_publish_failures_total,
        }
    }

    fn update_from_state(&self, state: &GatewayState) {
        let total = state.worker_count() as i64;
        self.cameras_connected.set(total);
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
    state.metrics.update_from_state(state.manager.state());
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
        .with_state(http_state);

    let port = std::env::var("GATEWAY_PORT").unwrap_or_else(|_| "8082".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr).await?;
    info!(address = %addr, "health server listening");

    let heartbeat_interval = config.heartbeat_interval;

    tokio::select! {
        _ = axum::serve(listener, app) => {
            tracing::error!("health server exited");
        }
        _ = run_heartbeat_loop(Arc::clone(&manager), heartbeat_interval) => {}
        _ = shutdown_signal() => {
            info!("shutdown signal received");
        }
    }

    info!("camera-gateway stopped");
    Ok(())
}

async fn run_heartbeat_loop(manager: Arc<GatewayManager>, interval: Duration) {
    loop {
        tokio::time::sleep(interval).await;
        let failures = manager.heartbeat_all().await;
        if failures > 0 {
            tracing::warn!(failures, "heartbeat check found failing workers");
        }
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
