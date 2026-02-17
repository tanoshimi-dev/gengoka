mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;

// ─── GET /users/me/social-accounts ────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_linked_accounts_empty() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "link@example.com", "password123", "LinkUser").await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/social-accounts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[actix_web::test]
#[serial]
async fn test_get_linked_accounts_with_data() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, token) =
        common::register_and_login(&app, "link2@example.com", "password123", "Link2").await;

    // Insert social account directly
    common::insert_social_account(&pool, user_id, "google", "google-user-123").await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/social-accounts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let accounts = body["data"].as_array().unwrap();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0]["provider"], "google");
    assert!(accounts[0]["linked_at"].is_string());
}

#[actix_web::test]
#[serial]
async fn test_get_linked_accounts_unauthorized() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/social-accounts")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

// ─── DELETE /users/me/social-accounts/{provider} ──────────

#[actix_web::test]
#[serial]
async fn test_unlink_account_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // User with password + one social account => can unlink
    let (user_id, token) =
        common::register_and_login(&app, "unlink@example.com", "password123", "UnlinkUser").await;
    common::insert_social_account(&pool, user_id, "google", "g-unlink-1").await;

    let req = test::TestRequest::delete()
        .uri("/api/v1/users/me/social-accounts/google")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);

    // Verify it's gone
    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/social-accounts")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[actix_web::test]
#[serial]
async fn test_unlink_last_method_rejected() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // Create a social-only user (no password) directly in DB
    let user_id: (uuid::Uuid,) = sqlx::query_as(
        r#"INSERT INTO users (name, status) VALUES ('SocialOnly', 'active') RETURNING id"#,
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let user_id = user_id.0;

    common::insert_social_account(&pool, user_id, "google", "g-only-1").await;

    // Generate a token for this user using the config
    let config = common::test_config();
    let token = gengoka_backend::utils::generate_access_token(&user_id, &config.auth).unwrap();

    // Try to unlink the only auth method
    let req = test::TestRequest::delete()
        .uri("/api/v1/users/me/social-accounts/google")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
#[serial]
async fn test_unlink_account_not_found() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "nounlink@example.com", "password123", "NoUnlink").await;

    let req = test::TestRequest::delete()
        .uri("/api/v1/users/me/social-accounts/apple")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}
