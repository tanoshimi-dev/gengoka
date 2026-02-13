use actix_web::{HttpResponse, http::StatusCode};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::Serialize;
use sha2::{Sha256, Digest};

use crate::config::AuthConfig;
use crate::models::Claims;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<PaginationInfo>,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub page_size: i64,
    pub total: i64,
    pub total_pages: i64,
    pub has_more: bool,
}

impl PaginationInfo {
    pub fn new(page: i64, page_size: i64, total: i64) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Self {
            page,
            page_size,
            total,
            total_pages,
            has_more: page < total_pages,
        }
    }
}

// Success responses
pub fn success<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        pagination: None,
    })
}

pub fn created<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        pagination: None,
    })
}

pub fn paginated<T: Serialize>(data: T, page: i64, page_size: i64, total: i64) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        pagination: Some(PaginationInfo::new(page, page_size, total)),
    })
}

pub fn no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

// Error responses
pub fn error(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(ApiResponse::<()> {
        success: false,
        data: None,
        error: Some(message.to_string()),
        pagination: None,
    })
}

pub fn bad_request(message: &str) -> HttpResponse {
    error(StatusCode::BAD_REQUEST, message)
}

pub fn unauthorized(message: &str) -> HttpResponse {
    error(StatusCode::UNAUTHORIZED, message)
}

pub fn forbidden(message: &str) -> HttpResponse {
    error(StatusCode::FORBIDDEN, message)
}

pub fn not_found(message: &str) -> HttpResponse {
    error(StatusCode::NOT_FOUND, message)
}

pub fn conflict(message: &str) -> HttpResponse {
    error(StatusCode::CONFLICT, message)
}

pub fn internal_error(message: &str) -> HttpResponse {
    error(StatusCode::INTERNAL_SERVER_ERROR, message)
}

// Helper function to get user_id from JWT bearer token or X-User-ID header
pub fn get_user_id(req: &actix_web::HttpRequest) -> Option<uuid::Uuid> {
    // First try Authorization: Bearer <jwt>
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Some(config) = req.app_data::<actix_web::web::Data<crate::config::Config>>() {
                    if let Ok(claims) = decode_access_token(token, &config.auth) {
                        if let Ok(uid) = uuid::Uuid::parse_str(&claims.sub) {
                            return Some(uid);
                        }
                    }
                }
            }
        }
    }

    // Fall back to X-User-ID header (backward compat)
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| uuid::Uuid::parse_str(s).ok())
}

// Password helpers (argon2, same pattern as admin auth)
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

// JWT helpers
pub fn generate_access_token(user_id: &uuid::Uuid, config: &AuthConfig) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::minutes(config.access_token_ttl_minutes);
    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
}

pub fn decode_access_token(token: &str, config: &AuthConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn generate_refresh_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn hash_refresh_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Pagination helpers
pub fn normalize_pagination(page: Option<i64>, page_size: Option<i64>, default_size: i64, max_size: i64) -> (i64, i64, i64) {
    let page = page.unwrap_or(1).max(1);
    let page_size = page_size.unwrap_or(default_size).min(max_size).max(1);
    let offset = (page - 1) * page_size;
    (page, page_size, offset)
}
