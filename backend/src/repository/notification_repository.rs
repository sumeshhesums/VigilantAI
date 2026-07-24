use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::dto::notification::NotificationPaginationParams;
use crate::models::{CreateNotification, Notification};

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(
        &self,
        pool: &PgPool,
        notification: &CreateNotification,
    ) -> anyhow::Result<Notification>;
    async fn find_by_id(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> anyhow::Result<Option<Notification>>;
    async fn list(
        &self,
        pool: &PgPool,
        params: &NotificationPaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Notification>>;
    async fn count(
        &self,
        pool: &PgPool,
        params: &NotificationPaginationParams,
    ) -> anyhow::Result<i64>;
    async fn update_status(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        status: &str,
        attempts: i32,
        response_code: Option<i32>,
        error_message: Option<&str>,
    ) -> anyhow::Result<Option<Notification>>;
    async fn find_retryable(
        &self,
        pool: &PgPool,
        max_attempts: i32,
    ) -> anyhow::Result<Vec<Notification>>;
}

pub struct PostgresNotificationRepository;

#[async_trait]
impl NotificationRepository for PostgresNotificationRepository {
    async fn create(
        &self,
        pool: &PgPool,
        notification: &CreateNotification,
    ) -> anyhow::Result<Notification> {
        let record = sqlx::query_as::<_, Notification>(
            "INSERT INTO notifications (incident_id, channel, recipient) \
             VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(notification.incident_id)
        .bind(notification.channel.as_db_str())
        .bind(&notification.recipient)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> anyhow::Result<Option<Notification>> {
        let record = sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(record)
    }

    async fn list(
        &self,
        pool: &PgPool,
        params: &NotificationPaginationParams,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Notification>> {
        let mut query = String::from("SELECT * FROM notifications WHERE 1=1");

        if params.status.is_some() {
            query.push_str(" AND status = $N");
        }
        if params.channel.is_some() {
            query.push_str(" AND channel = $N");
        }
        if params.incident_id.is_some() {
            query.push_str(" AND incident_id = $N");
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" OFFSET ${} LIMIT ${}", "O", "L"));

        // Use simple parameterized query instead of dynamic building
        let records = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications \
             WHERE ($1::text IS NULL OR status = $1) \
             AND ($2::text IS NULL OR channel = $2) \
             AND ($3::uuid IS NULL OR incident_id = $3) \
             ORDER BY created_at DESC \
             OFFSET $4 LIMIT $5",
        )
        .bind(params.status.as_ref().map(|s| s.as_db_str()))
        .bind(params.channel.as_ref().map(|c| c.as_db_str()))
        .bind(params.incident_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    async fn count(
        &self,
        pool: &PgPool,
        params: &NotificationPaginationParams,
    ) -> anyhow::Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM notifications \
             WHERE ($1::text IS NULL OR status = $1) \
             AND ($2::text IS NULL OR channel = $2) \
             AND ($3::uuid IS NULL OR incident_id = $3)",
        )
        .bind(params.status.as_ref().map(|s| s.as_db_str()))
        .bind(params.channel.as_ref().map(|c| c.as_db_str()))
        .bind(params.incident_id)
        .fetch_one(pool)
        .await?;

        Ok(count)
    }

    async fn update_status(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        status: &str,
        attempts: i32,
        response_code: Option<i32>,
        error_message: Option<&str>,
    ) -> anyhow::Result<Option<Notification>> {
        let sent_at = if status == "sent" {
            Some(chrono::Utc::now())
        } else {
            None
        };

        let record = sqlx::query_as::<_, Notification>(
            "UPDATE notifications \
             SET status = $2, attempts = $3, response_code = $4, error_message = $5, sent_at = $6 \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(status)
        .bind(attempts)
        .bind(response_code)
        .bind(error_message)
        .bind(sent_at)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn find_retryable(
        &self,
        pool: &PgPool,
        max_attempts: i32,
    ) -> anyhow::Result<Vec<Notification>> {
        let records = sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications \
             WHERE (status = 'pending' OR status = 'retrying') \
             AND attempts < $1 \
             ORDER BY created_at ASC",
        )
        .bind(max_attempts)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use crate::dto::notification::NotificationPaginationParams;

    #[test]
    fn test_default_pagination_params() {
        let params = NotificationPaginationParams {
            page: None,
            per_page: None,
            status: None,
            channel: None,
            incident_id: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 0);
        assert_eq!(limit, 20);
    }

    #[test]
    fn test_custom_pagination_params() {
        let params = NotificationPaginationParams {
            page: Some(5),
            per_page: Some(10),
            status: None,
            channel: None,
            incident_id: None,
        };
        let (offset, limit) = params.offset_limit();
        assert_eq!(offset, 40);
        assert_eq!(limit, 10);
    }

    #[test]
    fn test_max_per_page() {
        let params = NotificationPaginationParams {
            page: Some(1),
            per_page: Some(500),
            status: None,
            channel: None,
            incident_id: None,
        };
        let (_, limit) = params.offset_limit();
        assert_eq!(limit, 100);
    }
}
