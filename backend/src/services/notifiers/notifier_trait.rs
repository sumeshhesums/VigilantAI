use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Result of a notification send attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub success: bool,
    pub response_code: Option<i32>,
    pub error_message: Option<String>,
}

impl SendResult {
    pub fn success(response_code: i32) -> Self {
        Self {
            success: true,
            response_code: Some(response_code),
            error_message: None,
        }
    }

    pub fn failure(error_message: String) -> Self {
        Self {
            success: false,
            response_code: None,
            error_message: Some(error_message),
        }
    }

    pub fn failure_with_code(response_code: i32, error_message: String) -> Self {
        Self {
            success: false,
            response_code: Some(response_code),
            error_message: Some(error_message),
        }
    }

    pub fn is_retryable(&self) -> bool {
        if self.success {
            return false;
        }
        match self.response_code {
            Some(code) => code >= 500,
            None => true, // network/timeout errors are retryable
        }
    }
}

/// Trait that all notification channels must implement.
#[async_trait]
pub trait Notifier: Send + Sync {
    /// Send a notification.
    async fn send(&self, recipient: &str, payload: &str) -> SendResult;

    /// Health check for this notifier.
    async fn health(&self) -> bool;

    /// Human-readable name of this notifier.
    fn name(&self) -> &str;
}
