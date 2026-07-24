use std::time::Duration;

use async_trait::async_trait;

use super::notifier_trait::{Notifier, SendResult};

pub struct WebhookNotifier {
    client: reqwest::Client,
    default_url: String,
    timeout: Duration,
}

impl WebhookNotifier {
    pub fn new(default_url: String, timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("failed to build reqwest client");

        Self {
            client,
            default_url,
            timeout: Duration::from_secs(timeout_secs),
        }
    }
}

#[async_trait]
impl Notifier for WebhookNotifier {
    async fn send(&self, recipient: &str, payload: &str) -> SendResult {
        let url = if recipient.is_empty() {
            &self.default_url
        } else {
            recipient
        };

        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .body(payload.to_string())
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status().as_u16() as i32;
                if resp.status().is_success() {
                    SendResult::success(status)
                } else {
                    let body = resp.text().await.unwrap_or_default();
                    SendResult::failure_with_code(status, body)
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    SendResult::failure(format!("timeout after {}s", self.timeout.as_secs()))
                } else if e.is_connect() {
                    SendResult::failure(format!("connection error: {e}"))
                } else {
                    SendResult::failure(format!("request error: {e}"))
                }
            }
        }
    }

    async fn health(&self) -> bool {
        // A simple health check: just verify the client can be used
        // In a real implementation, you might ping a health endpoint
        true
    }

    fn name(&self) -> &str {
        "webhook"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_new() {
        let notifier = WebhookNotifier::new("https://example.com/hook".to_string(), 10);
        assert_eq!(notifier.name(), "webhook");
    }

    #[test]
    fn test_webhook_health() {
        let notifier = WebhookNotifier::new("https://example.com/hook".to_string(), 10);
        // Health check is always true for webhook
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(rt.block_on(notifier.health()));
    }

    #[test]
    fn test_send_result_success() {
        let result = SendResult::success(200);
        assert!(result.success);
        assert_eq!(result.response_code, Some(200));
        assert!(!result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_5xx_retryable() {
        let result = SendResult::failure_with_code(500, "server error".to_string());
        assert!(!result.success);
        assert!(result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_4xx_not_retryable() {
        let result = SendResult::failure_with_code(400, "bad request".to_string());
        assert!(!result.success);
        assert!(!result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_network_retryable() {
        let result = SendResult::failure("connection refused".to_string());
        assert!(!result.success);
        assert!(result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_401_not_retryable() {
        let result = SendResult::failure_with_code(401, "unauthorized".to_string());
        assert!(!result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_403_not_retryable() {
        let result = SendResult::failure_with_code(403, "forbidden".to_string());
        assert!(!result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_404_not_retryable() {
        let result = SendResult::failure_with_code(404, "not found".to_string());
        assert!(!result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_502_retryable() {
        let result = SendResult::failure_with_code(502, "bad gateway".to_string());
        assert!(result.is_retryable());
    }

    #[test]
    fn test_send_result_failure_503_retryable() {
        let result = SendResult::failure_with_code(503, "service unavailable".to_string());
        assert!(result.is_retryable());
    }
}
