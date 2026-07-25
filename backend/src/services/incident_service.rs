use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::postgres::PgPool;

use crate::dto::incident::{
    CreateIncidentRequest, IncidentListResponse, IncidentPaginationParams, IncidentResponse,
    UpdateIncidentRequest,
};
use crate::models::{
    CreateIncident, Incident, IncidentSeverity, IncidentStatus, UpdateIncidentStatus,
};
use crate::repository::incident_repository::IncidentRepository;

/// Default deduplication window: incidents within this duration from the
/// same camera with the same event type are considered duplicates.
const DEFAULT_DEDUP_WINDOW_SECS: i64 = 60;

pub struct IncidentService<R: IncidentRepository> {
    repository: R,
}

impl<R: IncidentRepository> IncidentService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Create a new incident. Validates the request and checks for duplicates
    /// within the deduplication window.
    pub async fn create(&self, pool: &PgPool, req: &CreateIncidentRequest) -> Result<Incident> {
        Self::validate_create(req)?;

        // Check for duplicates within the window
        if let Some(duplicate) = self.deduplicate(pool, req).await? {
            return Ok(duplicate);
        }

        let now = Utc::now();
        let create = CreateIncident {
            camera_id: req.camera_id,
            timestamp: req.timestamp.unwrap_or(now),
            severity: req.severity,
            event_type: req.event_type.clone(),
            confidence: req.confidence,
            bounding_box: req
                .bounding_box
                .as_ref()
                .map(|bb| serde_json::to_value(bb).unwrap_or(serde_json::Value::Null)),
            metadata: req.metadata.clone(),
        };

        self.repository.create(pool, &create).await
    }

    /// List incidents with pagination and optional filters.
    pub async fn list(
        &self,
        pool: &PgPool,
        params: &IncidentPaginationParams,
    ) -> Result<IncidentListResponse> {
        let (offset, limit) = params.offset_limit();
        let incidents = self
            .repository
            .list(pool, params, offset as i64, limit as i64)
            .await?;
        let total = self.repository.count(pool, params).await?;
        let page = params.page.unwrap_or(1).max(1);

        let responses: Vec<IncidentResponse> =
            incidents.into_iter().map(Self::incident_response).collect();

        Ok(IncidentListResponse {
            incidents: responses,
            total,
            page,
            per_page: limit,
            pages: if limit > 0 {
                (total as u32).div_ceil(limit).max(1)
            } else {
                1
            },
        })
    }

    /// Get an incident by ID.
    pub async fn get(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Incident> {
        self.repository
            .find_by_id(pool, id)
            .await?
            .ok_or_else(|| anyhow!("incident not found"))
    }

    /// Update an incident's status.
    pub async fn update_status(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        req: &UpdateIncidentRequest,
    ) -> Result<Incident> {
        let update = UpdateIncidentStatus { status: req.status };

        self.repository
            .update_status(pool, id, &update)
            .await?
            .ok_or_else(|| anyhow!("incident not found"))
    }

    /// Check for duplicate incidents within the deduplication window.
    ///
    /// Returns the existing incident if a duplicate is found, or `None` if
    /// no duplicate exists.
    pub async fn deduplicate(
        &self,
        pool: &PgPool,
        req: &CreateIncidentRequest,
    ) -> Result<Option<Incident>> {
        let since = Utc::now() - chrono::Duration::seconds(DEFAULT_DEDUP_WINDOW_SECS);

        let similar = self
            .repository
            .find_similar(pool, req.camera_id, &req.event_type, since)
            .await?;

        // Check if any similar incident has matching severity and high confidence overlap
        for incident in similar {
            if incident.severity == req.severity.as_db_str() {
                // Check bounding box overlap if both have bounding boxes
                let existing_bb = incident.bounding_box.clone();
                let new_bb = req.bounding_box.clone();
                if let (Some(existing_bb), Some(new_bb_val)) = (existing_bb, new_bb) {
                    if let (Some(existing_x1), Some(new_x1)) = (
                        existing_bb.get("x1").and_then(|v| v.as_f64()),
                        serde_json::to_value(&new_bb_val)
                            .ok()
                            .and_then(|v| v.get("x1").and_then(|x| x.as_f64())),
                    ) {
                        // Simple overlap check: if bounding boxes are within 50 pixels
                        let existing_x2 = existing_bb
                            .get("x2")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let new_x2 = new_bb_val.x2;
                        let overlap = (existing_x1 - new_x1).abs() < 50.0
                            && (existing_x2 - new_x2).abs() < 50.0;
                        if overlap {
                            return Ok(Some(incident));
                        }
                    }
                } else {
                    // No bounding boxes — consider it a duplicate
                    return Ok(Some(incident));
                }
            }
        }

        Ok(None)
    }

    fn incident_response(incident: Incident) -> IncidentResponse {
        IncidentResponse {
            id: incident.id,
            camera_id: incident.camera_id,
            timestamp: incident.timestamp,
            severity: IncidentSeverity::from_db_str(&incident.severity)
                .unwrap_or(IncidentSeverity::Medium),
            status: IncidentStatus::from_db_str(&incident.status).unwrap_or(IncidentStatus::Open),
            event_type: incident.event_type,
            confidence: incident.confidence,
            bounding_box: incident
                .bounding_box
                .and_then(|v| serde_json::from_value(v).ok()),
            metadata: incident.metadata,
            created_at: incident.created_at,
            updated_at: incident.updated_at,
        }
    }

    // -----------------------------------------------------------------------
    // Validation
    // -----------------------------------------------------------------------

    fn validate_create(req: &CreateIncidentRequest) -> Result<()> {
        Self::validate_event_type(&req.event_type)?;
        Self::validate_confidence(req.confidence)?;
        Ok(())
    }

    fn validate_event_type(event_type: &str) -> Result<()> {
        let trimmed = event_type.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("event_type is required"));
        }
        if trimmed.len() > 100 {
            return Err(anyhow!(
                "event_type must be at most 100 characters, got {}",
                trimmed.len()
            ));
        }
        Ok(())
    }

    fn validate_confidence(confidence: f64) -> Result<()> {
        if !(0.0..=1.0).contains(&confidence) {
            return Err(anyhow!(
                "confidence must be between 0.0 and 1.0, got {confidence}"
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_event_type_empty() {
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_event_type("");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("event_type is required"));
    }

    #[test]
    fn test_validate_event_type_whitespace() {
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_event_type("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_event_type_too_long() {
        let long_type = "a".repeat(101);
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_event_type(&long_type);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at most 100"));
    }

    #[test]
    fn test_validate_event_type_valid() {
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_event_type("person_detected");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_confidence_negative() {
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(-0.1);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("between 0.0 and 1.0"));
    }

    #[test]
    fn test_validate_confidence_over_one() {
        let result = IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(1.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_confidence_valid() {
        assert!(IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(0.0)
        .is_ok());
        assert!(IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(0.5)
        .is_ok());
        assert!(IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(1.0)
        .is_ok());
    }

    #[test]
    fn test_validate_confidence_boundaries() {
        assert!(IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(0.0)
        .is_ok());
        assert!(IncidentService::<
            crate::repository::incident_repository::PostgresIncidentRepository,
        >::validate_confidence(1.0)
        .is_ok());
    }
}
