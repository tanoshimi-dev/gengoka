use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::config::Config;
use crate::models::{AuthTokens, SocialLoginRequest, SocialUserInfo, UserSummary};
use crate::services::social_auth;
use crate::utils;

pub async fn social_login(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    body: web::Json<SocialLoginRequest>,
) -> HttpResponse {
    // Validate provider
    let provider = body.provider.to_lowercase();
    if !["google", "apple", "line"].contains(&provider.as_str()) {
        return utils::bad_request("Unsupported provider. Use: google, apple, line");
    }

    // Verify token with the appropriate provider and extract user info
    let social_user = match provider.as_str() {
        "google" => {
            let id_token = match &body.id_token {
                Some(t) => t,
                None => return utils::bad_request("id_token is required for Google sign-in"),
            };
            match social_auth::verify_google_token(id_token, &config.social_auth).await {
                Ok(user) => user,
                Err(e) => {
                    tracing::error!("Google token verification failed: {}", e);
                    return utils::unauthorized("Google authentication failed");
                }
            }
        }
        "apple" => {
            let id_token = match &body.id_token {
                Some(t) => t,
                None => return utils::bad_request("id_token is required for Apple sign-in"),
            };
            match social_auth::verify_apple_token(id_token, &config.social_auth).await {
                Ok(user) => user,
                Err(e) => {
                    tracing::error!("Apple token verification failed: {}", e);
                    return utils::unauthorized("Apple authentication failed");
                }
            }
        }
        "line" => {
            let access_token = match &body.access_token {
                Some(t) => t,
                None => {
                    return utils::bad_request("access_token is required for LINE sign-in")
                }
            };
            match social_auth::verify_line_token(access_token, &config.social_auth).await {
                Ok(user) => user,
                Err(e) => {
                    tracing::error!("LINE token verification failed: {}", e);
                    return utils::unauthorized("LINE authentication failed");
                }
            }
        }
        _ => unreachable!(),
    };

    // Find or create user
    match find_or_create_user(&pool, &config, &social_user, &body).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Social login user creation failed: {}", e);
            utils::internal_error("An error occurred during social login")
        }
    }
}

async fn find_or_create_user(
    pool: &PgPool,
    config: &Config,
    social_user: &SocialUserInfo,
    request: &SocialLoginRequest,
) -> Result<HttpResponse, String> {
    // Step 1: Check if social account already linked
    let existing_link = sqlx::query_as::<_, (uuid::Uuid,)>(
        "SELECT user_id FROM user_social_accounts WHERE provider = $1 AND provider_user_id = $2",
    )
    .bind(&social_user.provider)
    .bind(&social_user.provider_user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error checking social account: {}", e))?;

    let user_id = if let Some((uid,)) = existing_link {
        // Existing social account — update provider info
        sqlx::query(
            r#"
            UPDATE user_social_accounts
            SET provider_email = COALESCE($1, provider_email),
                provider_name = COALESCE($2, provider_name),
                provider_avatar = COALESCE($3, provider_avatar),
                updated_at = NOW()
            WHERE provider = $4 AND provider_user_id = $5
            "#,
        )
        .bind(&social_user.email)
        .bind(&social_user.name)
        .bind(&social_user.avatar)
        .bind(&social_user.provider)
        .bind(&social_user.provider_user_id)
        .execute(pool)
        .await
        .ok();

        uid
    } else {
        // Step 2: Check if a user with the same email exists (account linking)
        let email_user = if let Some(ref email) = social_user.email {
            sqlx::query_as::<_, (uuid::Uuid,)>(
                "SELECT id FROM users WHERE LOWER(email) = LOWER($1) AND status = 'active'",
            )
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("DB error checking email: {}", e))?
        } else {
            None
        };

        let uid = if let Some((existing_uid,)) = email_user {
            // Link social account to existing user
            existing_uid
        } else {
            // Step 3: Create new user
            let display_name = social_user
                .name
                .clone()
                .or_else(|| {
                    social_user
                        .email
                        .as_ref()
                        .and_then(|e| e.split('@').next())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| format!("user_{}", &social_user.provider_user_id[..8.min(social_user.provider_user_id.len())]));

            let new_user = sqlx::query_as::<_, (uuid::Uuid,)>(
                r#"
                INSERT INTO users (email, name, avatar, status)
                VALUES ($1, $2, $3, 'active')
                RETURNING id
                "#,
            )
            .bind(&social_user.email)
            .bind(&display_name)
            .bind(&social_user.avatar)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("DB error creating user: {}", e))?;

            new_user.0
        };

        // Insert social account link
        sqlx::query(
            r#"
            INSERT INTO user_social_accounts
                (user_id, provider, provider_user_id, provider_email, provider_name, provider_avatar)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (provider, provider_user_id) DO NOTHING
            "#,
        )
        .bind(uid)
        .bind(&social_user.provider)
        .bind(&social_user.provider_user_id)
        .bind(&social_user.email)
        .bind(&social_user.name)
        .bind(&social_user.avatar)
        .execute(pool)
        .await
        .map_err(|e| format!("DB error linking social account: {}", e))?;

        uid
    };

    // Fetch user info for response
    let user = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>)>(
        "SELECT id, name, avatar FROM users WHERE id = $1 AND status = 'active'",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error fetching user: {}", e))?
    .ok_or("User not found after creation")?;

    // Generate JWT tokens
    let access_token = utils::generate_access_token(&user.0, &config.auth)
        .map_err(|e| format!("Token generation error: {}", e))?;

    let refresh_token = utils::generate_refresh_token();
    let refresh_hash = utils::hash_refresh_token(&refresh_token);
    let expires_at =
        chrono::Utc::now() + chrono::Duration::days(config.auth.refresh_token_ttl_days);

    // Store refresh token
    sqlx::query(
        r#"
        INSERT INTO refresh_tokens (user_id, token_hash, device_info, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(user_id)
    .bind(&refresh_hash)
    .bind(&request.device_info)
    .bind(expires_at)
    .execute(pool)
    .await
    .map_err(|e| format!("DB error storing refresh token: {}", e))?;

    Ok(utils::success(AuthTokens {
        access_token,
        refresh_token,
        expires_in: config.auth.access_token_ttl_minutes * 60,
        user: UserSummary {
            id: user.0,
            name: user.1,
            avatar: user.2,
        },
    }))
}
