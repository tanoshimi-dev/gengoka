use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{
    Answer, Comment, CommentWithUser, CreateCommentRequest, PaginationParams, User, UserSummary,
};
use crate::utils;

pub async fn get_answer_comments(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let answer_id = path.into_inner();
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM comments WHERE answer_id = $1 AND status = 'active'"#,
    )
    .bind(answer_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let comments = sqlx::query_as::<_, Comment>(
        r#"
        SELECT * FROM comments
        WHERE answer_id = $1 AND status = 'active'
        ORDER BY created_at ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(answer_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    let comments = match comments {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to fetch comments: {}", e);
            return utils::internal_error("Failed to fetch comments");
        }
    };

    let mut results: Vec<CommentWithUser> = Vec::new();
    for comment in comments {
        let user = sqlx::query_as::<_, User>(
            r#"SELECT * FROM users WHERE id = $1"#,
        )
        .bind(comment.user_id)
        .fetch_one(pool.get_ref())
        .await;

        if let Ok(user) = user {
            results.push(CommentWithUser {
                comment,
                user: UserSummary {
                    id: user.id,
                    name: user.name,
                    avatar: user.avatar,
                },
            });
        }
    }

    utils::paginated(results, page, page_size, total.0)
}

pub async fn create_comment(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<CreateCommentRequest>,
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

    match answer {
        Ok(None) => return utils::not_found("Answer not found"),
        Err(e) => {
            tracing::error!("Failed to fetch answer: {}", e);
            return utils::internal_error("Failed to create comment");
        }
        _ => {}
    }

    let result = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (answer_id, user_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(answer_id)
    .bind(user_id)
    .bind(&body.content)
    .fetch_one(pool.get_ref())
    .await;

    // Update answer comment count
    let _ = sqlx::query(
        r#"UPDATE answers SET comment_count = comment_count + 1 WHERE id = $1"#,
    )
    .bind(answer_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(comment) => {
            let user = sqlx::query_as::<_, User>(
                r#"SELECT * FROM users WHERE id = $1"#,
            )
            .bind(user_id)
            .fetch_one(pool.get_ref())
            .await;

            match user {
                Ok(user) => utils::created(CommentWithUser {
                    comment,
                    user: UserSummary {
                        id: user.id,
                        name: user.name,
                        avatar: user.avatar,
                    },
                }),
                Err(_) => utils::created(comment),
            }
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {}", e);
            utils::internal_error("Failed to create comment")
        }
    }
}

pub async fn delete_comment(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let comment_id = path.into_inner();
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Check ownership
    let comment = sqlx::query_as::<_, Comment>(
        r#"SELECT * FROM comments WHERE id = $1 AND status = 'active'"#,
    )
    .bind(comment_id)
    .fetch_optional(pool.get_ref())
    .await;

    let comment = match comment {
        Ok(Some(c)) => c,
        Ok(None) => return utils::not_found("Comment not found"),
        Err(e) => {
            tracing::error!("Failed to fetch comment: {}", e);
            return utils::internal_error("Failed to delete comment");
        }
    };

    if comment.user_id != user_id {
        return utils::forbidden("You can only delete your own comments");
    }

    // Soft delete
    let result = sqlx::query(
        r#"UPDATE comments SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(comment_id)
    .execute(pool.get_ref())
    .await;

    // Update answer comment count
    let _ = sqlx::query(
        r#"UPDATE answers SET comment_count = GREATEST(0, comment_count - 1) WHERE id = $1"#,
    )
    .bind(comment.answer_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => utils::no_content(),
        Err(e) => {
            tracing::error!("Failed to delete comment: {}", e);
            utils::internal_error("Failed to delete comment")
        }
    }
}
