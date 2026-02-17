mod common;

use actix_web::{dev::ServiceResponse, test};
use serial_test::serial;
use uuid::Uuid;

// ─── Helper: set up a user + category + challenge ──────────

async fn setup_user_and_challenge(
    pool: &sqlx::PgPool,
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
) -> (Uuid, String, Uuid) {
    let (user_id, token) =
        common::register_and_login(app, "ans@example.com", "password123", "AnsUser").await;
    let cat_id = common::create_test_category(pool, "AnsCat").await;
    let ch_id = common::create_test_challenge(pool, cat_id, "Ans Challenge").await;
    (user_id, token, ch_id)
}

// ─── POST /challenges/{id}/answers ────────────────────────

#[actix_web::test]
#[serial]
async fn test_create_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;

    let (status, body) = common::create_test_answer(&app, ch_id, &token, "Hello world").await;
    assert_eq!(status, 201);
    assert_eq!(body["success"], true);
    assert_eq!(body["data"]["content"], "Hello world");
    assert_eq!(body["data"]["challenge_id"], ch_id.to_string());
}

#[actix_web::test]
#[serial]
async fn test_create_answer_exceeds_char_limit() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, _) = setup_user_and_challenge(&pool, &app).await;

    // Create category with small char limit
    let cat_id = common::create_test_category(&pool, "SmallCat").await;
    // Override char_limit to 10 for this challenge
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO challenges (category_id, title, char_limit, release_date, status)
        VALUES ($1, 'Tiny', 10, CURRENT_DATE, 'active')
        RETURNING id
        "#,
    )
    .bind(cat_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    let tiny_ch = row.0;

    let (status, _) =
        common::create_test_answer(&app, tiny_ch, &token, "This string is way too long for the limit").await;
    assert_eq!(status, 400);
}

#[actix_web::test]
#[serial]
async fn test_create_answer_duplicate() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;

    let (status, _) = common::create_test_answer(&app, ch_id, &token, "First").await;
    assert_eq!(status, 201);

    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Second").await;
    assert_eq!(status, 409);
}

#[actix_web::test]
#[serial]
async fn test_create_answer_challenge_not_found() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token) =
        common::register_and_login(&app, "noch@example.com", "password123", "NoCh").await;

    let fake_ch = Uuid::new_v4();
    let (status, _) = common::create_test_answer(&app, fake_ch, &token, "Test").await;
    assert_eq!(status, 404);
}

#[actix_web::test]
#[serial]
async fn test_create_answer_unauthorized() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let cat_id = common::create_test_category(&pool, "NoAuthCat").await;
    let ch_id = common::create_test_challenge(&pool, cat_id, "NoAuth Ch").await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/challenges/{}/answers", ch_id))
        .set_json(serde_json::json!({ "content": "Test" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
#[serial]
async fn test_create_answer_increments_count() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;

    // Check initial answer_count = 0
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/challenges/{}", ch_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["answer_count"], 0);

    // Create answer
    let (status, _) = common::create_test_answer(&app, ch_id, &token, "Count test").await;
    assert_eq!(status, 201);

    // Check answer_count = 1
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/challenges/{}", ch_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["answer_count"], 1);
}

// ─── GET /answers/{id} ────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_get_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Get me").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["content"], "Get me");
    assert!(body["data"]["user"]["name"].is_string());
    assert!(body["data"]["challenge"]["title"].is_string());
}

#[actix_web::test]
#[serial]
async fn test_get_answer_not_found() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let fake_id = Uuid::new_v4();
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/answers/{}", fake_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

// ─── PUT /answers/{id} ────────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_update_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Original").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({ "content": "Updated" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["content"], "Updated");
}

#[actix_web::test]
#[serial]
async fn test_update_answer_forbidden() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Owner's answer").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    // Another user tries to update
    let (_, other_token) =
        common::register_and_login(&app, "other@example.com", "password123", "Other").await;

    let req = test::TestRequest::put()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .set_json(serde_json::json!({ "content": "Hacked" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 403);
}

// ─── DELETE /answers/{id} ─────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_delete_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Delete me").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);

    // Verify it's gone (soft deleted)
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 404);
}

#[actix_web::test]
#[serial]
async fn test_delete_answer_forbidden() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Owned").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let (_, other_token) =
        common::register_and_login(&app, "del2@example.com", "password123", "Del2").await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 403);
}

// ─── POST /answers/{id}/like ──────────────────────────────

#[actix_web::test]
#[serial]
async fn test_like_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Like me").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    // Another user likes it
    let (_, liker_token) =
        common::register_and_login(&app, "liker@example.com", "password123", "Liker").await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/like", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", liker_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Check the answer now has like_count = 1
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["like_count"], 1);
}

#[actix_web::test]
#[serial]
async fn test_like_answer_duplicate() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "DupLike").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let (_, liker_token) =
        common::register_and_login(&app, "liker2@example.com", "password123", "Liker2").await;

    // First like
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/like", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", liker_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Duplicate like
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/like", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", liker_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 409);
}

// ─── DELETE /answers/{id}/like ────────────────────────────

#[actix_web::test]
#[serial]
async fn test_unlike_answer_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "UnLike").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let (_, liker_token) =
        common::register_and_login(&app, "liker3@example.com", "password123", "Liker3").await;

    // Like first
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/like", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", liker_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    // Unlike
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/answers/{}/like", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", liker_token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);

    // Check like_count back to 0
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/answers/{}", answer_id))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["like_count"], 0);
}

// ─── POST /answers/{id}/comments ──────────────────────────

#[actix_web::test]
#[serial]
async fn test_create_comment_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Comment target").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/comments", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({ "content": "Nice answer!" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["data"]["content"], "Nice answer!");
    assert!(body["data"]["user"]["name"].is_string());
}

// ─── DELETE /comments/{id} ────────────────────────────────

#[actix_web::test]
#[serial]
async fn test_delete_comment_success() {
    let pool = common::setup_test_db().await;
    common::clean_db(&pool).await;
    let app = common::create_test_app(&pool).await;

    let (_, token, ch_id) = setup_user_and_challenge(&pool, &app).await;
    let (_, ans_body) = common::create_test_answer(&app, ch_id, &token, "Comment del").await;
    let answer_id = ans_body["data"]["id"].as_str().unwrap();

    // Create comment
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/answers/{}/comments", answer_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(serde_json::json!({ "content": "To delete" }))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 201);
    let cmt_body: serde_json::Value = test::read_body_json(resp).await;
    let comment_id = cmt_body["data"]["id"].as_str().unwrap();

    // Delete comment
    let req = test::TestRequest::delete()
        .uri(&format!("/api/v1/comments/{}", comment_id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    let resp: ServiceResponse = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204);
}
