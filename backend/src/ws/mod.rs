use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::{broadcast, RwLock};
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::errors::AppError;
use crate::security::jwt;
use crate::state::AppState;

#[derive(Clone)]
pub struct WsState {
    pub alerts: broadcast::Sender<String>,
    pub incidents: broadcast::Sender<String>,
    pub dashboard: broadcast::Sender<String>,
    pub detections: broadcast::Sender<String>,
    pub last_state: Arc<RwLock<HashMap<String, String>>>,
}

impl WsState {
    pub fn new() -> Self {
        let (alerts, _) = broadcast::channel(256);
        let (incidents, _) = broadcast::channel(256);
        let (dashboard, _) = broadcast::channel(256);
        let (detections, _) = broadcast::channel(256);
        Self {
            alerts,
            incidents,
            dashboard,
            detections,
            last_state: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Deserialize)]
struct WsQuery {
    token: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    #[serde(rename = "subscribe")]
    Subscribe { channel: String },
    #[serde(rename = "unsubscribe")]
    Unsubscribe { channel: String },
    #[serde(rename = "ping")]
    Ping,
}

pub fn websocket_routes() -> Router<AppState> {
    Router::new().route("/ws/v1", get(ws_handler))
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    jwt::validate_token(&query.token, &state.security.decoding_key)
        .map_err(|e| AppError::InvalidToken(e.to_string()))?;

    info!("websocket upgrade successful");
    Ok(ws.on_upgrade(move |socket| handle_socket(socket, state)))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let ws_state = state.ws_state.clone();

    let mut alert_rx = ws_state.alerts.subscribe();
    let mut incident_rx = ws_state.incidents.subscribe();
    let mut dashboard_rx = ws_state.dashboard.subscribe();
    let mut detection_rx = ws_state.detections.subscribe();

    let mut subscribed: HashSet<String> = HashSet::new();
    let mut heartbeat = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Subscribe { channel }) => {
                                match channel.as_str() {
                                    "alerts" | "incidents" | "dashboard" | "detections" => {
                                        if subscribed.insert(channel.clone()) {
                                            info!(%channel, "client subscribed");
                                            let state_guard = ws_state.last_state.read().await;
                                            if let Some(current) = state_guard.get(&channel) {
                                                let _ = socket.send(Message::Text(current.clone())).await;
                                            }
                                        }
                                    }
                                    _ => {
                                        warn!(%channel, "unknown channel");
                                    }
                                }
                            }
                            Ok(ClientMessage::Unsubscribe { channel }) => {
                                if subscribed.remove(&channel) {
                                    info!(%channel, "client unsubscribed");
                                }
                            }
                            Ok(ClientMessage::Ping) => {}
                            Err(e) => {
                                warn!(error = %e, "invalid message");
                            }
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        info!("websocket connection closed");
                        break;
                    }
                    Some(Err(e)) => {
                        error!(error = %e, "websocket error");
                        break;
                    }
                    _ => {}
                }
            }
            result = alert_rx.recv() => {
                match result {
                    Ok(payload) => {
                        if subscribed.contains("alerts") {
                            if socket.send(Message::Text(payload)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!(%n, "alert channel lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            result = incident_rx.recv() => {
                match result {
                    Ok(payload) => {
                        if subscribed.contains("incidents") {
                            if socket.send(Message::Text(payload)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!(%n, "incident channel lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            result = dashboard_rx.recv() => {
                match result {
                    Ok(payload) => {
                        if subscribed.contains("dashboard") {
                            if socket.send(Message::Text(payload)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!(%n, "dashboard channel lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            result = detection_rx.recv() => {
                match result {
                    Ok(payload) => {
                        if subscribed.contains("detections") {
                            if socket.send(Message::Text(payload)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!(%n, "detection channel lagged");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            _ = heartbeat.tick() => {
                if socket.send(Message::Text(json!({"type": "pong"}).to_string())).await.is_err() {
                    break;
                }
            }
        }
    }
}
