use anyhow::Result;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

pub fn init() -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("backend=debug,tower_http=debug"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .init();

    Ok(())
}
