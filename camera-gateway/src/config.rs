use std::time::Duration;

#[derive(Debug, Clone)]
pub struct GatewayConfig {
    pub heartbeat_interval: Duration,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay: Duration,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(10),
            max_reconnect_attempts: 5,
            reconnect_delay: Duration::from_secs(5),
        }
    }
}
