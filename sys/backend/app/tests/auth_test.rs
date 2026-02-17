mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;

// ─── Register ───────────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_register_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (status, body) = common::register_test_user(
        &app,
        "alice@example.com",
        "password123",
        "Alice",
    )
    .await;

    assert_eq!(status, 201);
    assert_eq!(body["success"], true);
    assert!(body["data"]["access_token"].is_string());
    assert!(body["data"]["refresh_token"].is_string());
    assert!(body["data"]["expires_in"].is_number());
    assert_eq!(body["data"]["user"]["name"], "Alice");
}

#[actix_web::test]
#[serial]
async fn test_register_duplicate_email() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // First registration should succeed
    let (status, _) = common::register_test_user(
        &app,
        "dup@example.com",
        "password123",
        "User1",
    )
    .await;
    assert_eq!(status, 201);

    // Second registration with same email should fail with 409
    let (status, body) = common::register_test_user(
        &app,
        "dup@example.com",
        "password456",
        "User2",
    )
    .await;
    assert_eq!(status, 409);
    assert_eq!(body["success"], false);
}

#[actix_web::test]
#[serial]
async fn test_register_invalid_email() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (status, body) = common::register_test_user(
        &app,
        "not-an-email",
        "password123",
        "BadEmail",
    )
    .await;

    assert_eq!(status, 400);
    assert_eq!(body["success"], false);
}

#[actix_web::test]
#[serial]
async fn test_register_short_password() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (status, body) = common::register_test_user(
        &app,
        "short@example.com",
        "short",
        "ShortPw",
    )
    .await;

    assert_eq!(status, 400);
    assert_eq!(body["success"], false);
}

// ─── Login ──────────────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_login_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // Register first
    common::register_test_user(&app, "login@example.com", "password123", "LoginUser").await;

    // Login
    let (status, body) = common::login_test_user(&app, "login@example.com", "password123").await;

    assert_eq!(status, 200);
    assert_eq!(body["success"], true);
    assert!(body["data"]["access_token"].is_string());
    assert!(body["data"]["refresh_token"].is_string());
    assert_eq!(body["data"]["user"]["name"], "LoginUser");
}

#[actix_web::test]
#[serial]
async fn test_login_wrong_password() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // Register first
    common::register_test_user(&app, "wrongpw@example.com", "password123", "WrongPw").await;

    // Login with wrong password
    let (status, body) =
        common::login_test_user(&app, "wrongpw@example.com", "wrongpassword").await;

    assert_eq!(status, 401);
    assert_eq!(body["success"], false);
}

#[actix_web::test]
#[serial]
async fn test_login_nonexistent_user() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (status, body) =
        common::login_test_user(&app, "nobody@example.com", "password123").await;

    assert_eq!(status, 401);
    assert_eq!(body["success"], false);
}

// ─── Refresh ────────────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_refresh_token_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // Register to get a refresh token
    let (_, reg_body) = common::register_test_user(
        &app,
        "refresh@example.com",
        "password123",
        "RefreshUser",
    )
    .await;
    let refresh_token = reg_body["data"]["refresh_token"].as_str().unwrap();

    // Use the refresh token
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .set_json(serde_json::json!({
            "refresh_token": refresh_token,
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["access_token"].is_string());
    assert!(body["data"]["refresh_token"].is_string());

    // The new refresh token should be different (rotation)
    let new_refresh = body["data"]["refresh_token"].as_str().unwrap();
    assert_ne!(new_refresh, refresh_token);
}

#[actix_web::test]
#[serial]
async fn test_refresh_token_invalid() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .set_json(serde_json::json!({
            "refresh_token": "nonexistent-refresh-token",
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], false);
}

// ─── Full Auth Flow ─────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_full_auth_flow() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // 1. Register
    let (status, reg_body) = common::register_test_user(
        &app,
        "flow@example.com",
        "password123",
        "FlowUser",
    )
    .await;
    assert_eq!(status, 201);
    let access_token = reg_body["data"]["access_token"].as_str().unwrap().to_string();
    let refresh_token = reg_body["data"]["refresh_token"].as_str().unwrap().to_string();

    // 2. Login
    let (status, login_body) =
        common::login_test_user(&app, "flow@example.com", "password123").await;
    assert_eq!(status, 200);
    assert_eq!(
        login_body["data"]["user"]["name"].as_str().unwrap(),
        "FlowUser"
    );

    // 3. Refresh
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .set_json(serde_json::json!({
            "refresh_token": refresh_token,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);
    let refresh_body: serde_json::Value = test::read_body_json(resp).await;
    let new_refresh_token = refresh_body["data"]["refresh_token"]
        .as_str()
        .unwrap()
        .to_string();

    // Old refresh token should no longer work (rotation)
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .set_json(serde_json::json!({
            "refresh_token": refresh_token,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);

    // 4. Logout (with specific refresh token)
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/logout")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(serde_json::json!({
            "refresh_token": new_refresh_token,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);

    // 5. The logged-out refresh token should be invalid
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/refresh")
        .set_json(serde_json::json!({
            "refresh_token": new_refresh_token,
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}
