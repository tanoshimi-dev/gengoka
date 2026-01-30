use actix_web::{HttpResponse, http::StatusCode};
use serde::Serialize;

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

// Helper function to get user_id from header
pub fn get_user_id(req: &actix_web::HttpRequest) -> Option<uuid::Uuid> {
    req.headers()
        .get("X-User-ID")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| uuid::Uuid::parse_str(s).ok())
}

// Pagination helpers
pub fn normalize_pagination(page: Option<i64>, page_size: Option<i64>, default_size: i64, max_size: i64) -> (i64, i64, i64) {
    let page = page.unwrap_or(1).max(1);
    let page_size = page_size.unwrap_or(default_size).min(max_size).max(1);
    let offset = (page - 1) * page_size;
    (page, page_size, offset)
}
