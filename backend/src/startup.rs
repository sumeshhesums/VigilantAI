use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

use crate::app;
use crate::config::AppConfig;
use crate::state::AppState;

pub async fn run(config: AppConfig) -> Result<()> {
    let state = AppState {
        config: config.clone(),
    };

    let app = app::router(state);
    let listener = TcpListener::bind(&config.address()).await?;

    info!(address = %config.address(), "starting server");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install SIGTERM handler");

        tokio::select! {
            _ = ctrl_c => {
                info!("received SIGINT, shutting down");
            }
            _ = sigterm.recv() => {
                info!("received SIGTERM, shutting down");
            }
        }
    }

    #[cfg(not(unix))]
    {
        match ctrl_c.await {
            Ok(()) => info!("received SIGINT, shutting down"),
            Err(e) => tracing::error!(error = %e, "failed to listen for Ctrl+C"),
        }
    }
}
