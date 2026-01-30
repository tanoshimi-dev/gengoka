use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::config::Config;
use crate::models::{
    Answer, AnswerWithDetails, Challenge, FeedQueryParams, PaginationParams, User, UserSummary,
};
use crate::utils;

pub async fn get_feed(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<FeedQueryParams>,
) -> HttpResponse {
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let filter = query.filter.as_deref().unwrap_or("all");

    let (answers, total) = match filter {
        "following" => {
            let user_id = match current_user_id {
                Some(id) => id,
                None => return utils::unauthorized("User ID required for following feed"),
            };

            let total: (i64,) = sqlx::query_as(
                r#"
                SELECT COUNT(*) FROM answers a
                JOIN follows f ON a.user_id = f.following_id
                WHERE f.follower_id = $1 AND a.status = 'active'
                "#,
            )
            .bind(user_id)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or((0,));

            let answers = sqlx::query_as::<_, Answer>(
                r#"
                SELECT a.* FROM answers a
                JOIN follows f ON a.user_id = f.following_id
                WHERE f.follower_id = $1 AND a.status = 'active'
                ORDER BY a.created_at DESC
                LIMIT $2 OFFSET $3
                "#,
            )
            .bind(user_id)
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await;

            (answers, total.0)
        }
        _ => {
            // Default: all answers
            let total: (i64,) = sqlx::query_as(
                r#"SELECT COUNT(*) FROM answers WHERE status = 'active'"#,
            )
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or((0,));

            let answers = sqlx::query_as::<_, Answer>(
                r#"
                SELECT * FROM answers
                WHERE status = 'active'
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
            )
            .bind(page_size)
            .bind(offset)
            .fetch_all(pool.get_ref())
            .await;

            (answers, total.0)
        }
    };

    let answers = match answers {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to fetch feed: {}", e);
            return utils::internal_error("Failed to fetch feed");
        }
    };

    let mut results: Vec<AnswerWithDetails> = Vec::new();
    for answer in answers {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE id = $1"#,
        )
        .bind(answer.user_id)
        .fetch_one(pool.get_ref())
        .await;

        let challenge = sqlx::query_as::<_, Challenge>(
            r#"SELECT * FROM challenges WHERE id = $1"#,
        )
        .bind(answer.challenge_id)
        .fetch_one(pool.get_ref())
        .await;

        let is_liked = if let Some(uid) = current_user_id {
            sqlx::query_scalar::<_, bool>(
                r#"SELECT EXISTS(SELECT 1 FROM likes WHERE answer_id = $1 AND user_id = $2)"#,
            )
            .bind(answer.id)
            .bind(uid)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(false)
        } else {
            false
        };

        if let (Ok(user), Ok(challenge)) = (user, challenge) {
            results.push(AnswerWithDetails {
                answer,
                user: UserSummary {
                    id: user.id,
                    name: user.name,
                    avatar: user.avatar,
                },
                challenge,
                is_liked,
            });
        }
    }

    utils::paginated(results, page, page_size, total)
}

pub async fn get_trending(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM answers
        WHERE status = 'active' AND created_at > NOW() - INTERVAL '7 days'
        "#,
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // Trending score: likes + comments*2 + views*0.1
    let answers = sqlx::query_as::<_, Answer>(
        r#"
        SELECT * FROM answers
        WHERE status = 'active' AND created_at > NOW() - INTERVAL '7 days'
        ORDER BY (like_count + comment_count * 2 + view_count * 0.1) DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    let answers = match answers {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to fetch trending: {}", e);
            return utils::internal_error("Failed to fetch trending");
        }
    };

    let mut results: Vec<AnswerWithDetails> = Vec::new();
    for answer in answers {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE id = $1"#,
        )
        .bind(answer.user_id)
        .fetch_one(pool.get_ref())
        .await;

        let challenge = sqlx::query_as::<_, Challenge>(
            r#"SELECT * FROM challenges WHERE id = $1"#,
        )
        .bind(answer.challenge_id)
        .fetch_one(pool.get_ref())
        .await;

        let is_liked = if let Some(uid) = current_user_id {
            sqlx::query_scalar::<_, bool>(
                r#"SELECT EXISTS(SELECT 1 FROM likes WHERE answer_id = $1 AND user_id = $2)"#,
            )
            .bind(answer.id)
            .bind(uid)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(false)
        } else {
            false
        };

        if let (Ok(user), Ok(challenge)) = (user, challenge) {
            results.push(AnswerWithDetails {
                answer,
                user: UserSummary {
                    id: user.id,
                    name: user.name,
                    avatar: user.avatar,
                },
                challenge,
                is_liked,
            });
        }
    }

    utils::paginated(results, page, page_size, total.0)
}

pub async fn get_daily_ranking(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    get_ranking_by_period(pool, config, req, query, "1 day").await
}

pub async fn get_weekly_ranking(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    get_ranking_by_period(pool, config, req, query, "7 days").await
}

pub async fn get_alltime_ranking(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE status = 'active'"#,
    )
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let answers = sqlx::query_as::<_, Answer>(
        r#"
        SELECT * FROM answers
        WHERE status = 'active'
        ORDER BY like_count DESC, created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    build_ranking_response(pool, answers, current_user_id, page, page_size, total.0).await
}

async fn get_ranking_by_period(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    query: web::Query<PaginationParams>,
    period: &str,
) -> HttpResponse {
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(&format!(
        r#"SELECT COUNT(*) FROM answers WHERE status = 'active' AND created_at > NOW() - INTERVAL '{}'"#,
        period
    ))
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let answers = sqlx::query_as::<_, Answer>(&format!(
        r#"
        SELECT * FROM answers
        WHERE status = 'active' AND created_at > NOW() - INTERVAL '{}'
        ORDER BY like_count DESC, created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        period
    ))
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    build_ranking_response(pool, answers, current_user_id, page, page_size, total.0).await
}

async fn build_ranking_response(
    pool: web::Data<PgPool>,
    answers: Result<Vec<Answer>, sqlx::Error>,
    current_user_id: Option<uuid::Uuid>,
    page: i64,
    page_size: i64,
    total: i64,
) -> HttpResponse {
    let answers = match answers {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to fetch ranking: {}", e);
            return utils::internal_error("Failed to fetch ranking");
        }
    };

    let mut results: Vec<AnswerWithDetails> = Vec::new();
    for answer in answers {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE id = $1"#,
        )
        .bind(answer.user_id)
        .fetch_one(pool.get_ref())
        .await;

        let challenge = sqlx::query_as::<_, Challenge>(
            r#"SELECT * FROM challenges WHERE id = $1"#,
        )
        .bind(answer.challenge_id)
        .fetch_one(pool.get_ref())
        .await;

        let is_liked = if let Some(uid) = current_user_id {
            sqlx::query_scalar::<_, bool>(
                r#"SELECT EXISTS(SELECT 1 FROM likes WHERE answer_id = $1 AND user_id = $2)"#,
            )
            .bind(answer.id)
            .bind(uid)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(false)
        } else {
            false
        };

        if let (Ok(user), Ok(challenge)) = (user, challenge) {
            results.push(AnswerWithDetails {
                answer,
                user: UserSummary {
                    id: user.id,
                    name: user.name,
                    avatar: user.avatar,
                },
                challenge,
                is_liked,
            });
        }
    }

    utils::paginated(results, page, page_size, total)
}
