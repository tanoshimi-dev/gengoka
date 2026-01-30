use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{
    Answer, AnswerQueryParams, AnswerWithDetails, AnswerWithUser, Challenge,
    CreateAnswerRequest, UpdateAnswerRequest, User, UserSummary,
};
use crate::utils;

pub async fn get_challenge_answers(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    query: web::Query<AnswerQueryParams>,
) -> HttpResponse {
    let challenge_id = path.into_inner();
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let sort = query.sort.as_deref().unwrap_or("latest");
    let order_by = match sort {
        "popular" => "a.like_count DESC, a.created_at DESC",
        "trending" => "(a.like_count + a.comment_count * 2 + a.view_count * 0.1) DESC",
        _ => "a.created_at DESC",
    };

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE challenge_id = $1 AND status = 'active'"#,
    )
    .bind(challenge_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let answers = sqlx::query_as::<_, Answer>(&format!(
        r#"
        SELECT a.* FROM answers a
        WHERE a.challenge_id = $1 AND a.status = 'active'
        ORDER BY {}
        LIMIT $2 OFFSET $3
        "#,
        order_by
    ))
    .bind(challenge_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    let answers = match answers {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to fetch answers: {}", e);
            return utils::internal_error("Failed to fetch answers");
        }
    };

    // Fetch users and likes for each answer
    let mut results: Vec<AnswerWithUser> = Vec::new();
    for answer in answers {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE id = $1"#,
        )
        .bind(answer.user_id)
        .fetch_one(pool.get_ref())
        .await
        .ok();

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

        if let Some(user) = user {
            results.push(AnswerWithUser {
                answer,
                user: UserSummary {
                    id: user.id,
                    name: user.name,
                    avatar: user.avatar,
                },
                is_liked,
            });
        }
    }

    utils::paginated(results, page, page_size, total.0)
}

pub async fn get_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let current_user_id = utils::get_user_id(&req);

    // Increment view count
    let _ = sqlx::query(
        r#"UPDATE answers SET view_count = view_count + 1 WHERE id = $1"#,
    )
    .bind(answer_id)
    .execute(pool.get_ref())
    .await;

    let answer = sqlx::query_as::<_, Answer>(
        r#"SELECT * FROM answers WHERE id = $1 AND status = 'active'"#,
    )
    .bind(answer_id)
    .fetch_optional(pool.get_ref())
    .await;

    let answer = match answer {
        Ok(Some(a)) => a,
        Ok(None) => return utils::not_found("Answer not found"),
        Err(e) => {
            tracing::error!("Failed to fetch answer: {}", e);
            return utils::internal_error("Failed to fetch answer");
        }
    };

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
        .bind(answer_id)
        .bind(uid)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(false)
    } else {
        false
    };

    match (user, challenge) {
        (Ok(user), Ok(challenge)) => utils::success(AnswerWithDetails {
            answer,
            user: UserSummary {
                id: user.id,
                name: user.name,
                avatar: user.avatar,
            },
            challenge,
            is_liked,
        }),
        _ => utils::internal_error("Failed to fetch answer details"),
    }
}

pub async fn create_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<CreateAnswerRequest>,
) -> HttpResponse {
    let challenge_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check challenge exists and get char limit
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
            return utils::internal_error("Failed to create answer");
        }
    };

    // Validate content length
    if body.content.chars().count() > challenge.char_limit as usize {
        return utils::bad_request(&format!(
            "Content exceeds {} character limit",
            challenge.char_limit
        ));
    }

    // Check if user already answered this challenge
    let existing = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM answers WHERE challenge_id = $1 AND user_id = $2 AND status = 'active')"#,
    )
    .bind(challenge_id)
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(false);

    if existing {
        return utils::conflict("You have already answered this challenge");
    }

    // Create answer
    let result = sqlx::query_as::<_, Answer>(
        r#"
        INSERT INTO answers (challenge_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(challenge_id)
    .bind(user_id)
    .bind(&body.content)
    .fetch_one(pool.get_ref())
    .await;

    // Update challenge answer count
    let _ = sqlx::query(
        r#"UPDATE challenges SET answer_count = answer_count + 1 WHERE id = $1"#,
    )
    .bind(challenge_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(answer) => utils::created(answer),
        Err(e) => {
            tracing::error!("Failed to create answer: {}", e);
            utils::internal_error("Failed to create answer")
        }
    }
}

pub async fn update_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateAnswerRequest>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check ownership
    let answer = sqlx::query_as::<_, Answer>(
        r#"SELECT * FROM answers WHERE id = $1 AND status = 'active'"#,
    )
    .bind(answer_id)
    .fetch_optional(pool.get_ref())
    .await;

    let answer = match answer {
        Ok(Some(a)) => a,
        Ok(None) => return utils::not_found("Answer not found"),
        Err(e) => {
            tracing::error!("Failed to fetch answer: {}", e);
            return utils::internal_error("Failed to update answer");
        }
    };

    if answer.user_id != user_id {
        return utils::forbidden("You can only update your own answers");
    }

    let content = body.content.as_ref().unwrap_or(&answer.content);

    let result = sqlx::query_as::<_, Answer>(
        r#"
        UPDATE answers
        SET content = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING *
        "#,
    )
    .bind(content)
    .bind(answer_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(answer) => utils::success(answer),
        Err(e) => {
            tracing::error!("Failed to update answer: {}", e);
            utils::internal_error("Failed to update answer")
        }
    }
}

pub async fn delete_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check ownership
    let answer = sqlx::query_as::<_, Answer>(
        r#"SELECT * FROM answers WHERE id = $1 AND status = 'active'"#,
    )
    .bind(answer_id)
    .fetch_optional(pool.get_ref())
    .await;

    let answer = match answer {
        Ok(Some(a)) => a,
        Ok(None) => return utils::not_found("Answer not found"),
        Err(e) => {
            tracing::error!("Failed to fetch answer: {}", e);
            return utils::internal_error("Failed to delete answer");
        }
    };

    if answer.user_id != user_id {
        return utils::forbidden("You can only delete your own answers");
    }

    // Soft delete
    let result = sqlx::query(
        r#"UPDATE answers SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(answer_id)
    .execute(pool.get_ref())
    .await;

    // Update challenge answer count
    let _ = sqlx::query(
        r#"UPDATE challenges SET answer_count = answer_count - 1 WHERE id = $1"#,
    )
    .bind(answer.challenge_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => utils::no_content(),
        Err(e) => {
            tracing::error!("Failed to delete answer: {}", e);
            utils::internal_error("Failed to delete answer")
        }
    }
}
