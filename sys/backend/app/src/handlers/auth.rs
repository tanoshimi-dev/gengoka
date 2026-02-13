use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use validator::Validate;

use crate::config::Config;
use crate::models::{LoginRequest, LogoutRequest, RefreshRequest, SignupRequest, UserSummary};
use crate::utils;

pub async fn register(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<SignupRequest>,
) -> HttpResponse {
    if let Err(e) = body.validate() {
        return utils::bad_request(&format!("Validation error: {}", e));
    }

    let email = body.email.trim().to_lowercase();

    // Check email uniqueness
    let existing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE LOWER(email) = $1",
    )
    .bind(&email)
    .fetch_one(pool.get_ref())
    .await;

    match existing {
        Ok(count) if count > 0 => {
            return utils::conflict("An account with this email already exists");
        }
        Err(e) => {
            tracing::error!("Database error checking email: {}", e);
            return utils::internal_error("An error occurred");
        }
        _ => {}
    }

    // Hash password
    let password_hash = match utils::hash_password(&body.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Password hashing error: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    // Insert user
    let user = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>)>(
        r#"
        INSERT INTO users (email, name, password_hash, status)
        VALUES ($1, $2, $3, 'active')
        RETURNING id, name, avatar
        "#,
    )
    .bind(&email)
    .bind(&body.name)
    .bind(&password_hash)
    .fetch_one(pool.get_ref())
    .await;

    let (user_id, name, avatar) = match user {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Database error creating user: {}", e);
            if e.to_string().contains("duplicate") {
                return utils::conflict("An account with this email already exists");
            }
            return utils::internal_error("An error occurred");
        }
    };

    // Generate tokens
    let access_token = match utils::generate_access_token(&user_id, &config.auth) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Token generation error: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    let refresh_token = utils::generate_refresh_token();
    let refresh_hash = utils::hash_refresh_token(&refresh_token);
    let expires_at = chrono::Utc::now()
        + chrono::Duration::days(config.auth.refresh_token_ttl_days);

    // Store refresh token
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&refresh_hash)
    .bind(&body.device_info)
    .bind(expires_at)
    .execute(pool.get_ref())
    .await
    {
        tracing::error!("Database error storing refresh token: {}", e);
        return utils::internal_error("An error occurred");
    }

    utils::created(crate::models::AuthTokens {
        access_token,
        refresh_token,
        expires_in: config.auth.access_token_ttl_minutes * 60,
        user: UserSummary {
            id: user_id,
            name,
            avatar,
        },
    })
}

pub async fn login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let email = body.email.trim().to_lowercase();

    // Find user by email (must have password_hash set)
    let user = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>, String)>(
        r#"
        SELECT id, name, avatar, password_hash
        FROM users
        WHERE LOWER(email) = $1 AND status = 'active' AND password_hash IS NOT NULL
        "#,
    )
    .bind(&email)
    .fetch_optional(pool.get_ref())
    .await;

    let (user_id, name, avatar, password_hash) = match user {
        Ok(Some(u)) => u,
        Ok(None) => return utils::unauthorized("Invalid email or password"),
        Err(e) => {
            tracing::error!("Database error during login: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    // Verify password
    if !utils::verify_password(&body.password, &password_hash) {
        return utils::unauthorized("Invalid email or password");
    }

    // Generate tokens
    let access_token = match utils::generate_access_token(&user_id, &config.auth) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Token generation error: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    let refresh_token = utils::generate_refresh_token();
    let refresh_hash = utils::hash_refresh_token(&refresh_token);
    let expires_at = chrono::Utc::now()
        + chrono::Duration::days(config.auth.refresh_token_ttl_days);

    // Store refresh token
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&refresh_hash)
    .bind(&body.device_info)
    .bind(expires_at)
    .execute(pool.get_ref())
    .await
    {
        tracing::error!("Database error storing refresh token: {}", e);
        return utils::internal_error("An error occurred");
    }

    utils::success(crate::models::AuthTokens {
        access_token,
        refresh_token,
        expires_in: config.auth.access_token_ttl_minutes * 60,
        user: UserSummary {
            id: user_id,
            name,
            avatar,
        },
    })
}

pub async fn refresh(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<RefreshRequest>,
) -> HttpResponse {
    let token_hash = utils::hash_refresh_token(&body.refresh_token);

    // Look up refresh token
    let token_row = sqlx::query_as::<_, (uuid::Uuid, uuid::Uuid, chrono::DateTime<chrono::Utc>)>(
        r#"
        SELECT id, user_id, expires_at
        FROM refresh_tokens
        WHERE token_hash = $1
        "#,
    )
    .bind(&token_hash)
    .fetch_optional(pool.get_ref())
    .await;

    let (token_id, user_id, expires_at) = match token_row {
        Ok(Some(r)) => r,
        Ok(None) => return utils::unauthorized("Invalid refresh token"),
        Err(e) => {
            tracing::error!("Database error looking up refresh token: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    // Check expiration
    if expires_at < chrono::Utc::now() {
        // Delete expired token
        let _ = sqlx::query("DELETE FROM refresh_tokens WHERE id = $1")
            .bind(token_id)
            .execute(pool.get_ref())
            .await;
        return utils::unauthorized("Refresh token expired");
    }

    // Delete old token (rotation)
    let _ = sqlx::query("DELETE FROM refresh_tokens WHERE id = $1")
        .bind(token_id)
        .execute(pool.get_ref())
        .await;

    // Fetch user info
    let user = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>)>(
        "SELECT id, name, avatar FROM users WHERE id = $1 AND status = 'active'",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let (user_id, name, avatar) = match user {
        Ok(Some(u)) => u,
        Ok(None) => return utils::unauthorized("User not found"),
        Err(e) => {
            tracing::error!("Database error fetching user: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    // Generate new token pair
    let access_token = match utils::generate_access_token(&user_id, &config.auth) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Token generation error: {}", e);
            return utils::internal_error("An error occurred");
        }
    };

    let new_refresh_token = utils::generate_refresh_token();
    let new_refresh_hash = utils::hash_refresh_token(&new_refresh_token);
    let new_expires_at = chrono::Utc::now()
        + chrono::Duration::days(config.auth.refresh_token_ttl_days);

    // Store new refresh token
    if let Err(e) = sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(&new_refresh_hash)
    .bind(new_expires_at)
    .execute(pool.get_ref())
    .await
    {
        tracing::error!("Database error storing new refresh token: {}", e);
        return utils::internal_error("An error occurred");
    }

    utils::success(crate::models::AuthTokens {
        access_token,
        refresh_token: new_refresh_token,
        expires_in: config.auth.access_token_ttl_minutes * 60,
        user: UserSummary {
            id: user_id,
            name,
            avatar,
        },
    })
}

pub async fn logout(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<LogoutRequest>,
) -> HttpResponse {
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("Authentication required"),
    };

    // If a specific refresh token is provided, delete just that one
    if let Some(ref token) = body.refresh_token {
        let token_hash = utils::hash_refresh_token(token);
        let _ = sqlx::query(
            "DELETE FROM refresh_tokens WHERE user_id = $1 AND token_hash = $2",
        )
        .bind(user_id)
        .bind(&token_hash)
        .execute(pool.get_ref())
        .await;
    } else {
        // Otherwise delete all refresh tokens for this user (logout everywhere)
        let _ = sqlx::query("DELETE FROM refresh_tokens WHERE user_id = $1")
            .bind(user_id)
            .execute(pool.get_ref())
            .await;
    }

    utils::no_content()
}
