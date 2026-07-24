use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Severity
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl IncidentSeverity {
    pub const ALL: &'static [IncidentSeverity] = &[
        IncidentSeverity::Low,
        IncidentSeverity::Medium,
        IncidentSeverity::High,
        IncidentSeverity::Critical,
    ];

    pub fn as_db_str(&self) -> &'static str {
        match self {
            IncidentSeverity::Low => "low",
            IncidentSeverity::Medium => "medium",
            IncidentSeverity::High => "high",
            IncidentSeverity::Critical => "critical",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(IncidentSeverity::Low),
            "medium" => Some(IncidentSeverity::Medium),
            "high" => Some(IncidentSeverity::High),
            "critical" => Some(IncidentSeverity::Critical),
            _ => None,
        }
    }
}

impl fmt::Display for IncidentSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for IncidentSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown severity: {s}"))
    }
}

// ---------------------------------------------------------------------------
// IncidentStatus
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentStatus {
    Open,
    Acknowledged,
    Resolved,
    FalsePositive,
}

impl IncidentStatus {
    pub const ALL: &'static [IncidentStatus] = &[
        IncidentStatus::Open,
        IncidentStatus::Acknowledged,
        IncidentStatus::Resolved,
        IncidentStatus::FalsePositive,
    ];

    pub fn as_db_str(&self) -> &'static str {
        match self {
            IncidentStatus::Open => "open",
            IncidentStatus::Acknowledged => "acknowledged",
            IncidentStatus::Resolved => "resolved",
            IncidentStatus::FalsePositive => "false_positive",
        }
    }

    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "open" => Some(IncidentStatus::Open),
            "acknowledged" => Some(IncidentStatus::Acknowledged),
            "resolved" => Some(IncidentStatus::Resolved),
            "false_positive" => Some(IncidentStatus::FalsePositive),
            _ => None,
        }
    }
}

impl fmt::Display for IncidentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_db_str())
    }
}

impl FromStr for IncidentStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_db_str(s).ok_or_else(|| format!("unknown status: {s}"))
    }
}

// ---------------------------------------------------------------------------
// BoundingBox
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

// ---------------------------------------------------------------------------
// Incident
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Incident {
    pub id: uuid::Uuid,
    pub camera_id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub severity: String,
    pub status: String,
    pub event_type: String,
    pub confidence: f64,
    pub bounding_box: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ---------------------------------------------------------------------------
// CreateIncident / UpdateIncident (internal payloads)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct CreateIncident {
    pub camera_id: uuid::Uuid,
    pub timestamp: DateTime<Utc>,
    pub severity: IncidentSeverity,
    pub event_type: String,
    pub confidence: f64,
    pub bounding_box: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct UpdateIncidentStatus {
    pub status: IncidentStatus,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_display_roundtrip() {
        for s in IncidentSeverity::ALL {
            let s_str = s.to_string();
            let parsed: IncidentSeverity = s_str.parse().unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_severity_db_str_roundtrip() {
        for s in IncidentSeverity::ALL {
            let db_str = s.as_db_str();
            let parsed = IncidentSeverity::from_db_str(db_str).unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_severity_from_str_invalid() {
        assert!("invalid".parse::<IncidentSeverity>().is_err());
    }

    #[test]
    fn test_severity_serialization() {
        let s = IncidentSeverity::Critical;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"critical\"");
        let deserialized: IncidentSeverity = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, IncidentSeverity::Critical);
    }

    #[test]
    fn test_severity_all_variants() {
        assert_eq!(IncidentSeverity::ALL.len(), 4);
    }

    #[test]
    fn test_status_display_roundtrip() {
        for s in IncidentStatus::ALL {
            let s_str = s.to_string();
            let parsed: IncidentStatus = s_str.parse().unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_status_db_str_roundtrip() {
        for s in IncidentStatus::ALL {
            let db_str = s.as_db_str();
            let parsed = IncidentStatus::from_db_str(db_str).unwrap();
            assert_eq!(*s, parsed);
        }
    }

    #[test]
    fn test_status_from_str_invalid() {
        assert!("invalid".parse::<IncidentStatus>().is_err());
    }

    #[test]
    fn test_status_serialization() {
        let s = IncidentStatus::Acknowledged;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"acknowledged\"");
        let deserialized: IncidentStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, IncidentStatus::Acknowledged);
    }

    #[test]
    fn test_status_all_variants() {
        assert_eq!(IncidentStatus::ALL.len(), 4);
    }

    #[test]
    fn test_bounding_box_serialization() {
        let bb = BoundingBox {
            x1: 10.0,
            y1: 20.0,
            x2: 100.0,
            y2: 200.0,
        };
        let json = serde_json::to_string(&bb).unwrap();
        let parsed: BoundingBox = serde_json::from_str(&json).unwrap();
        assert_eq!(bb, parsed);
    }
}
