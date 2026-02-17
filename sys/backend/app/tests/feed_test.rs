mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;

// ─── GET /feed ────────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_feed_all_returns_answers() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "feed@example.com", "password123", "FeedUser").await;
    let cat_id = common::create_test_category(&pool, "FeedCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "Feed Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Feed answer").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/feed")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 1);
    assert!(body["pagination"]["total"].as_i64().unwrap() >= 1);
}

#[actix_web::test]
#[serial]
async fn test_feed_all_empty() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/feed")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
    assert_eq!(body["pagination"]["total"], 0);
}

#[actix_web::test]
#[serial]
async fn test_feed_following_filter() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    // User A follows User B
    let (_, token_a) =
        common::register_and_login(&app, "ffa@example.com", "password123", "FFA").await;
    let (user_b, token_b) =
        common::register_and_login(&app, "ffb@example.com", "password123", "FFB").await;

    // A follows B
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/users/{}/follow", user_b))
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // B creates an answer
    let cat_id = common::create_test_category(&pool, "FFCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "FF Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token_b, "B's answer").await;
    assert_eq!(status, 201);

    // A's following feed should include B's answer
    let req = test::TestRequest::get()
        .uri("/api/v1/feed?filter=following")
        .insert_header(("Authorization", format!("Bearer {}", token_a)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"].as_array().unwrap().len(), 1);
}

#[actix_web::test]
#[serial]
async fn test_feed_following_requires_auth() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/feed?filter=following")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

// ─── GET /trending ────────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_trending_returns_answers() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "trend@example.com", "password123", "TrendUser").await;
    let cat_id = common::create_test_category(&pool, "TrendCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "Trend Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Trending answer").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/trending")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["data"].as_array().unwrap().len() >= 1);
}

// ─── GET /rankings/* ──────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_daily_ranking() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "rank@example.com", "password123", "RankUser").await;
    let cat_id = common::create_test_category(&pool, "RankCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "Rank Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Rank answer").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/rankings/daily")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["data"].as_array().unwrap().len() >= 1);
}

#[actix_web::test]
#[serial]
async fn test_weekly_ranking() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "wrank@example.com", "password123", "WRank").await;
    let cat_id = common::create_test_category(&pool, "WRankCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "WRank Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Weekly rank").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/rankings/weekly")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["data"].as_array().unwrap().len() >= 1);
}

#[actix_web::test]
#[serial]
async fn test_alltime_ranking() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "arank@example.com", "password123", "ARank").await;
    let cat_id = common::create_test_category(&pool, "ARankCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "ARank Ch").await;
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "All-time rank").await;
    assert_eq!(status, 201);

    let req = test::TestRequest::get()
        .uri("/api/v1/rankings/all-time")
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["data"].as_array().unwrap().len() >= 1);
    assert!(body["pagination"].is_object());
}
