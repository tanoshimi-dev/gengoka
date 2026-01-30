use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{Category, Challenge, ChallengeWithCategory, CreateChallengeRequest, PaginationParams};
use crate::utils;

pub async fn get_daily_challenges(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as::<_, Challenge>(
        r#"
        SELECT * FROM challenges
        WHERE status = 'active'
          AND (release_date IS NULL OR release_date <= CURRENT_DATE)
        ORDER BY release_date DESC NULLS LAST, created_at DESC
        LIMIT 5
        "#,
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(challenges) => utils::success(challenges),
        Err(e) => {
            tracing::error!("Failed to fetch daily challenges: {}", e);
            utils::internal_error("Failed to fetch daily challenges")
        }
    }
}

pub async fn list_challenges(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    // Get total count
    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM challenges WHERE status = 'active'"#,
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let result = sqlx::query_as::<_, Challenge>(
        r#"
        SELECT * FROM challenges
        WHERE status = 'active'
        ORDER BY release_date DESC NULLS LAST, created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(challenges) => utils::paginated(challenges, page, page_size, total.0),
        Err(e) => {
            tracing::error!("Failed to fetch challenges: {}", e);
            utils::internal_error("Failed to fetch challenges")
        }
    }
}

pub async fn get_challenge(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> HttpResponse {
    let challenge_id = path.into_inner();

    // Get challenge with category
    let challenge = sqlx::query_as::<_, Challenge>(
        r#"SELECT * FROM challenges WHERE id = $1 AND status = 'active'"#,
    )
    .bind(challenge_id)
    .fetch_optional(pool.get_ref())
    .await;

    let challenge = match challenge {
        Ok(Some(c)) => c,
        Ok(None) => return utils::not_found("Challenge not found"),
        Err(e) => {
            tracing::error!("Failed to fetch challenge: {}", e);
            return utils::internal_error("Failed to fetch challenge");
        }
    };

    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1"#,
    )
    .bind(challenge.category_id)
    .fetch_one(pool.get_ref())
    .await;

    match category {
        Ok(category) => utils::success(ChallengeWithCategory { challenge, category }),
        Err(e) => {
            tracing::error!("Failed to fetch category: {}", e);
            utils::internal_error("Failed to fetch challenge details")
        }
    }
}

pub async fn get_challenges_by_category(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let category_id = path.into_inner();
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM challenges WHERE category_id = $1 AND status = 'active'"#,
    )
    .bind(category_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let result = sqlx::query_as::<_, Challenge>(
        r#"
        SELECT * FROM challenges
        WHERE category_id = $1 AND status = 'active'
        ORDER BY release_date DESC NULLS LAST, created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(category_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(challenges) => utils::paginated(challenges, page, page_size, total.0),
        Err(e) => {
            tracing::error!("Failed to fetch challenges: {}", e);
            utils::internal_error("Failed to fetch challenges")
        }
    }
}

pub async fn create_challenge(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<CreateChallengeRequest>,
) -> HttpResponse {
    // Check user authentication
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Validate category exists
    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1 AND status = 'active'"#,
    )
    .bind(body.category_id)
    .fetch_optional(pool.get_ref())
    .await;

    let category = match category {
        Ok(Some(c)) => c,
        Ok(None) => return utils::bad_request("Category not found"),
        Err(e) => {
            tracing::error!("Failed to validate category: {}", e);
            return utils::internal_error("Failed to create challenge");
        }
    };

    let char_limit = body.char_limit.unwrap_or(category.char_limit);

    let result = sqlx::query_as::<_, Challenge>(
        r#"
        INSERT INTO challenges (category_id, title, description, char_limit, release_date)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(body.category_id)
    .bind(&body.title)
    .bind(&body.description)
    .bind(char_limit)
    .bind(body.release_date)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(challenge) => utils::created(challenge),
        Err(e) => {
            tracing::error!("Failed to create challenge: {}", e);
            utils::internal_error("Failed to create challenge")
        }
    }
}
