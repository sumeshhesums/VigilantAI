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
use backend::security::jwt::create_access_token;
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
    sqlx::query("DELETE FROM role_permissions")
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM cameras").execute(&pool).await.ok();
    sqlx::query("DELETE FROM users").execute(&pool).await.ok();
    sqlx::query("DELETE FROM permissions")
        .execute(&pool)
        .await
        .ok();
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

/// Seed roles and permissions into the database, then assign a role to a user.
async fn seed_and_assign_role(pool: &PgPool, user_id: uuid::Uuid, role_name: &str) {
    // Ensure the role exists
    sqlx::query(
        "INSERT INTO roles (name, description) VALUES ($1, $2) ON CONFLICT (name) DO NOTHING",
    )
    .bind(role_name)
    .bind(format!("{role_name} role for tests"))
    .execute(pool)
    .await
    .expect("failed to insert role");

    // Assign the role to the user
    sqlx::query(
        "INSERT INTO user_roles (user_id, role_id) \
         SELECT $1, id FROM roles WHERE name = $2 \
         ON CONFLICT DO NOTHING",
    )
    .bind(user_id)
    .bind(role_name)
    .execute(pool)
    .await
    .expect("failed to assign role");
}

/// Register a user and return their ID.
async fn register_user(_pool: &PgPool, app: &axum::Router, email: &str) -> uuid::Uuid {
    let body = json!({
        "email": email,
        "password": "password123",
        "first_name": "Test",
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    uuid::Uuid::parse_str(json["id"].as_str().unwrap()).unwrap()
}

/// Create a JWT for the given user.
fn make_token(state: &AppState, user_id: uuid::Uuid, email: &str, role: &str) -> String {
    create_access_token(
        user_id,
        email,
        role,
        state.security.access_token_expiry_secs,
        &state.security.encoding_key,
    )
    .expect("failed to create token")
}

// ===========================================================================
// List cameras — GET /api/v1/cameras
// ===========================================================================

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_list_cameras_unauthorized() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/cameras")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_list_cameras_forbidden_viewer_role_not_assigned() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    // Register a user (no roles assigned)
    let user_id = register_user(&pool, &app, "forbidden_list@example.com").await;
    let token = make_token(&state, user_id, "forbidden_list@example.com", "user");

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_list_cameras_allowed_viewer() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "viewer_list@example.com").await;
    seed_and_assign_role(&pool, user_id, "viewer").await;
    let token = make_token(&state, user_id, "viewer_list@example.com", "viewer");

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_list_cameras_allowed_system_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "admin_list@example.com").await;
    seed_and_assign_role(&pool, user_id, "system_admin").await;
    let token = make_token(&state, user_id, "admin_list@example.com", "system_admin");

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// ===========================================================================
// Create camera — POST /api/v1/cameras
// ===========================================================================

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_create_camera_unauthorized() {
    let (_pool, state) = setup().await;
    let app = app::router(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name": "test", "rtsp_url": "rtsp://10.0.0.1/stream"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_create_camera_forbidden_operator() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "operator_create@example.com").await;
    seed_and_assign_role(&pool, user_id, "operator").await;
    let token = make_token(&state, user_id, "operator_create@example.com", "operator");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name": "test", "rtsp_url": "rtsp://10.0.0.1/stream"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_create_camera_allowed_security_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "secadmin_create@example.com").await;
    seed_and_assign_role(&pool, user_id, "security_admin").await;
    let token = make_token(
        &state,
        user_id,
        "secadmin_create@example.com",
        "security_admin",
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Lobby Camera",
                        "rtsp_url": "rtsp://10.0.0.1:554/stream"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

// ===========================================================================
// Update camera — PATCH /api/v1/cameras/:id
// ===========================================================================

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_update_camera_forbidden_operator() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "operator_update@example.com").await;
    seed_and_assign_role(&pool, user_id, "operator").await;
    let token = make_token(&state, user_id, "operator_update@example.com", "operator");

    let fake_id = uuid::Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/cameras/{fake_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({"name": "updated"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_update_camera_allowed_system_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "admin_update@example.com").await;
    seed_and_assign_role(&pool, user_id, "system_admin").await;
    let token = make_token(&state, user_id, "admin_update@example.com", "system_admin");

    // Create a camera first
    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Camera To Update",
                        "rtsp_url": "rtsp://10.0.0.2:554/stream"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let camera_id = json["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!("/api/v1/cameras/{camera_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(json!({"name": "Updated Camera"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["name"], "Updated Camera");
}

// ===========================================================================
// Delete camera — DELETE /api/v1/cameras/:id
// ===========================================================================

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_delete_camera_forbidden_security_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "secadmin_delete@example.com").await;
    seed_and_assign_role(&pool, user_id, "security_admin").await;
    let token = make_token(
        &state,
        user_id,
        "secadmin_delete@example.com",
        "security_admin",
    );

    let fake_id = uuid::Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/cameras/{fake_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_delete_camera_allowed_system_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "admin_delete@example.com").await;
    seed_and_assign_role(&pool, user_id, "system_admin").await;
    let token = make_token(&state, user_id, "admin_delete@example.com", "system_admin");

    // Create a camera to delete
    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Camera To Delete",
                        "rtsp_url": "rtsp://10.0.0.3:554/stream"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let camera_id = json["id"].as_str().unwrap();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/cameras/{camera_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/api/v1/cameras/{camera_id}"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ===========================================================================
// Enable/Disable camera — POST /api/v1/cameras/:id/enable|disable
// ===========================================================================

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_enable_camera_forbidden_viewer() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "viewer_enable@example.com").await;
    seed_and_assign_role(&pool, user_id, "viewer").await;
    let token = make_token(&state, user_id, "viewer_enable@example.com", "viewer");

    let fake_id = uuid::Uuid::new_v4();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/cameras/{fake_id}/enable"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_enable_camera_allowed_operator() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "operator_enable@example.com").await;
    seed_and_assign_role(&pool, user_id, "operator").await;
    let token = make_token(&state, user_id, "operator_enable@example.com", "operator");

    // Create a camera (need system_admin for that)
    let admin_id = register_user(&pool, &app, "admin_for_enable@example.com").await;
    seed_and_assign_role(&pool, admin_id, "system_admin").await;
    let admin_token = make_token(
        &state,
        admin_id,
        "admin_for_enable@example.com",
        "system_admin",
    );

    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Enable Test Camera",
                        "rtsp_url": "rtsp://10.0.0.4:554/stream"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let camera_id = json["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/cameras/{camera_id}/enable"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
#[ignore = "requires PostgreSQL"]
async fn test_disable_camera_allowed_security_admin() {
    let (pool, state) = setup().await;
    let app = app::router(state.clone());

    let user_id = register_user(&pool, &app, "secadmin_disable@example.com").await;
    seed_and_assign_role(&pool, user_id, "security_admin").await;
    let token = make_token(
        &state,
        user_id,
        "secadmin_disable@example.com",
        "security_admin",
    );

    // Create a camera first (as system_admin)
    let admin_id = register_user(&pool, &app, "admin_for_disable@example.com").await;
    seed_and_assign_role(&pool, admin_id, "system_admin").await;
    let admin_token = make_token(
        &state,
        admin_id,
        "admin_for_disable@example.com",
        "system_admin",
    );

    let create_resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/cameras")
                .header("Authorization", format!("Bearer {admin_token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "name": "Disable Test Camera",
                        "rtsp_url": "rtsp://10.0.0.5:554/stream"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let body = axum::body::to_bytes(create_resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let camera_id = json["id"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/v1/cameras/{camera_id}/disable"))
                .header("Authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
