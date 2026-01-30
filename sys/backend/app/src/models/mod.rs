use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// ============ User ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub total_likes: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub total_likes: i32,
    pub answer_count: i64,
    pub follower_count: i64,
    pub following_count: i64,
    pub is_following: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    pub email: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

// ============ Category ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub char_limit: i32,
    pub sort_order: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============ Challenge ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Challenge {
    pub id: Uuid,
    pub category_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub char_limit: i32,
    pub release_date: Option<NaiveDate>,
    pub answer_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ChallengeWithCategory {
    #[serde(flatten)]
    pub challenge: Challenge,
    pub category: Category,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChallengeRequest {
    pub category_id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    pub char_limit: Option<i32>,
    pub release_date: Option<NaiveDate>,
}

// ============ Answer ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Answer {
    pub id: Uuid,
    pub challenge_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub score: Option<i32>,
    pub ai_feedback: Option<serde_json::Value>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AnswerWithUser {
    #[serde(flatten)]
    pub answer: Answer,
    pub user: UserSummary,
    pub is_liked: bool,
}

#[derive(Debug, Serialize)]
pub struct AnswerWithDetails {
    #[serde(flatten)]
    pub answer: Answer,
    pub user: UserSummary,
    pub challenge: Challenge,
    pub is_liked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAnswerRequest {
    #[validate(length(min = 1, max = 200))]
    pub content: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAnswerRequest {
    #[validate(length(min = 1, max = 200))]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiFeedback {
    pub score: i32,
    pub good_points: String,
    pub improvement: String,
    pub example_answer: String,
}

// ============ Comment ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub answer_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CommentWithUser {
    #[serde(flatten)]
    pub comment: Comment,
    pub user: UserSummary,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1, max = 500))]
    pub content: String,
}

// ============ Like ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Like {
    pub id: Uuid,
    pub answer_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ============ Follow ============

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub following_id: Uuid,
    pub created_at: DateTime<Utc>,
}

// ============ Query Parameters ============

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AnswerQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort: Option<String>, // latest, popular, trending
}

#[derive(Debug, Deserialize)]
pub struct FeedQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub filter: Option<String>, // all, following, category_id
    pub category_id: Option<Uuid>,
}
