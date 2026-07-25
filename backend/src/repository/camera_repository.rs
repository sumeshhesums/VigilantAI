use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::models::{Camera, CreateCamera, UpdateCamera};
use crate::repository::CameraRepository;

pub struct PostgresCameraRepository;

#[async_trait]
impl CameraRepository for PostgresCameraRepository {
    async fn create(&self, pool: &PgPool, camera: &CreateCamera) -> anyhow::Result<Camera> {
        let record = sqlx::query_as::<_, Camera>(
            "INSERT INTO cameras (name, location, rtsp_url, fps, resolution) \
             VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(&camera.name)
        .bind(&camera.location)
        .bind(&camera.rtsp_url)
        .bind(camera.fps)
        .bind(&camera.resolution)
        .fetch_one(pool)
        .await?;

        Ok(record)
    }

    async fn find_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>("SELECT * FROM cameras WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn find_by_name(&self, pool: &PgPool, name: &str) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>("SELECT * FROM cameras WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn find_by_rtsp_url(
        &self,
        pool: &PgPool,
        rtsp_url: &str,
    ) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>("SELECT * FROM cameras WHERE rtsp_url = $1")
            .bind(rtsp_url)
            .fetch_optional(pool)
            .await?;

        Ok(record)
    }

    async fn list(&self, pool: &PgPool) -> anyhow::Result<Vec<Camera>> {
        let records = sqlx::query_as::<_, Camera>("SELECT * FROM cameras ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;

        Ok(records)
    }

    async fn list_paginated(
        &self,
        pool: &PgPool,
        offset: i64,
        limit: i64,
    ) -> anyhow::Result<Vec<Camera>> {
        let records = sqlx::query_as::<_, Camera>(
            "SELECT * FROM cameras ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(records)
    }

    async fn update(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
        camera: &UpdateCamera,
    ) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>(
            "UPDATE cameras \
             SET name = COALESCE($2, name), \
                 location = $3, \
                 rtsp_url = COALESCE($4, rtsp_url), \
                 fps = $5, \
                 resolution = $6, \
                 enabled = COALESCE($7, enabled), \
                 updated_at = NOW() \
             WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .bind(&camera.name)
        .bind(&camera.location)
        .bind(&camera.rtsp_url)
        .bind(camera.fps)
        .bind(&camera.resolution)
        .bind(camera.enabled)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<bool> {
        let result = sqlx::query("DELETE FROM cameras WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn enable(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>(
            "UPDATE cameras SET enabled = TRUE, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn disable(&self, pool: &PgPool, id: uuid::Uuid) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>(
            "UPDATE cameras SET enabled = FALSE, updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn update_last_seen(
        &self,
        pool: &PgPool,
        id: uuid::Uuid,
    ) -> anyhow::Result<Option<Camera>> {
        let record = sqlx::query_as::<_, Camera>(
            "UPDATE cameras SET last_seen = NOW(), updated_at = NOW() WHERE id = $1 RETURNING *",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(record)
    }

    async fn count(&self, pool: &PgPool) -> anyhow::Result<i64> {
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cameras")
            .fetch_one(pool)
            .await?;
        Ok(row.0)
    }
}
