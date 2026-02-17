mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;
use uuid::Uuid;

// ─── GET /users/me ─────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_me_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, token) =
        common::register_and_login(&app, "me@example.com", "password123", "MeUser").await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["id"], user_id.to_string());
    assert_eq!(body["data"]["name"], "MeUser");
    assert_eq!(body["data"]["email"], "me@example.com");
    assert_eq!(body["data"]["total_challenges"], 0);
    assert_eq!(body["data"]["follower_count"], 0);
    assert_eq!(body["data"]["following_count"], 0);
}

#[actix_web::test]
#[serial]
async fn test_get_me_unauthorized() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

// ─── GET /users/{id} ──────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_user_profile_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, _) =
        common::register_and_login(&app, "profile@example.com", "password123", "ProfileUser")
            .await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/users/{}", user_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["name"], "ProfileUser");
    assert_eq!(body["data"]["is_following"], false);
}

#[actix_web::test]
#[serial]
async fn test_get_user_profile_not_found() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let fake_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/users/{}", fake_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
#[serial]
async fn test_get_user_profile_is_following() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token_a) =
        common::register_and_login(&app, "a@example.com", "password123", "UserA").await;
    let (user_b, _) =
        common::register_and_login(&app, "b@example.com", "password123", "UserB").await;

    // A follows B
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // A views B's profile => is_following = true
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/users/{}", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["is_following"], true);
}

// ─── PUT /users/{id} ──────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_update_user_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, token) =
        common::register_and_login(&app, "upd@example.com", "password123", "OldName").await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/users/{}", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "name": "NewName",
            "bio": "Hello world"
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["name"], "NewName");
    assert_eq!(body["data"]["bio"], "Hello world");
}

#[actix_web::test]
#[serial]
async fn test_update_user_forbidden() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token_a) =
        common::register_and_login(&app, "a2@example.com", "password123", "A2").await;
    let (user_b, _) =
        common::register_and_login(&app, "b2@example.com", "password123", "B2").await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/users/{}", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .set_json(serde_json::json!({ "name": "Hacked" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 403);
}

#[actix_web::test]
#[serial]
async fn test_update_user_unauthorized() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, _) =
        common::register_and_login(&app, "noauth@example.com", "password123", "NoAuth").await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/users/{}", user_id))
        .set_json(serde_json::json!({ "name": "Nope" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

// ─── GET /users/me/stats ──────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_user_stats_empty() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "stats@example.com", "password123", "StatsUser").await;

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["total_challenges"], 0);
    assert_eq!(body["data"]["completed_today"], 0);
    assert_eq!(body["data"]["current_streak"], 0);
    assert_eq!(body["data"]["best_streak"], 0);
}

#[actix_web::test]
#[serial]
async fn test_get_user_stats_with_answer() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "stats2@example.com", "password123", "Stats2").await;

    // Create category + challenge, then answer
    let cat_id = common::create_test_category(&pool, "StatsCategory").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "Stats challenge").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "My answer").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/users/me/stats")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["total_challenges"], 1);
    assert_eq!(body["data"]["completed_today"], 1);
}

// ─── Follow / Unfollow ─────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_follow_user_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token_a) =
        common::register_and_login(&app, "fa@example.com", "password123", "FA").await;
    let (user_b, _) =
        common::register_and_login(&app, "fb@example.com", "password123", "FB").await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert!(body["data"]["id"].is_string());
}

#[actix_web::test]
#[serial]
async fn test_follow_self_rejected() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (user_id, token) =
        common::register_and_login(&app, "self@example.com", "password123", "SelfFollow").await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
#[serial]
async fn test_follow_duplicate_rejected() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token_a) =
        common::register_and_login(&app, "dup1@example.com", "password123", "Dup1").await;
    let (user_b, _) =
        common::register_and_login(&app, "dup2@example.com", "password123", "Dup2").await;

    // First follow
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Duplicate follow
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 409);
}

#[actix_web::test]
#[serial]
async fn test_unfollow_user_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token_a) =
        common::register_and_login(&app, "uf1@example.com", "password123", "UF1").await;
    let (user_b, _) =
        common::register_and_login(&app, "uf2@example.com", "password123", "UF2").await;

    // Follow first
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Unfollow
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);
}
