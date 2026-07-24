use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::dto::incident::IncidentPaginationParams;
use crate::models::{CreateIncident, Incident, UpdateIncidentStatus};

#[async_trait]
pub trait IncidentRepository: Send + Sync {
    async fn create(&self, pool: &PgPool, incident: &CreateIncident) -> anyhow::Result<Incident>;
    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Incident>>;
    async fn list(
        &self,
        pool: &PgPool,
        params: &IncidentPaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Incident>>;
    async fn count(&self, pool: &PgPool, params: &IncidentPaginationParams) -> anyhow::Result<i64>;
    async fn update_status(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        update: &UpdateIncidentStatus,
    ) -> anyhow::Result<Option<Incident>>;
    async fn find_similar(
        &self,
        pool: &PgPool,
        camera_id: uuid::Uuid,
        event_type: &str,
        since: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Vec<Incident>>;
}

pub struct PostgresIncidentRepository;

#[async_trait]
impl IncidentRepository for PostgresIncidentRepository {
    async fn create(&self, pool: &PgPool, incident: &CreateIncident) -> anyhow::Result<Incident> {
        let record = sqlx::query_as::<_, Incident>(
            "INSERT INTO incidents (camera_id, timestamp, severity, event_type, confidence, bounding_box, metadata) \
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(incident.camera_id)
        .bind(incident.timestamp)
        .bind(incident.severity.as_db_str())
        .bind(&incident.event_type)
        .bind(incident.confidence)
        .bind(&incident.bounding_box)
        .bind(&incident.metadata)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Incident>> {
        let record = sqlx::query_as::<_, Incident>("SELECT * FROM incidents WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list(
        &self,
        pool: &PgPool,
        params: &IncidentPaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Incident>> {
        let mut query = String::from("SELECT * FROM incidents WHERE 1=1");
        let mut bind_values: Vec<String> = Vec::new();

        if params.camera_id.is_some() {
            query.push_str(" AND camera_id = $");
            bind_values.push("camera_id".to_string());
        }
        if params.severity.is_some() {
            query.push_str(" AND severity = $");
            bind_values.push("severity".to_string());
        }
        if params.status.is_some() {
            query.push_str(" AND status = $");
            bind_values.push("status".to_string());
        }
        if params.event_type.is_some() {
            query.push_str(" AND event_type = $");
            bind_values.push("event_type".to_string());
        }
        if params.since.is_some() {
            query.push_str(" AND timestamp >= $");
            bind_values.push("since".to_string());
        }
        if params.until.is_some() {
            query.push_str(" AND timestamp <= $");
            bind_values.push("until".to_string());
        }

        query.push_str(" ORDER BY timestamp DESC");
        query.push_str(&format!(
            " OFFSET ${} LIMIT ${}",
            bind_values.len() + 1,
            bind_values.len() + 2
        ));

        let mut sql = sqlx::query_as::<_, Incident>(&query);

        if let Some(camera_id) = params.camera_id {
            sql = sql.bind(camera_id);
        }
        if let Some(ref severity) = params.severity {
            sql = sql.bind(severity.as_db_str());
        }
        if let Some(ref status) = params.status {
            sql = sql.bind(status.as_db_str());
        }
        if let Some(ref event_type) = params.event_type {
            sql = sql.bind(event_type);
        }
        if let Some(since) = params.since {
            sql = sql.bind(since);
        }
        if let Some(until) = params.until {
            sql = sql.bind(until);
        }

        let records = sql.bind(offset).bind(limit).fetch_all(pool).await?;

        Ok(records)
    }

    async fn count(&self, pool: &PgPool, params: &IncidentPaginationParams) -> anyhow::Result<i64> {
        let mut query = String::from("SELECT COUNT(*) FROM incidents WHERE 1=1");
        let mut bind_values: Vec<String> = Vec::new();

        if params.camera_id.is_some() {
            query.push_str(" AND camera_id = $");
            bind_values.push("camera_id".to_string());
        }
        if params.severity.is_some() {
            query.push_str(" AND severity = $");
            bind_values.push("severity".to_string());
        }
        if params.status.is_some() {
            query.push_str(" AND status = $");
            bind_values.push("status".to_string());
        }
        if params.event_type.is_some() {
            query.push_str(" AND event_type = $");
            bind_values.push("event_type".to_string());
        }
        if params.since.is_some() {
            query.push_str(" AND timestamp >= $");
            bind_values.push("since".to_string());
        }
        if params.until.is_some() {
            query.push_str(" AND timestamp <= $");
            bind_values.push("until".to_string());
        }

        let mut sql = sqlx::query_scalar::<_, i64>(&query);

        if let Some(camera_id) = params.camera_id {
            sql = sql.bind(camera_id);
        }
        if let Some(ref severity) = params.severity {
            sql = sql.bind(severity.as_db_str());
        }
        if let Some(ref status) = params.status {
            sql = sql.bind(status.as_db_str());
        }
        if let Some(ref event_type) = params.event_type {
            sql = sql.bind(event_type);
        }
        if let Some(since) = params.since {
            sql = sql.bind(since);
        }
        if let Some(until) = params.until {
            sql = sql.bind(until);
        }

        let count = sql.fetch_one(pool).await?;
        Ok(count)
    }

    async fn update_status(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        update: &UpdateIncidentStatus,
    ) -> anyhow::Result<Option<Incident>> {
        let record = sqlx::query_as::<_, Incident>(
            "UPDATE incidents SET status = $2, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(update.status.as_db_str())
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn find_similar(
        &self,
        pool: &PgPool,
        camera_id: uuid::Uuid,
        event_type: &str,
        since: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Vec<Incident>> {
        let records = sqlx::query_as::<_, Incident>(
            "SELECT * FROM incidents \
             WHERE camera_id = $1 AND event_type = $2 AND timestamp >= $3 \
             ORDER BY timestamp DESC",
        )
        .bind(camera_id)
        .bind(event_type)
        .bind(since)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::incident::IncidentPaginationParams;

    #[test]
    fn test_default_pagination_params() {
        let params = IncidentPaginationParams {
            page: None,
            per_page: None,
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_custom_pagination_params() {
        let params = IncidentPaginationParams {
            page: Some(5),
            per_page: Some(10),
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 40);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_max_per_page() {
        let params = IncidentPaginationParams {
            page: Some(1),
            per_page: Some(500),
            camera_id: None,
            severity: None,
            status: None,
            event_type: None,
            since: None,
            until: None,
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }
}
