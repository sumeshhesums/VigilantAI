use axum::routing::get;
use axum::Json;
use axum::Router;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    service: &'static str,
    version: &'static str,
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "backend",
        version: env!("CARGO_PKG_VERSION"),
    })
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use sqlx::postgres::PgPoolOptions;
    use tower::ServiceExt;

    use crate::app;
    use crate::config::AppConfig;
    use crate::config::database::DatabaseConfig;
    use crate::config::redis::RedisConfig;
    use crate::config::server::ServerConfig;

    fn test_state() -> crate::state::AppState {
        let config = AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            database: DatabaseConfig {
                url: "postgres://localhost:5432/test".to_string(),
            },
            redis: RedisConfig {
                url: "redis://127.0.0.1:6379".to_string(),
            },
        };

        // connect_lazy creates a pool that does NOT connect until first query
        let postgres_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy(&config.database.url)
            .expect("failed to create lazy pool");

        // Client::open does NOT connect until first command
        let redis_client = redis::Client::open(config.redis.url.clone())
            .expect("failed to create redis client");

        crate::state::AppState {
            config,
            postgres_pool,
            redis_client,
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let state = test_state();
        let app = app::router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
        assert_eq!(json["service"], "backend");
    }
}
