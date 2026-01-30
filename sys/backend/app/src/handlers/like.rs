use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Answer, Like};
use crate::utils;

pub async fn like_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check answer exists
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
            return utils::internal_error("Failed to like answer");
        }
    };

    // Create like
    let result = sqlx::query_as::<_, Like>(
        r#"
        INSERT INTO likes (answer_id, user_id)
        VALUES ($1, $2)
        ON CONFLICT (answer_id, user_id) DO NOTHING
        RETURNING *
        "#,
    )
    .bind(answer_id)
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(like)) => {
            // Update answer like count
            let _ = sqlx::query(
                r#"UPDATE answers SET like_count = like_count + 1 WHERE id = $1"#,
            )
            .bind(answer_id)
            .execute(pool.get_ref())
            .await;

            // Update user total likes
            let _ = sqlx::query(
                r#"UPDATE users SET total_likes = total_likes + 1 WHERE id = $1"#,
            )
            .bind(answer.user_id)
            .execute(pool.get_ref())
            .await;

            utils::created(like)
        }
        Ok(None) => utils::conflict("Already liked"),
        Err(e) => {
            tracing::error!("Failed to like answer: {}", e);
            utils::internal_error("Failed to like answer")
        }
    }
}

pub async fn unlike_answer(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check answer exists
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
            return utils::internal_error("Failed to unlike answer");
        }
    };

    // Delete like
    let result = sqlx::query(
        r#"DELETE FROM likes WHERE answer_id = $1 AND user_id = $2"#,
    )
    .bind(answer_id)
    .bind(user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            // Update answer like count
            let _ = sqlx::query(
                r#"UPDATE answers SET like_count = GREATEST(0, like_count - 1) WHERE id = $1"#,
            )
            .bind(answer_id)
            .execute(pool.get_ref())
            .await;

            // Update user total likes
            let _ = sqlx::query(
                r#"UPDATE users SET total_likes = GREATEST(0, total_likes - 1) WHERE id = $1"#,
            )
            .bind(answer.user_id)
            .execute(pool.get_ref())
            .await;

            utils::no_content()
        }
        Ok(_) => utils::not_found("Like not found"),
        Err(e) => {
            tracing::error!("Failed to unlike answer: {}", e);
            utils::internal_error("Failed to unlike answer")
        }
    }
}
