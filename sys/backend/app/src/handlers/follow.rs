use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{Follow, PaginationParams, User, UserSummary};
use crate::utils;

pub async fn follow_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let following_id = path.into_inner();
    let follower_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Cannot follow yourself
    if follower_id == following_id {
        return utils::bad_request("Cannot follow yourself");
    }

    // Check target user exists
    let user_exists = sqlx::query_scalar::<_, bool>(
        r#"SELECT EXISTS(SELECT 1 FROM users WHERE id = $1 AND status = 'active')"#,
    )
    .bind(following_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or(false);

    if !user_exists {
        return utils::not_found("User not found");
    }

    // Create follow
    let result = sqlx::query_as::<_, Follow>(
        r#"
        INSERT INTO follows (follower_id, following_id)
        VALUES ($1, $2)
        ON CONFLICT (follower_id, following_id) DO NOTHING
        RETURNING *
        "#,
    )
    .bind(follower_id)
    .bind(following_id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(follow)) => utils::created(follow),
        Ok(None) => utils::conflict("Already following"),
        Err(e) => {
            tracing::error!("Failed to follow user: {}", e);
            utils::internal_error("Failed to follow user")
        }
    }
}

pub async fn unfollow_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let following_id = path.into_inner();
    let follower_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    let result = sqlx::query(
        r#"DELETE FROM follows WHERE follower_id = $1 AND following_id = $2"#,
    )
    .bind(follower_id)
    .bind(following_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => utils::no_content(),
        Ok(_) => utils::not_found("Follow relationship not found"),
        Err(e) => {
            tracing::error!("Failed to unfollow user: {}", e);
            utils::internal_error("Failed to unfollow user")
        }
    }
}

pub async fn get_followers(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE following_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let followers = sqlx::query_as::<_, User>(
        r#"
        SELECT u.* FROM users u
        JOIN follows f ON u.id = f.follower_id
        WHERE f.following_id = $1 AND u.status = 'active'
        ORDER BY f.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match followers {
        Ok(users) => {
            let summaries: Vec<UserSummary> = users
                .into_iter()
                .map(|u| UserSummary {
                    id: u.id,
                    name: u.name,
                    avatar: u.avatar,
                })
                .collect();
            utils::paginated(summaries, page, page_size, total.0)
        }
        Err(e) => {
            tracing::error!("Failed to fetch followers: {}", e);
            utils::internal_error("Failed to fetch followers")
        }
    }
}

pub async fn get_following(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE follower_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let following = sqlx::query_as::<_, User>(
        r#"
        SELECT u.* FROM users u
        JOIN follows f ON u.id = f.following_id
        WHERE f.follower_id = $1 AND u.status = 'active'
        ORDER BY f.created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    match following {
        Ok(users) => {
            let summaries: Vec<UserSummary> = users
                .into_iter()
                .map(|u| UserSummary {
                    id: u.id,
                    name: u.name,
                    avatar: u.avatar,
                })
                .collect();
            utils::paginated(summaries, page, page_size, total.0)
        }
        Err(e) => {
            tracing::error!("Failed to fetch following: {}", e);
            utils::internal_error("Failed to fetch following")
        }
    }
}
