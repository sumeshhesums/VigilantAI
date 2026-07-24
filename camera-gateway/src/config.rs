use std::collections::HashMap;
use std::time::Duration;

use crate::backend::models::IncidentSeverity;

/// AI service integration configuration.
#[derive(Debug, Clone)]
pub struct AiConfig {
    /// Base URL of the AI inference service (e.g. http://ai-service:8081).
    pub service_url: String,
    /// Request timeout for AI inference calls.
    pub request_timeout: Duration,
    /// JPEG encoding quality (1–100).
    pub jpeg_quality: u8,
    /// Interval between inference cycles.
    pub inference_interval: Duration,
    /// Maximum allowed frame size in bytes.
    pub max_frame_size: usize,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            service_url: "http://localhost:8081".to_string(),
            request_timeout: Duration::from_secs(10),
            jpeg_quality: 85,
            inference_interval: Duration::from_millis(500),
            max_frame_size: 10 * 1024 * 1024,
        }
    }
}

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

/// Backend incident API integration configuration.
#[derive(Debug, Clone)]
pub struct BackendConfig {
    /// Base URL of the backend API (e.g. http://backend:8080).
    pub url: String,
    /// Request timeout for backend API calls.
    pub request_timeout: Duration,
    /// Bearer token for authenticating with the backend.
    pub auth_token: String,
    /// Whether to automatically publish detections as incidents.
    pub auto_publish: bool,
    /// Maximum number of retry attempts for transient failures.
    pub publish_retries: u32,
    /// Mapping from detection class_name to incident severity.
    pub severity_mapping: HashMap<String, IncidentSeverity>,
}

impl Default for BackendConfig {
    fn default() -> Self {
        let mut severity_mapping = HashMap::new();
        severity_mapping.insert("person".to_string(), IncidentSeverity::Medium);
        severity_mapping.insert("vehicle".to_string(), IncidentSeverity::Low);
        severity_mapping.insert("fire".to_string(), IncidentSeverity::Critical);
        severity_mapping.insert("smoke".to_string(), IncidentSeverity::High);
        severity_mapping.insert("weapon".to_string(), IncidentSeverity::Critical);
        severity_mapping.insert("unknown".to_string(), IncidentSeverity::Low);

        Self {
            url: "http://localhost:8080".to_string(),
            request_timeout: Duration::from_secs(10),
            auth_token: String::new(),
            auto_publish: true,
            publish_retries: 3,
            severity_mapping,
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
    /// AI service integration settings.
    pub ai: AiConfig,
    /// Backend incident API integration settings.
    pub backend: BackendConfig,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(10),
            rtsp: RtspConfig::default(),
            reconnect: ReconnectConfig::default(),
            ai: AiConfig::default(),
            backend: BackendConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_config_defaults() {
        let config = AiConfig::default();
        assert_eq!(config.service_url, "http://localhost:8081");
        assert_eq!(config.request_timeout, Duration::from_secs(10));
        assert_eq!(config.jpeg_quality, 85);
        assert_eq!(config.inference_interval, Duration::from_millis(500));
        assert_eq!(config.max_frame_size, 10 * 1024 * 1024);
    }

    #[test]
    fn test_ai_config_clone() {
        let config = AiConfig::default();
        let cloned = config.clone();
        assert_eq!(config.service_url, cloned.service_url);
        assert_eq!(config.request_timeout, cloned.request_timeout);
    }

    #[test]
    fn test_ai_config_debug() {
        let config = AiConfig::default();
        let debug = format!("{config:?}");
        assert!(debug.contains("AiConfig"));
        assert!(debug.contains("8081"));
    }

    #[test]
    fn test_gateway_config_includes_ai() {
        let config = GatewayConfig::default();
        assert_eq!(config.ai.service_url, "http://localhost:8081");
        assert_eq!(config.ai.jpeg_quality, 85);
    }

    #[test]
    fn test_rtsp_config_defaults() {
        let config = RtspConfig::default();
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
    }

    #[test]
    fn test_reconnect_config_defaults() {
        let config = ReconnectConfig::default();
        assert_eq!(config.initial_delay, Duration::from_secs(1));
        assert_eq!(config.max_delay, Duration::from_secs(60));
        assert_eq!(config.max_retries, Some(10));
    }

    #[test]
    fn test_reconnect_to_policy() {
        let config = ReconnectConfig::default();
        let policy = config.to_policy();
        assert_eq!(policy.initial_delay, Duration::from_secs(1));
        assert_eq!(policy.max_delay, Duration::from_secs(60));
    }

    #[test]
    fn test_backend_config_defaults() {
        let config = BackendConfig::default();
        assert_eq!(config.url, "http://localhost:8080");
        assert_eq!(config.request_timeout, Duration::from_secs(10));
        assert!(config.auth_token.is_empty());
        assert!(config.auto_publish);
        assert_eq!(config.publish_retries, 3);
    }

    #[test]
    fn test_backend_config_severity_mapping() {
        let config = BackendConfig::default();
        assert_eq!(
            config.severity_mapping.get("person"),
            Some(&IncidentSeverity::Medium)
        );
        assert_eq!(
            config.severity_mapping.get("vehicle"),
            Some(&IncidentSeverity::Low)
        );
        assert_eq!(
            config.severity_mapping.get("fire"),
            Some(&IncidentSeverity::Critical)
        );
        assert_eq!(
            config.severity_mapping.get("smoke"),
            Some(&IncidentSeverity::High)
        );
        assert_eq!(
            config.severity_mapping.get("weapon"),
            Some(&IncidentSeverity::Critical)
        );
        assert_eq!(
            config.severity_mapping.get("unknown"),
            Some(&IncidentSeverity::Low)
        );
    }

    #[test]
    fn test_backend_config_clone() {
        let config = BackendConfig::default();
        let cloned = config.clone();
        assert_eq!(config.url, cloned.url);
        assert_eq!(config.auth_token, cloned.auth_token);
    }

    #[test]
    fn test_backend_config_debug() {
        let config = BackendConfig::default();
        let debug = format!("{config:?}");
        assert!(debug.contains("BackendConfig"));
        assert!(debug.contains("8080"));
    }

    #[test]
    fn test_gateway_config_includes_backend() {
        let config = GatewayConfig::default();
        assert_eq!(config.backend.url, "http://localhost:8080");
        assert!(config.backend.auto_publish);
    }
}
