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

    // Clean up
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

/// Seed a system_admin user and return (pool, state, access_token, user_id).
async fn setup_admin() -> (PgPool, AppState, String, String) {
    let (pool, state) = setup().await;

    // Register user
    let body = json!({
        "email": "admin_user_mgmt@example.com",
        "password": "secure_password123",
        "first_name": "Admin",
        "last_name": "UserMgmt"
    });
    let app = app::router(state.clone());
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
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let user_id = json["id"].as_str().unwrap().to_string();

    // Insert system_admin role into roles table (if not exists)
    sqlx::query(
        "INSERT INTO roles (name, description) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
    )
    .bind("system_admin")
    .bind("Full system administrator")
    .execute(&pool)
    .await
    .unwrap();

    // Assign role
    sqlx::query("INSERT INTO user_roles (user_id, role_id) SELECT $1, r.id FROM roles r WHERE r.name = $2 ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&user_id).unwrap())
        .bind("system_admin")
        .execute(&pool)
        .await
        .unwrap();

    // Login
    let login_body = json!({
        "email": "admin_user_mgmt@example.com",
        "password": "secure_password123"
    });
    let app = app::router(state.clone());
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
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let access_token = json["access_token"].as_str().unwrap().to_string();

    (pool, state, access_token, user_id)
}

/// Seed a viewer user (non-admin) and return access_token.
async fn setup_viewer(state: &AppState, pool: &PgPool) -> String {
    let body = json!({
        "email": "viewer_user_mgmt@example.com",
        "password": "viewer_pass123",
        "first_name": "Viewer",
        "last_name": "UserMgmt"
    });
    let app = app::router(state.clone());
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
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let user_id = json["id"].as_str().unwrap().to_string();

    // Insert viewer role
    sqlx::query(
        "INSERT INTO roles (name, description) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
    )
    .bind("viewer")
    .bind("Read-only viewer")
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO user_roles (user_id, role_id) SELECT $1, r.id FROM roles r WHERE r.name = $2 ON CONFLICT DO NOTHING")
        .bind(uuid::Uuid::parse_str(&user_id).unwrap())
        .bind("viewer")
        .execute(pool)
        .await
        .unwrap();

    // Login
    let login_body = json!({
        "email": "viewer_user_mgmt@example.com",
        "password": "viewer_pass123"
    });
    let app = app::router(state.clone());
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
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    json["access_token"].as_str().unwrap().to_string()
}

// ─── Unauthorized ──────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_list_users_unauthorized() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/users")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// ─── Forbidden (viewer role) ───────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_list_users_forbidden_viewer() {
    let (_pool, state) = setup().await;
    let token = setup_viewer(&state, &_pool).await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

// ─── Create User ───────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_create_user_success() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state);

    let body = json!({
        "email": "new_created@example.com",
        "password": "strong_pass123",
        "first_name": "New",
        "last_name": "Created"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert_eq!(json["email"], "new_created@example.com");
    assert_eq!(json["first_name"], "New");
    assert_eq!(json["last_name"], "Created");
    assert!(json["id"].is_string());
    assert!(json.get("password_hash").is_none());
    assert_eq!(json["is_active"], true);
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_create_user_duplicate_email() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state.clone());

    let body = json!({
        "email": "duplicate_um@example.com",
        "password": "strong_pass123",
        "first_name": "Dup",
        "last_name": "User"
    });

    // First create — should succeed
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Second create — should conflict
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// ─── List Users ────────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_list_users() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert!(json["users"].is_array());
    assert!(json["total"].is_number());
    assert!(json["page"].is_number());
    assert!(json["per_page"].is_number());
}

// ─── Get User By ID ────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_get_user_by_id() {
    let (_pool, state, token, admin_id) = setup_admin().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{admin_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert_eq!(json["email"], "admin_user_mgmt@example.com");
    assert!(json.get("password_hash").is_none());
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_get_user_not_found() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state);
    let fake_id = uuid::Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{fake_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ─── Update User ───────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_update_user() {
    let (_pool, state, token, admin_id) = setup_admin().await;
    let app = app::router(state);

    let body = json!({
        "first_name": "Updated",
        "last_name": "AdminName"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/users/{admin_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    assert_eq!(json["first_name"], "Updated");
    assert_eq!(json["last_name"], "AdminName");
}

// ─── Delete (Deactivate) User ──────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_delete_user() {
    let (pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state.clone());

    // Create a user to delete
    let body = json!({
        "email": "to_delete@example.com",
        "password": "delete_me_pass",
        "first_name": "Delete",
        "last_name": "Me"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let target_id = json["id"].as_str().unwrap();

    // Deactivate
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/users/{target_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify is_active = false
    let row: (bool,) = sqlx::query_as("SELECT is_active FROM users WHERE id = $1")
        .bind(uuid::Uuid::parse_str(target_id).unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!row.0);

    // Second deactivation should conflict
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/users/{target_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// ─── Assign Role ───────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_assign_role() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state.clone());

    // Create a target user
    let body = json!({
        "email": "role_target@example.com",
        "password": "target_pass123",
        "first_name": "Role",
        "last_name": "Target"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let target_id = json["id"].as_str().unwrap();

    // Assign viewer role
    let role_body = json!({ "role": "viewer" });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/users/{target_id}/roles"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(role_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify role assigned via GET user
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{target_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let roles = json["roles"].as_array().unwrap();
    assert!(roles.contains(&json!("viewer")));
}

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_assign_role_unknown_role() {
    let (_pool, state, token, admin_id) = setup_admin().await;
    let app = app::router(state);

    let role_body = json!({ "role": "nonexistent_role" });
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/users/{admin_id}/roles"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(role_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

// ─── Remove Role ───────────────────────────────────────

#[tokio::test]
#[ignore = "requires PostgreSQL — run with: cargo test -p backend --test user_tests -- --ignored"]
async fn test_remove_role() {
    let (_pool, state, token, _admin_id) = setup_admin().await;
    let app = app::router(state.clone());

    // Create a target user
    let body = json!({
        "email": "remove_role_target@example.com",
        "password": "target_pass123",
        "first_name": "Remove",
        "last_name": "Role"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/users")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let target_id = json["id"].as_str().unwrap();

    // Assign viewer role first
    let role_body = json!({ "role": "viewer" });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/users/{target_id}/roles"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(role_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Remove viewer role
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/users/{target_id}/roles"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(role_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify role removed
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/users/{target_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let resp_body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&resp_body).unwrap();
    let roles = json["roles"].as_array().unwrap();
    assert!(!roles.contains(&json!("viewer")));
}

// ─── Pagination Unit Tests (no DB required) ────────────

use backend::dto::user::PaginationParams;

#[test]
fn test_pagination_default() {
    let params = PaginationParams {
        page: None,
        per_page: None,
    };
    let (offset, limit) = params.offset_limit();
    assert_eq!(offset, 0);
    assert_eq!(limit, 20);
}

#[test]
fn test_pagination_page_2() {
    let params = PaginationParams {
        page: Some(2),
        per_page: Some(10),
    };
    let (offset, limit) = params.offset_limit();
    assert_eq!(offset, 10);
    assert_eq!(limit, 10);
}

#[test]
fn test_pagination_clamp_max_per_page() {
    let params = PaginationParams {
        page: Some(1),
        per_page: Some(500),
    };
    let (_, limit) = params.offset_limit();
    assert_eq!(limit, 100);
}

#[test]
fn test_pagination_clamp_min_page() {
    let params = PaginationParams {
        page: Some(0),
        per_page: Some(10),
    };
    let (offset, _) = params.offset_limit();
    assert_eq!(offset, 0);
}
