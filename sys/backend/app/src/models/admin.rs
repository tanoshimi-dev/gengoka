use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ============ Admin User ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminUser {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminUserSummary {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    pub status: String,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<AdminUser> for AdminUserSummary {
    fn from(user: AdminUser) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            status: user.status,
            last_login_at: user.last_login_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAdminUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAdminUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

// ============ Admin Audit Log ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AdminAuditLog {
    pub id: Uuid,
    pub admin_user_id: Uuid,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogWithAdmin {
    #[serde(flatten)]
    pub log: AdminAuditLog,
    pub admin_name: String,
}

// ============ System Config ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SystemConfig {
    pub key: String,
    pub value: serde_json::Value,
    pub description: Option<String>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSystemConfigRequest {
    pub value: serde_json::Value,
    pub description: Option<String>,
}

// ============ Dashboard Stats ============

#[derive(Debug, Serialize, Default)]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_categories: i64,
    pub total_challenges: i64,
    pub total_answers: i64,
    pub total_comments: i64,
    pub users_today: i64,
    pub answers_today: i64,
    pub recent_users: Vec<UserStat>,
    pub recent_answers: Vec<AnswerStat>,
}

#[derive(Debug, Serialize)]
pub struct UserStat {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AnswerStat {
    pub id: Uuid,
    pub content: String,
    pub user_name: String,
    pub challenge_title: String,
    pub created_at: DateTime<Utc>,
}

// ============ Category Admin ============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub char_limit: Option<i32>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub char_limit: Option<i32>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

// ============ Challenge Admin ============

#[derive(Debug, Deserialize, Validate)]
pub struct AdminCreateChallengeRequest {
    pub category_id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    pub char_limit: Option<i32>,
    pub release_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AdminUpdateChallengeRequest {
    pub category_id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub char_limit: Option<i32>,
    pub release_date: Option<chrono::NaiveDate>,
    pub status: Option<String>,
}

// ============ User Moderation ============

#[derive(Debug, Deserialize, Validate)]
pub struct SuspendUserRequest {
    #[validate(length(min = 1))]
    pub reason: String,
}

// ============ Content Moderation ============

#[derive(Debug, Deserialize, Validate)]
pub struct ModerateContentRequest {
    #[validate(length(min = 1))]
    pub reason: String,
}

// ============ Gemini Settings ============

#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiSettings {
    pub model: String,
    pub max_tokens: i32,
    pub temperature: f32,
}

// ============ Scheduler Settings ============

#[derive(Debug, Serialize, Deserialize)]
pub struct SchedulerSettings {
    pub enabled: bool,
    pub daily_count: u32,
    pub cron_schedule: String,
}

// ============ Bulk Challenge Generation ============

#[derive(Debug, Deserialize)]
pub struct BulkGenerateCategoryItem {
    pub category_id: Uuid,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct BulkGenerateRequest {
    pub items: Vec<BulkGenerateCategoryItem>,
}

#[derive(Debug, Serialize)]
pub struct BulkGeneratedChallengeItem {
    pub category_id: Uuid,
    pub category_name: String,
    pub title: String,
    pub description: String,
    pub char_limit: i32,
}

#[derive(Debug, Serialize)]
pub struct BulkGenerateResponse {
    pub challenges: Vec<BulkGeneratedChallengeItem>,
    pub total: usize,
}

#[derive(Debug, Deserialize)]
pub struct BulkSaveChallengeItem {
    pub category_id: Uuid,
    pub title: String,
    pub description: String,
    pub char_limit: i32,
}

#[derive(Debug, Deserialize)]
pub struct BulkSaveRequest {
    pub challenges: Vec<BulkSaveChallengeItem>,
}

#[derive(Debug, Serialize)]
pub struct BulkSaveResponse {
    pub saved_count: usize,
}

// ============ Pagination ============

#[derive(Debug, Deserialize)]
pub struct AdminPaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
}
