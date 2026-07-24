use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// CameraStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraStatus {
    Online,
    Offline,
    Connecting,
    Error,
}

impl CameraStatus {
    pub const ALL: &'static [CameraStatus] = &[
        CameraStatus::Online,
        CameraStatus::Offline,
        CameraStatus::Connecting,
        CameraStatus::Error,
    ];

    pub fn as_db_str(&self) -> &'static str {
        match self {
            CameraStatus::Online => "online",
            CameraStatus::Offline => "offline",
            CameraStatus::Connecting => "connecting",
            CameraStatus::Error => "error",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "online" => Some(CameraStatus::Online),
            "offline" => Some(CameraStatus::Offline),
            "connecting" => Some(CameraStatus::Connecting),
            "error" => Some(CameraStatus::Error),
            _ => None,
        }
    }
}

impl fmt::Display for CameraStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for CameraStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown camera status: {s}"))
    }
}

// ---------------------------------------------------------------------------
// Camera
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Camera {
    pub id: uuid::Uuid,
    pub name: String,
    pub location: Option<String>,
    pub rtsp_url: String,
    pub status: String,
    pub enabled: bool,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payload for creating a new camera.
#[derive(Debug, Clone)]
pub struct CreateCamera {
    pub name: String,
    pub location: Option<String>,
    pub rtsp_url: String,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
}

/// Payload for updating an existing camera.
#[derive(Debug, Clone)]
pub struct UpdateCamera {
    pub name: Option<String>,
    pub location: Option<Option<String>>,
    pub rtsp_url: Option<String>,
    pub fps: Option<Option<i32>>,
    pub resolution: Option<Option<String>>,
    pub enabled: Option<bool>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_display_roundtrip() {
        for status in CameraStatus::ALL {
            let s = status.to_string();
            let parsed: CameraStatus = s.parse().unwrap();
            assert_eq!(*status, parsed);
        }
    }

    #[test]
    fn test_status_db_str_roundtrip() {
        for status in CameraStatus::ALL {
            let db_str = status.as_db_str();
            let parsed = CameraStatus::from_db_str(db_str).unwrap();
            assert_eq!(*status, parsed);
        }
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!("invalid_status".parse::<CameraStatus>().is_err());
    }

    #[test]
    fn test_status_serialization() {
        let status = CameraStatus::Online;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"online\"");

        let deserialized: CameraStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, CameraStatus::Online);
    }

    #[test]
    fn test_status_all_variants() {
        assert_eq!(CameraStatus::ALL.len(), 4);
        assert!(CameraStatus::ALL.contains(&CameraStatus::Online));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Offline));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Connecting));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Error));
    }

    #[test]
    fn test_status_deserialize_all() {
        for status in CameraStatus::ALL {
            let json = serde_json::to_string(status).unwrap();
            let deserialized: CameraStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, *status);
        }
    }
}
