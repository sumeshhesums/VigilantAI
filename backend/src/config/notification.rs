use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub webhook_url: String,
    pub webhook_timeout_secs: u64,
    pub email_enabled: bool,
    pub max_retries: u32,
}

impl NotificationConfig {
    pub fn from_env() -> Result<Self> {
        let enabled = std::env::var("NOTIFICATION_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .context("NOTIFICATION_ENABLED must be true or false")?;

        let webhook_url = std::env::var("NOTIFICATION_WEBHOOK_URL").unwrap_or_default();

        let webhook_timeout_secs = std::env::var("NOTIFICATION_WEBHOOK_TIMEOUT_SECS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u64>()
            .context("NOTIFICATION_WEBHOOK_TIMEOUT_SECS must be a number")?;

        let email_enabled = std::env::var("NOTIFICATION_EMAIL_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .context("NOTIFICATION_EMAIL_ENABLED must be true or false")?;

        let max_retries = std::env::var("NOTIFICATION_MAX_RETRIES")
            .unwrap_or_else(|_| "3".to_string())
            .parse::<u32>()
            .context("NOTIFICATION_MAX_RETRIES must be a number")?;

        Ok(Self {
            enabled,
            webhook_url,
            webhook_timeout_secs,
            email_enabled,
            max_retries,
        })
    }
}
