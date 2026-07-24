use std::sync::Arc;
use std::time::Duration;

use axum::extract::State as AxumState;
use axum::routing::get;
use axum::{Json, Router};
use tokio::net::TcpListener;
use tracing::info;

use camera_gateway::config::GatewayConfig;
use camera_gateway::gateway::manager::GatewayManager;
use camera_gateway::gateway::state::GatewayState;
use camera_gateway::services::health::{GatewayHealth, HealthResponse};

#[derive(Clone)]
struct HttpState {
    health: Arc<GatewayHealth>,
    manager: Arc<GatewayManager>,
}

async fn health_handler(AxumState(state): AxumState<HttpState>) -> Json<HealthResponse> {
    let ai_metrics = state.manager.ai_health_metrics().await;
    let backend_metrics = state.manager.backend_health_metrics().await;
    let response = state.health.check_full(ai_metrics, backend_metrics).await;
    Json(response)
}

fn build_config_from_env() -> GatewayConfig {
    let ai_url = std::env::var("GATEWAY_AI_SERVICE_URL")
        .unwrap_or_else(|_| "http://ai-service:8081".to_string());
    let backend_url = std::env::var("GATEWAY_BACKEND_URL")
        .unwrap_or_else(|_| "http://backend:8080".to_string());
    let auth_token = std::env::var("GATEWAY_AUTH_TOKEN")
        .unwrap_or_default();

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

    let http_state = HttpState {
        health: Arc::clone(&health),
        manager: Arc::clone(&manager),
    };

    let app = Router::new()
        .route("/health", get(health_handler))
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
