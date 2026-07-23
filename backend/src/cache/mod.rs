use anyhow::{Context, Result};
use redis::Client;
use tracing::info;

pub async fn create_client(redis_url: &str) -> Result<Client> {
    info!("connecting to redis");

    let client = Client::open(redis_url.to_string())
        .context("failed to create redis client")?;

    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .context("failed to connect to redis")?;

    let pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await
        .context("redis PING failed")?;

    info!(pong, "connected to redis");

    Ok(client)
}
