use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::info;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    info!("connecting to postgres");

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .context("failed to connect to postgres")?;

    info!("connected to postgres");

    Ok(pool)
}
