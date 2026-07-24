use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraStatus {
    Offline,
    Connecting,
    Online,
    Error,
    Stopped,
}

impl CameraStatus {
    pub const ALL: &'static [CameraStatus] = &[
        CameraStatus::Offline,
        CameraStatus::Connecting,
        CameraStatus::Online,
        CameraStatus::Error,
        CameraStatus::Stopped,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            CameraStatus::Offline => "offline",
            CameraStatus::Connecting => "connecting",
            CameraStatus::Online => "online",
            CameraStatus::Error => "error",
            CameraStatus::Stopped => "stopped",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "offline" => Some(CameraStatus::Offline),
            "connecting" => Some(CameraStatus::Connecting),
            "online" => Some(CameraStatus::Online),
            "error" => Some(CameraStatus::Error),
            "stopped" => Some(CameraStatus::Stopped),
            _ => None,
        }
    }
}

impl fmt::Display for CameraStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for CameraStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown camera status: {s}"))
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub id: uuid::Uuid,
    pub name: String,
    pub rtsp_url: String,
    pub location: Option<String>,
    pub fps: Option<i32>,
    pub resolution: Option<String>,
    pub enabled: bool,
}

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
    fn test_status_as_str_roundtrip() {
        for status in CameraStatus::ALL {
            let s = status.as_str();
            let parsed = CameraStatus::from_db_str(s).unwrap();
            assert_eq!(*status, parsed);
        }
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!("invalid".parse::<CameraStatus>().is_err());
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
        assert_eq!(CameraStatus::ALL.len(), 5);
        assert!(CameraStatus::ALL.contains(&CameraStatus::Offline));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Connecting));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Online));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Error));
        assert!(CameraStatus::ALL.contains(&CameraStatus::Stopped));
    }

    #[test]
    fn test_camera_clone() {
        let camera = Camera {
            id: uuid::Uuid::new_v4(),
            name: "Test".to_string(),
            rtsp_url: "rtsp://10.0.0.1/stream".to_string(),
            location: Some("Lobby".to_string()),
            fps: Some(30),
            resolution: Some("1920x1080".to_string()),
            enabled: true,
        };
        let cloned = camera.clone();
        assert_eq!(camera.id, cloned.id);
        assert_eq!(camera.name, cloned.name);
    }
}
