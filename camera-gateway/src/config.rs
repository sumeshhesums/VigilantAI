use std::time::Duration;

/// RTSP connection-specific configuration.
#[derive(Debug, Clone)]
pub struct RtspConfig {
    /// Timeout for a single connection attempt.
    pub connection_timeout: Duration,
}

impl Default for RtspConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(10),
        }
    }
}

/// Reconnection policy configuration.
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Initial delay before the first retry.
    pub initial_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
    /// Maximum number of retry attempts. `None` means unlimited.
    pub max_retries: Option<u32>,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            max_retries: Some(10),
        }
    }
}

impl ReconnectConfig {
    /// Convert this config into a `ReconnectPolicy`.
    pub fn to_policy(&self) -> crate::stream::reconnect::ReconnectPolicy {
        crate::stream::reconnect::ReconnectPolicy {
            initial_delay: self.initial_delay,
            max_delay: self.max_delay,
            max_retries: self.max_retries,
        }
    }
}

/// Top-level gateway configuration.
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Interval between heartbeat checks.
    pub heartbeat_interval: Duration,
    /// RTSP connection settings.
    pub rtsp: RtspConfig,
    /// Reconnection policy settings.
    pub reconnect: ReconnectConfig,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(10),
            rtsp: RtspConfig::default(),
            reconnect: ReconnectConfig::default(),
        }
    }
}
