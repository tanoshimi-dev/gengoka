mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;
use uuid::Uuid;

// ─── GET /categories ───────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_list_categories_empty() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/categories")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[actix_web::test]
#[serial]
async fn test_list_categories_with_data() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "TestCat").await;
    // Create a challenge to verify challenge_count
    common::create_test_challenge(&pool, cat_id, "Ch1").await;

    let req = test::TestRequest::get()
        .uri("/api/v1/categories")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let cats = body["data"].as_array().unwrap();
    assert_eq!(cats.len(), 1);
    assert_eq!(cats[0]["name"], "TestCat");
    assert_eq!(cats[0]["challenge_count"], 1);
}

// ─── GET /categories/{id} ─────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_category_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "GetCat").await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/categories/{}", cat_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["name"], "GetCat");
}

#[actix_web::test]
#[serial]
async fn test_get_category_not_found() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let fake_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/categories/{}", fake_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

// ─── POST /challenges ─────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_create_challenge_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "ch@example.com", "password123", "ChUser").await;
    let cat_id = common::create_test_category(&pool, "ChCat").await;

    let req = test::TestRequest::post()
        .uri("/api/v1/challenges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "category_id": cat_id,
            "title": "New Challenge",
            "description": "A test challenge"
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["title"], "New Challenge");
    assert_eq!(body["data"]["category_id"], cat_id.to_string());
}

#[actix_web::test]
#[serial]
async fn test_create_challenge_invalid_category() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "ch2@example.com", "password123", "Ch2").await;
    let fake_cat = Uuid::new_v4();

    let req = test::TestRequest::post()
        .uri("/api/v1/challenges")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({
            "category_id": fake_cat,
            "title": "Bad Challenge"
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 400);
}

#[actix_web::test]
#[serial]
async fn test_create_challenge_unauthorized() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "AuthCat").await;

    let req = test::TestRequest::post()
        .uri("/api/v1/challenges")
        .set_json(serde_json::json!({
            "category_id": cat_id,
            "title": "No Auth"
        }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

// ─── GET /challenges ──────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_list_challenges_paginated() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "PagCat").await;
    for i in 0..3 {
        common::create_test_challenge(&pool, cat_id, &format!("Challenge {}", i)).await;
    }

    let req = test::TestRequest::get()
        .uri("/api/v1/challenges?page=1&page_size=2")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 2);
    assert_eq!(body["pagination"]["total"], 3);
    assert_eq!(body["pagination"]["total_pages"], 2);
    assert_eq!(body["pagination"]["has_more"], true);
}

// ─── GET /challenges/{id} ─────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_challenge_with_category() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "DetailCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "Detail Challenge").await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/challenges/{}", ch_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["title"], "Detail Challenge");
    assert_eq!(body["data"]["category"]["name"], "DetailCat");
}

// ─── GET /challenges/daily ────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_daily_challenges() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "DailyCat").await;
    for i in 0..7 {
        common::create_test_challenge(&pool, cat_id, &format!("Daily {}", i)).await;
    }

    let req = test::TestRequest::get()
        .uri("/api/v1/challenges/daily")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    let daily = body["data"].as_array().unwrap();
    assert!(daily.len() <= 5, "daily challenges should return at most 5");
    assert!(!daily.is_empty());
    // Each entry should have category_name
    assert!(daily[0]["category_name"].is_string());
}
