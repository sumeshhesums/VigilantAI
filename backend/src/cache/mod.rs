use anyhow::{Context, Result};
use redis::Client;
use tracing::info;

pub async fn create_client(redis_url: &str) -> Result<Client> {
    info!("connecting to redis");

    let client = Client::open(redis_url.to_string()).context("failed to create redis client")?;

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

/// Blacklist a JWT token in Redis using its JTI claim.
///
/// The key expires automatically after `expiry_secs` seconds, matching the
/// token's remaining lifetime.
pub async fn blacklist_token(client: &Client, jti: &str, expiry_secs: u64) -> Result<()> {
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .context("failed to connect to redis")?;

    let key = format!("blacklist:{}", jti);
    redis::cmd("SETEX")
        .arg(&key)
        .arg(expiry_secs)
        .arg("1")
        .query_async::<_, ()>(&mut conn)
        .await
        .context("failed to blacklist token in redis")?;

    Ok(())
}

/// Check whether a JWT token (by JTI) has been blacklisted.
pub async fn is_token_blacklisted(client: &Client, jti: &str) -> Result<bool> {
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .context("failed to connect to redis")?;

    let key = format!("blacklist:{}", jti);
    let exists: bool = redis::cmd("EXISTS")
        .arg(&key)
        .query_async(&mut conn)
        .await
        .context("failed to check token blacklist")?;

    Ok(exists)
}
