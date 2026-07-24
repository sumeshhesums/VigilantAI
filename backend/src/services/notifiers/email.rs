use async_trait::async_trait;

use super::notifier_trait::{Notifier, SendResult};

/// Stub email notifier. Returns a successful mock result.
/// Architecture allows replacing with lettre or another mail provider.
pub struct EmailNotifier {
    enabled: bool,
}

impl EmailNotifier {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

#[async_trait]
impl Notifier for EmailNotifier {
    async fn send(&self, recipient: &str, payload: &str) -> SendResult {
        if !self.enabled {
            return SendResult::failure("email notifications are disabled".to_string());
        }

        // Stub implementation: log and return success
        tracing::info!(
            to = %recipient,
            payload_len = payload.len(),
            "email notification sent (stub)"
        );

        SendResult::success(200)
    }

    async fn health(&self) -> bool {
        self.enabled
    }

    fn name(&self) -> &str {
        "email"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_new() {
        let notifier = EmailNotifier::new(true);
        assert_eq!(notifier.name(), "email");
    }

    #[test]
    fn test_email_health_enabled() {
        let notifier = EmailNotifier::new(true);
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(rt.block_on(notifier.health()));
    }

    #[test]
    fn test_email_health_disabled() {
        let notifier = EmailNotifier::new(false);
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(!rt.block_on(notifier.health()));
    }

    #[tokio::test]
    async fn test_email_send_enabled() {
        let notifier = EmailNotifier::new(true);
        let result = notifier.send("admin@example.com", "test payload").await;
        assert!(result.success);
        assert_eq!(result.response_code, Some(200));
    }

    #[tokio::test]
    async fn test_email_send_disabled() {
        let notifier = EmailNotifier::new(false);
        let result = notifier.send("admin@example.com", "test payload").await;
        assert!(!result.success);
        assert!(result.error_message.unwrap().contains("disabled"));
    }
}
