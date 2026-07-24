use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// NotificationChannel
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Webhook,
}

impl NotificationChannel {
    pub const ALL: &'static [NotificationChannel] =
        &[NotificationChannel::Email, NotificationChannel::Webhook];

    pub fn as_db_str(&self) -> &'static str {
        match self {
            NotificationChannel::Email => "email",
            NotificationChannel::Webhook => "webhook",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "email" => Some(NotificationChannel::Email),
            "webhook" => Some(NotificationChannel::Webhook),
            _ => None,
        }
    }
}

impl fmt::Display for NotificationChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for NotificationChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown channel: {s}"))
    }
}

// ---------------------------------------------------------------------------
// NotificationStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
    Retrying,
}

impl NotificationStatus {
    pub const ALL: &'static [NotificationStatus] = &[
        NotificationStatus::Pending,
        NotificationStatus::Sent,
        NotificationStatus::Failed,
        NotificationStatus::Retrying,
    ];

    pub fn as_db_str(&self) -> &'static str {
        match self {
            NotificationStatus::Pending => "pending",
            NotificationStatus::Sent => "sent",
            NotificationStatus::Failed => "failed",
            NotificationStatus::Retrying => "retrying",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(NotificationStatus::Pending),
            "sent" => Some(NotificationStatus::Sent),
            "failed" => Some(NotificationStatus::Failed),
            "retrying" => Some(NotificationStatus::Retrying),
            _ => None,
        }
    }

    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            NotificationStatus::Pending | NotificationStatus::Retrying
        )
    }
}

impl fmt::Display for NotificationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for NotificationStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown status: {s}"))
    }
}

// ---------------------------------------------------------------------------
// Notification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub incident_id: Uuid,
    pub channel: String,
    pub recipient: String,
    pub status: String,
    pub attempts: i32,
    pub response_code: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

// ---------------------------------------------------------------------------
// CreateNotification (internal payload)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateNotification {
    pub incident_id: Uuid,
    pub channel: NotificationChannel,
    pub recipient: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_display_roundtrip() {
        for c in NotificationChannel::ALL {
            let s = c.to_string();
            let parsed: NotificationChannel = s.parse().unwrap();
            assert_eq!(*c, parsed);
        }
    }

    #[test]
    fn test_channel_db_str_roundtrip() {
        for c in NotificationChannel::ALL {
            let db_str = c.as_db_str();
            let parsed = NotificationChannel::from_db_str(db_str).unwrap();
            assert_eq!(*c, parsed);
        }
    }

    #[test]
    fn test_channel_from_str_invalid() {
        assert!("invalid".parse::<NotificationChannel>().is_err());
    }

    #[test]
    fn test_channel_serialization() {
        let c = NotificationChannel::Webhook;
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(json, "\"webhook\"");
        let deserialized: NotificationChannel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, NotificationChannel::Webhook);
    }

    #[test]
    fn test_status_display_roundtrip() {
        for s in NotificationStatus::ALL {
            let s_str = s.to_string();
            let parsed: NotificationStatus = s_str.parse().unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_status_db_str_roundtrip() {
        for s in NotificationStatus::ALL {
            let db_str = s.as_db_str();
            let parsed = NotificationStatus::from_db_str(db_str).unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!("invalid".parse::<NotificationStatus>().is_err());
    }

    #[test]
    fn test_status_serialization() {
        let s = NotificationStatus::Failed;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"failed\"");
        let deserialized: NotificationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, NotificationStatus::Failed);
    }

    #[test]
    fn test_status_is_retryable() {
        assert!(NotificationStatus::Pending.is_retryable());
        assert!(NotificationStatus::Retrying.is_retryable());
        assert!(!NotificationStatus::Sent.is_retryable());
        assert!(!NotificationStatus::Failed.is_retryable());
    }

    #[test]
    fn test_channel_all_variants() {
        assert_eq!(NotificationChannel::ALL.len(), 2);
    }

    #[test]
    fn test_status_all_variants() {
        assert_eq!(NotificationStatus::ALL.len(), 4);
    }

    #[test]
    fn test_notification_serialization_roundtrip() {
        let notification = Notification {
            id: Uuid::new_v4(),
            incident_id: Uuid::new_v4(),
            channel: "webhook".to_string(),
            recipient: "https://example.com/hook".to_string(),
            status: "sent".to_string(),
            attempts: 1,
            response_code: Some(200),
            error_message: None,
            created_at: Utc::now(),
            sent_at: Some(Utc::now()),
        };
        let json = serde_json::to_string(&notification).unwrap();
        let deserialized: Notification = serde_json::from_str(&json).unwrap();
        assert_eq!(notification.id, deserialized.id);
        assert_eq!(notification.channel, deserialized.channel);
    }
}
