use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::json;
use sqlx::postgres::PgPool;
use tower::ServiceExt;

use backend::app;
use backend::config::database::DatabaseConfig;
use backend::config::jwt::JwtConfig;
use backend::config::redis::RedisConfig;
use backend::config::server::ServerConfig;
use backend::config::AppConfig;
use backend::security::Security;
use backend::state::AppState;

fn test_jwt_config() -> JwtConfig {
    let private_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_private_key.pem"
    ))
    .expect("failed to read test private key");

    let public_key = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_public_key.pem"
    ))
    .expect("failed to read test public key");

    JwtConfig {
        private_key,
        public_key,
        access_token_expiry_secs: 900,
        refresh_token_expiry_secs: 604800,
    }
}

async fn setup() -> (PgPool, AppState) {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/vigilantai".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to test database — is PostgreSQL running?");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    sqlx::query("DELETE FROM user_roles")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users").execute(&pool).await.ok();
    sqlx::query("DELETE FROM roles").execute(&pool).await.ok();

    let config = AppConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
        },
        database: DatabaseConfig { url: database_url },
        redis: RedisConfig {
            url: "redis://127.0.0.1:6379".to_string(),
        },
        jwt: test_jwt_config(),
    };

    let redis_client =
        redis::Client::open(config.redis.url.clone()).expect("failed to create redis client");

    let security = Security::from_config(&config.jwt).expect("failed to create security");

    let state = AppState {
        config,
        postgres_pool: pool.clone(),
        redis_client,
        security,
    };

    (pool, state)
}

// ─── Register ────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_register_success() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let body = json!({
        "email": "test_register@example.com",
        "password": "password123",
        "first_name": "Test",
        "last_name": "User"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["email"], "test_register@example.com");
    assert_eq!(json["first_name"], "Test");
    assert_eq!(json["last_name"], "User");
    assert_eq!(json["role"], "user");
    assert!(json["id"].is_string());
    assert!(json.get("password_hash").is_none());
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_register_duplicate_email() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let body = json!({
        "email": "duplicate@example.com",
        "password": "password123",
        "first_name": "First",
        "last_name": "User"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// ─── Login ───────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_login_success() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let register_body = json!({
        "email": "login_test@example.com",
        "password": "secure_password",
        "first_name": "Login",
        "last_name": "Tester"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": "login_test@example.com",
        "password": "secure_password"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["access_token"].is_string());
    assert!(json["refresh_token"].is_string());
    assert_eq!(json["token_type"], "Bearer");
    assert!(json["expires_in"].is_number());
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_login_invalid_password() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let register_body = json!({
        "email": "invalid_pw@example.com",
        "password": "correct_password",
        "first_name": "Invalid",
        "last_name": "PW"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": "invalid_pw@example.com",
        "password": "wrong_password"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("invalid credentials"));
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_login_nonexistent_user() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let login_body = json!({
        "email": "nonexistent@example.com",
        "password": "any_password"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ─── Refresh ─────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_refresh_token() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let register_body = json!({
        "email": "refresh_test@example.com",
        "password": "refresh_pass",
        "first_name": "Refresh",
        "last_name": "Tester"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": "refresh_test@example.com",
        "password": "refresh_pass"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let refresh_token = json["refresh_token"].as_str().unwrap().to_string();

    let refresh_body = json!({
        "refresh_token": refresh_token
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/refresh")
                .header("content-type", "application/json")
                .body(Body::from(refresh_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["access_token"].is_string());
    assert!(json["refresh_token"].is_string());
    assert_eq!(json["token_type"], "Bearer");
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_refresh_invalid_token() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let refresh_body = json!({
        "refresh_token": "invalid.token.here"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/refresh")
                .header("content-type", "application/json")
                .body(Body::from(refresh_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ─── Me ──────────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_me_endpoint() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let register_body = json!({
        "email": "me_test@example.com",
        "password": "me_password",
        "first_name": "Me",
        "last_name": "Tester"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": "me_test@example.com",
        "password": "me_password"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let access_token = json["access_token"].as_str().unwrap().to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("Authorization", format!("Bearer {access_token}"))
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
    assert_eq!(json["email"], "me_test@example.com");
    assert_eq!(json["first_name"], "Me");
    assert_eq!(json["last_name"], "Tester");
    assert_eq!(json["role"], "user");
    assert!(json.get("password_hash").is_none());
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_me_without_token() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_me_with_invalid_token() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/auth/me")
                .header("Authorization", "Bearer invalid.token.value")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ─── Logout ──────────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test auth_tests -- --ignored"]
async fn test_logout() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let register_body = json!({
        "email": "logout_test@example.com",
        "password": "logout_pass",
        "first_name": "Logout",
        "last_name": "Tester"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(register_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let login_body = json!({
        "email": "logout_test@example.com",
        "password": "logout_pass"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let access_token = json["access_token"].as_str().unwrap().to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout")
                .header("Authorization", format!("Bearer {access_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

// ─── Validation ──────────────────────────────────────────

#[tokio::test]
async fn test_register_validation_missing_fields() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/vigilantai".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy(&database_url)
        .expect("failed to create lazy pool");

    let config = AppConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
        },
        database: DatabaseConfig { url: database_url },
        redis: RedisConfig {
            url: "redis://127.0.0.1:6379".to_string(),
        },
        jwt: test_jwt_config(),
    };
    let redis_client =
        redis::Client::open(config.redis.url.clone()).expect("failed to create redis client");
    let security = Security::from_config(&config.jwt).expect("failed to create security");
    let state = AppState {
        config,
        postgres_pool: pool,
        redis_client,
        security,
    };
    let app = app::router(state);

    let body = json!({
        "email": "test@example.com"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_login_validation_missing_fields() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/vigilantai".to_string());

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy(&database_url)
        .expect("failed to create lazy pool");

    let config = AppConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
        },
        database: DatabaseConfig { url: database_url },
        redis: RedisConfig {
            url: "redis://127.0.0.1:6379".to_string(),
        },
        jwt: test_jwt_config(),
    };
    let redis_client =
        redis::Client::open(config.redis.url.clone()).expect("failed to create redis client");
    let security = Security::from_config(&config.jwt).expect("failed to create security");
    let state = AppState {
        config,
        postgres_pool: pool,
        redis_client,
        security,
    };
    let app = app::router(state);

    let body = json!({
        "email": "test@example.com"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
