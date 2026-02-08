use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminPaginationParams, AdminUser, ModerateContentRequest, PageLink};
use crate::models::Answer;

#[derive(FromRow)]
struct AnswerRow {
    id: Uuid,
    challenge_id: Uuid,
    user_id: Uuid,
    content: String,
    score: Option<i32>,
    ai_feedback: Option<serde_json::Value>,
    like_count: i32,
    comment_count: i32,
    view_count: i32,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
    challenge_title: String,
    moderated_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
struct AnswerDetailRow {
    id: Uuid,
    challenge_id: Uuid,
    user_id: Uuid,
    content: String,
    score: Option<i32>,
    ai_feedback: Option<serde_json::Value>,
    like_count: i32,
    comment_count: i32,
    view_count: i32,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
    challenge_title: String,
    moderated_at: Option<DateTime<Utc>>,
    moderation_reason: Option<String>,
}

#[derive(FromRow)]
struct AnswerModerateRow {
    id: Uuid,
    challenge_id: Uuid,
    user_id: Uuid,
    content: String,
    score: Option<i32>,
    ai_feedback: Option<serde_json::Value>,
    like_count: i32,
    comment_count: i32,
    view_count: i32,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
}

#[derive(Template)]
#[template(path = "admin/answers/list.html")]
pub struct AnswersListTemplate {
    pub admin: AdminUser,
    pub answers: Vec<AnswerWithDetails>,
    pub search: String,
    pub status_filter: String,
    pub page: i64,
    pub page_size: i64,
    pub total_count: i64,
    pub total_pages: i64,
    pub showing_from: i64,
    pub showing_to: i64,
    pub has_prev: bool,
    pub has_next: bool,
    pub page_links: Vec<PageLink>,
}

pub struct AnswerWithDetails {
    pub answer: Answer,
    pub user_name: String,
    pub challenge_title: String,
    pub is_moderated: bool,
}

impl From<AnswerRow> for AnswerWithDetails {
    fn from(row: AnswerRow) -> Self {
        Self {
            answer: Answer {
                id: row.id,
                challenge_id: row.challenge_id,
                user_id: row.user_id,
                content: row.content,
                score: row.score,
                ai_feedback: row.ai_feedback,
                like_count: row.like_count,
                comment_count: row.comment_count,
                view_count: row.view_count,
                status: row.status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            user_name: row.user_name,
            challenge_title: row.challenge_title,
            is_moderated: row.moderated_at.is_some(),
        }
    }
}

#[derive(Template)]
#[template(path = "admin/answers/detail.html")]
pub struct AnswerDetailTemplate {
    pub admin: AdminUser,
    pub answer: Answer,
    pub user_name: String,
    pub challenge_title: String,
    pub is_moderated: bool,
    pub moderation_reason: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/answers/moderate.html")]
pub struct ModerateAnswerTemplate {
    pub admin: AdminUser,
    pub answer: Answer,
    pub user_name: String,
    pub error: Option<String>,
}

pub async fn list_answers(
    pool: web::Data<PgPool>,
    session: Session,
    query: web::Query<AdminPaginationParams>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(20).min(100);
    let offset = (page - 1) * page_size;
    let search = query.search.clone().unwrap_or_default();
    let status_filter = query.status.clone().unwrap_or_default();

    let search_pattern = format!("%{}%", search);

    let (total, answers_data): ((i64,), Vec<AnswerRow>) = if status_filter == "moderated" {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM answers WHERE moderated_at IS NOT NULL AND content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, AnswerRow>(
            r#"
            SELECT a.id, a.challenge_id, a.user_id, a.content, a.score, a.ai_feedback,
                   a.like_count, a.comment_count, a.view_count, a.status, a.created_at, a.updated_at,
                   u.name as user_name, c.title as challenge_title, a.moderated_at
            FROM answers a
            JOIN users u ON a.user_id = u.id
            JOIN challenges c ON a.challenge_id = c.id
            WHERE a.moderated_at IS NOT NULL AND a.content ILIKE $1
            ORDER BY a.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, data)
    } else if status_filter == "active" {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM answers WHERE status = 'active' AND moderated_at IS NULL AND content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, AnswerRow>(
            r#"
            SELECT a.id, a.challenge_id, a.user_id, a.content, a.score, a.ai_feedback,
                   a.like_count, a.comment_count, a.view_count, a.status, a.created_at, a.updated_at,
                   u.name as user_name, c.title as challenge_title, a.moderated_at
            FROM answers a
            JOIN users u ON a.user_id = u.id
            JOIN challenges c ON a.challenge_id = c.id
            WHERE a.status = 'active' AND a.moderated_at IS NULL AND a.content ILIKE $1
            ORDER BY a.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, data)
    } else {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM answers WHERE content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, AnswerRow>(
            r#"
            SELECT a.id, a.challenge_id, a.user_id, a.content, a.score, a.ai_feedback,
                   a.like_count, a.comment_count, a.view_count, a.status, a.created_at, a.updated_at,
                   u.name as user_name, c.title as challenge_title, a.moderated_at
            FROM answers a
            JOIN users u ON a.user_id = u.id
            JOIN challenges c ON a.challenge_id = c.id
            WHERE a.content ILIKE $1
            ORDER BY a.created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, data)
    };

    let total_pages = ((total.0 as f64) / (page_size as f64)).ceil() as i64;

    let answers: Vec<AnswerWithDetails> = answers_data
        .into_iter()
        .map(AnswerWithDetails::from)
        .collect();

    let showing_from = if total.0 == 0 { 0 } else { (page - 1) * page_size + 1 };
    let showing_to = (page * page_size).min(total.0);
    let page_links = PageLink::generate(page, total_pages);

    let template = AnswersListTemplate {
        admin,
        answers,
        search,
        status_filter,
        page,
        page_size,
        total_count: total.0,
        total_pages,
        showing_from,
        showing_to,
        has_prev: page > 1,
        has_next: page < total_pages,
        page_links,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn get_answer(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let answer_id = path.into_inner();

    let answer_data = sqlx::query_as::<_, AnswerDetailRow>(
        r#"
        SELECT a.id, a.challenge_id, a.user_id, a.content, a.score, a.ai_feedback,
               a.like_count, a.comment_count, a.view_count, a.status, a.created_at, a.updated_at,
               u.name as user_name, c.title as challenge_title, a.moderated_at, a.moderation_reason
        FROM answers a
        JOIN users u ON a.user_id = u.id
        JOIN challenges c ON a.challenge_id = c.id
        WHERE a.id = $1
        "#,
    )
    .bind(answer_id)
    .fetch_optional(pool.get_ref())
    .await;

    let row = match answer_data {
        Ok(Some(data)) => data,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/answers"))
                .finish()
        }
    };

    let template = AnswerDetailTemplate {
        admin,
        answer: Answer {
            id: row.id,
            challenge_id: row.challenge_id,
            user_id: row.user_id,
            content: row.content,
            score: row.score,
            ai_feedback: row.ai_feedback,
            like_count: row.like_count,
            comment_count: row.comment_count,
            view_count: row.view_count,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        user_name: row.user_name,
        challenge_title: row.challenge_title,
        is_moderated: row.moderated_at.is_some(),
        moderation_reason: row.moderation_reason,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn moderate_answer_form(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let answer_id = path.into_inner();

    let answer_data = sqlx::query_as::<_, AnswerModerateRow>(
        r#"
        SELECT a.id, a.challenge_id, a.user_id, a.content, a.score, a.ai_feedback,
               a.like_count, a.comment_count, a.view_count, a.status, a.created_at, a.updated_at,
               u.name as user_name
        FROM answers a
        JOIN users u ON a.user_id = u.id
        WHERE a.id = $1
        "#,
    )
    .bind(answer_id)
    .fetch_optional(pool.get_ref())
    .await;

    let row = match answer_data {
        Ok(Some(data)) => data,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/answers"))
                .finish()
        }
    };

    let template = ModerateAnswerTemplate {
        admin,
        answer: Answer {
            id: row.id,
            challenge_id: row.challenge_id,
            user_id: row.user_id,
            content: row.content,
            score: row.score,
            ai_feedback: row.ai_feedback,
            like_count: row.like_count,
            comment_count: row.comment_count,
            view_count: row.view_count,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        user_name: row.user_name,
        error: None,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn moderate_answer(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
    form: web::Form<ModerateContentRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let answer_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE answers
        SET moderated_at = NOW(),
            moderated_by = $2,
            moderation_reason = $3,
            status = 'moderated',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(answer_id)
    .bind(admin_id)
    .bind(&form.reason)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log_audit(
                &pool,
                admin_id,
                "moderate",
                "answer",
                Some(answer_id),
                Some(serde_json::json!({ "reason": form.reason })),
                &req,
            )
            .await;
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/answers/{}", answer_id)))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to moderate answer: {}", e);
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/answers/{}/moderate", answer_id)))
                .finish()
        }
    }
}

pub async fn unmoderate_answer(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let answer_id = path.into_inner();

    let _ = sqlx::query(
        r#"
        UPDATE answers
        SET moderated_at = NULL,
            moderated_by = NULL,
            moderation_reason = NULL,
            status = 'active',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(answer_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "unmoderate", "answer", Some(answer_id), None, &req).await;

    HttpResponse::Found()
        .insert_header(("Location", format!("/admin/answers/{}", answer_id)))
        .finish()
}

pub async fn delete_answer(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let answer_id = path.into_inner();

    let _ = sqlx::query(
        r#"UPDATE answers SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(answer_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "delete", "answer", Some(answer_id), None, &req).await;

    HttpResponse::Found()
        .insert_header(("Location", "/admin/answers"))
        .finish()
}

// Helper functions
async fn get_admin_from_session(pool: &PgPool, session: &Session) -> Option<AdminUser> {
    let admin_id = get_admin_user_id(session)?;
    sqlx::query_as::<_, AdminUser>(r#"SELECT * FROM admin_users WHERE id = $1"#)
        .bind(admin_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

fn redirect_to_login() -> HttpResponse {
    HttpResponse::Found()
        .insert_header(("Location", "/admin/login"))
        .finish()
}

async fn log_audit(
    pool: &PgPool,
    admin_id: Uuid,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    details: Option<serde_json::Value>,
    req: &HttpRequest,
) {
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let _ = sqlx::query(
        r#"
        INSERT INTO admin_audit_logs (admin_user_id, action, entity_type, entity_id, details, ip_address)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(admin_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(details)
    .bind(&ip_address)
    .execute(pool)
    .await;
}
