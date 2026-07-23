use backend::config::AppConfig;
use backend::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    telemetry::init()?;

    let config = AppConfig::from_env()?;

    tracing::info!(version = env!("CARGO_PKG_VERSION"), "vigilantai backend starting");
    tracing::info!(address = %config.address(), "binding to address");

    backend::startup::run(config).await
}
