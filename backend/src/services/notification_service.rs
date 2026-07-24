use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use sqlx::postgres::PgPool;

use crate::config::notification::NotificationConfig;
use crate::dto::notification::{
    NotificationListResponse, NotificationPaginationParams, NotificationResponse,
};
use crate::models::{
    CreateNotification, Incident, Notification, NotificationChannel, NotificationStatus,
};
use crate::repository::notification_repository::NotificationRepository;

use super::notifiers::Notifier;

pub struct NotificationService<R: NotificationRepository> {
    repository: R,
    notifiers: HashMap<String, Arc<dyn Notifier>>,
    config: NotificationConfig,
}

impl<R: NotificationRepository> NotificationService<R> {
    pub fn new(repository: R, config: NotificationConfig) -> Self {
        Self {
            repository,
            notifiers: HashMap::new(),
            config,
        }
    }

    /// Register a notifier for a channel.
    pub fn register_notifier(&mut self, channel: NotificationChannel, notifier: Arc<dyn Notifier>) {
        self.notifiers.insert(channel.to_string(), notifier);
    }

    /// Send a single notification.
    pub async fn send_notification(
        &self,
        pool: &PgPool,
        notification: &CreateNotification,
    ) -> Result<Notification> {
        let record = self.repository.create(pool, notification).await?;

        let notifier = self
            .notifiers
            .get(notification.channel.as_db_str())
            .ok_or_else(|| {
                anyhow!(
                    "no notifier registered for channel: {}",
                    notification.channel
                )
            })?;

        let payload = serde_json::json!({
            "notification_id": record.id.to_string(),
            "incident_id": notification.incident_id.to_string(),
            "channel": notification.channel.to_string(),
            "recipient": notification.recipient,
        });

        let result = notifier
            .send(&notification.recipient, &payload.to_string())
            .await;

        let (status, response_code, error_message) = if result.success {
            (
                NotificationStatus::Sent.as_db_str(),
                result.response_code,
                None,
            )
        } else if result.is_retryable() && self.config.max_retries > 0 {
            (
                NotificationStatus::Retrying.as_db_str(),
                result.response_code,
                result.error_message.as_deref(),
            )
        } else {
            (
                NotificationStatus::Failed.as_db_str(),
                result.response_code,
                result.error_message.as_deref(),
            )
        };

        let updated = self
            .repository
            .update_status(pool, record.id, status, 1, response_code, error_message)
            .await?;

        Ok(updated.unwrap_or(record))
    }

    /// Send notifications through all registered notifiers for an incident.
    pub async fn send_incident_notifications(
        &self,
        pool: &PgPool,
        incident: &Incident,
        recipients: &[(NotificationChannel, String)],
    ) -> Result<Vec<Notification>> {
        if !self.config.enabled {
            return Ok(vec![]);
        }

        let mut results = Vec::new();

        for (channel, recipient) in recipients {
            if !self.is_channel_enabled(channel) {
                continue;
            }

            let create = CreateNotification {
                incident_id: incident.id,
                channel: *channel,
                recipient: recipient.clone(),
            };

            match self.send_notification(pool, &create).await {
                Ok(notification) => results.push(notification),
                Err(e) => {
                    tracing::error!(
                        channel = %channel,
                        recipient = %recipient,
                        incident_id = %incident.id,
                        error = %e,
                        "failed to send notification"
                    );
                }
            }
        }

        Ok(results)
    }

    /// Retry failed notifications.
    pub async fn retry_failed(&self, pool: &PgPool) -> Result<Vec<Notification>> {
        let retryable = self
            .repository
            .find_retryable(pool, self.config.max_retries as i32)
            .await?;

        let mut results = Vec::new();

        for notification in retryable {
            let notifier = match self.notifiers.get(&notification.channel) {
                Some(n) => n,
                None => continue,
            };

            let payload = serde_json::json!({
                "notification_id": notification.id.to_string(),
                "incident_id": notification.incident_id.to_string(),
                "channel": notification.channel,
                "recipient": notification.recipient,
                "retry": true,
                "attempt": notification.attempts + 1,
            });

            let result = notifier
                .send(&notification.recipient, &payload.to_string())
                .await;

            let (status, response_code, error_message) = if result.success {
                (
                    NotificationStatus::Sent.as_db_str(),
                    result.response_code,
                    None,
                )
            } else if result.is_retryable()
                && notification.attempts + 1 < self.config.max_retries as i32
            {
                (
                    NotificationStatus::Retrying.as_db_str(),
                    result.response_code,
                    result.error_message.as_deref(),
                )
            } else {
                (
                    NotificationStatus::Failed.as_db_str(),
                    result.response_code,
                    result.error_message.as_deref(),
                )
            };

            if let Some(updated) = self
                .repository
                .update_status(
                    pool,
                    notification.id,
                    status,
                    notification.attempts + 1,
                    response_code,
                    error_message,
                )
                .await?
            {
                results.push(updated);
            }
        }

        Ok(results)
    }

    /// Get notification history.
    pub async fn history(
        &self,
        pool: &PgPool,
        params: &NotificationPaginationParams,
    ) -> Result<NotificationListResponse> {
        let (offset, limit) = params.offset_limit();
        let notifications = self
            .repository
            .list(pool, params, offset as i64, limit as i64)
            .await?;
        let total = self.repository.count(pool, params).await?;
        let page = params.page.unwrap_or(1).max(1);

        let responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(Self::notification_response)
            .collect();

        Ok(NotificationListResponse {
            notifications: responses,
            total,
            page,
            per_page: limit,
        })
    }

    /// Get a single notification by ID.
    pub async fn get(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Notification> {
        self.repository
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("notification not found"))
    }

    fn is_channel_enabled(&self, channel: &NotificationChannel) -> bool {
        match channel {
            NotificationChannel::Email => self.config.email_enabled,
            NotificationChannel::Webhook => !self.config.webhook_url.is_empty(),
        }
    }

    fn notification_response(notification: Notification) -> NotificationResponse {
        NotificationResponse {
            id: notification.id,
            incident_id: notification.incident_id,
            channel: NotificationChannel::from_db_str(&notification.channel)
                .unwrap_or(NotificationChannel::Webhook),
            recipient: notification.recipient,
            status: NotificationStatus::from_db_str(&notification.status)
                .unwrap_or(NotificationStatus::Pending),
            attempts: notification.attempts,
            response_code: notification.response_code,
            error_message: notification.error_message,
            created_at: notification.created_at,
            sent_at: notification.sent_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_service_new() {
        let config = NotificationConfig {
            enabled: true,
            webhook_url: "https://example.com/hook".to_string(),
            webhook_timeout_secs: 10,
            email_enabled: false,
            max_retries: 3,
        };
        let repo = crate::repository::notification_repository::PostgresNotificationRepository;
        let service = NotificationService::new(repo, config);
        assert!(service.notifiers.is_empty());
    }

    #[test]
    fn test_is_channel_enabled_webhook() {
        let config = NotificationConfig {
            enabled: true,
            webhook_url: "https://example.com/hook".to_string(),
            webhook_timeout_secs: 10,
            email_enabled: false,
            max_retries: 3,
        };
        let repo = crate::repository::notification_repository::PostgresNotificationRepository;
        let service = NotificationService::new(repo, config);
        assert!(service.is_channel_enabled(&NotificationChannel::Webhook));
    }

    #[test]
    fn test_is_channel_disabled_email() {
        let config = NotificationConfig {
            enabled: true,
            webhook_url: "https://example.com/hook".to_string(),
            webhook_timeout_secs: 10,
            email_enabled: false,
            max_retries: 3,
        };
        let repo = crate::repository::notification_repository::PostgresNotificationRepository;
        let service = NotificationService::new(repo, config);
        assert!(!service.is_channel_enabled(&NotificationChannel::Email));
    }

    #[test]
    fn test_is_channel_enabled_email() {
        let config = NotificationConfig {
            enabled: true,
            webhook_url: "https://example.com/hook".to_string(),
            webhook_timeout_secs: 10,
            email_enabled: true,
            max_retries: 3,
        };
        let repo = crate::repository::notification_repository::PostgresNotificationRepository;
        let service = NotificationService::new(repo, config);
        assert!(service.is_channel_enabled(&NotificationChannel::Email));
    }

    #[test]
    fn test_notification_response_conversion() {
        let notification = Notification {
            id: uuid::Uuid::new_v4(),
            incident_id: uuid::Uuid::new_v4(),
            channel: "webhook".to_string(),
            recipient: "https://example.com/hook".to_string(),
            status: "sent".to_string(),
            attempts: 1,
            response_code: Some(200),
            error_message: None,
            created_at: chrono::Utc::now(),
            sent_at: Some(chrono::Utc::now()),
        };

        let response = NotificationService::<
            crate::repository::notification_repository::PostgresNotificationRepository,
        >::notification_response(notification.clone());

        assert_eq!(response.id, notification.id);
        assert_eq!(response.channel, NotificationChannel::Webhook);
        assert_eq!(response.status, NotificationStatus::Sent);
    }

    #[test]
    fn test_notification_response_default_channel() {
        let notification = Notification {
            id: uuid::Uuid::new_v4(),
            incident_id: uuid::Uuid::new_v4(),
            channel: "unknown".to_string(),
            recipient: "test".to_string(),
            status: "pending".to_string(),
            attempts: 0,
            response_code: None,
            error_message: None,
            created_at: chrono::Utc::now(),
            sent_at: None,
        };

        let response = NotificationService::<
            crate::repository::notification_repository::PostgresNotificationRepository,
        >::notification_response(notification);

        // Unknown channel defaults to Webhook
        assert_eq!(response.channel, NotificationChannel::Webhook);
    }
}
