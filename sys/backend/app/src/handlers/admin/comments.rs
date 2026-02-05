use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminPaginationParams, AdminUser, ModerateContentRequest};
use crate::models::Comment;

#[derive(FromRow)]
struct CommentRow {
    id: Uuid,
    answer_id: Uuid,
    user_id: Uuid,
    content: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
    answer_preview: String,
    moderated_at: Option<DateTime<Utc>>,
}

#[derive(FromRow)]
struct CommentDetailRow {
    id: Uuid,
    answer_id: Uuid,
    user_id: Uuid,
    content: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
    answer_preview: String,
    moderated_at: Option<DateTime<Utc>>,
    moderation_reason: Option<String>,
}

#[derive(FromRow)]
struct CommentModerateRow {
    id: Uuid,
    answer_id: Uuid,
    user_id: Uuid,
    content: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    user_name: String,
}

#[derive(Template)]
#[template(path = "admin/comments/list.html")]
pub struct CommentsListTemplate {
    pub admin: AdminUser,
    pub comments: Vec<CommentWithDetails>,
    pub search: String,
    pub status_filter: String,
    pub page: i64,
    pub total_pages: i64,
    pub has_prev: bool,
    pub has_next: bool,
}

pub struct CommentWithDetails {
    pub comment: Comment,
    pub user_name: String,
    pub answer_preview: String,
    pub is_moderated: bool,
}

impl From<CommentRow> for CommentWithDetails {
    fn from(row: CommentRow) -> Self {
        Self {
            comment: Comment {
                id: row.id,
                answer_id: row.answer_id,
                user_id: row.user_id,
                content: row.content,
                status: row.status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            user_name: row.user_name,
            answer_preview: row.answer_preview,
            is_moderated: row.moderated_at.is_some(),
        }
    }
}

#[derive(Template)]
#[template(path = "admin/comments/detail.html")]
pub struct CommentDetailTemplate {
    pub admin: AdminUser,
    pub comment: Comment,
    pub user_name: String,
    pub answer_preview: String,
    pub is_moderated: bool,
    pub moderation_reason: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/comments/moderate.html")]
pub struct ModerateCommentTemplate {
    pub admin: AdminUser,
    pub comment: Comment,
    pub user_name: String,
    pub error: Option<String>,
}

pub async fn list_comments(
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

    let (total, comments_data): ((i64,), Vec<CommentRow>) = if status_filter == "moderated" {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM comments WHERE moderated_at IS NOT NULL AND content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, CommentRow>(
            r#"
            SELECT c.id, c.answer_id, c.user_id, c.content, c.status, c.created_at, c.updated_at,
                   u.name as user_name, SUBSTRING(a.content, 1, 50) as answer_preview, c.moderated_at
            FROM comments c
            JOIN users u ON c.user_id = u.id
            JOIN answers a ON c.answer_id = a.id
            WHERE c.moderated_at IS NOT NULL AND c.content ILIKE $1
            ORDER BY c.created_at DESC
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
            r#"SELECT COUNT(*) FROM comments WHERE status = 'active' AND moderated_at IS NULL AND content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, CommentRow>(
            r#"
            SELECT c.id, c.answer_id, c.user_id, c.content, c.status, c.created_at, c.updated_at,
                   u.name as user_name, SUBSTRING(a.content, 1, 50) as answer_preview, c.moderated_at
            FROM comments c
            JOIN users u ON c.user_id = u.id
            JOIN answers a ON c.answer_id = a.id
            WHERE c.status = 'active' AND c.moderated_at IS NULL AND c.content ILIKE $1
            ORDER BY c.created_at DESC
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
            r#"SELECT COUNT(*) FROM comments WHERE content ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, CommentRow>(
            r#"
            SELECT c.id, c.answer_id, c.user_id, c.content, c.status, c.created_at, c.updated_at,
                   u.name as user_name, SUBSTRING(a.content, 1, 50) as answer_preview, c.moderated_at
            FROM comments c
            JOIN users u ON c.user_id = u.id
            JOIN answers a ON c.answer_id = a.id
            WHERE c.content ILIKE $1
            ORDER BY c.created_at DESC
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

    let comments: Vec<CommentWithDetails> = comments_data
        .into_iter()
        .map(CommentWithDetails::from)
        .collect();

    let template = CommentsListTemplate {
        admin,
        comments,
        search,
        status_filter,
        page,
        total_pages,
        has_prev: page > 1,
        has_next: page < total_pages,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn get_comment(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let comment_id = path.into_inner();

    let comment_data = sqlx::query_as::<_, CommentDetailRow>(
        r#"
        SELECT c.id, c.answer_id, c.user_id, c.content, c.status, c.created_at, c.updated_at,
               u.name as user_name, SUBSTRING(a.content, 1, 100) as answer_preview,
               c.moderated_at, c.moderation_reason
        FROM comments c
        JOIN users u ON c.user_id = u.id
        JOIN answers a ON c.answer_id = a.id
        WHERE c.id = $1
        "#,
    )
    .bind(comment_id)
    .fetch_optional(pool.get_ref())
    .await;

    let row = match comment_data {
        Ok(Some(data)) => data,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/comments"))
                .finish()
        }
    };

    let template = CommentDetailTemplate {
        admin,
        comment: Comment {
            id: row.id,
            answer_id: row.answer_id,
            user_id: row.user_id,
            content: row.content,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        },
        user_name: row.user_name,
        answer_preview: row.answer_preview,
        is_moderated: row.moderated_at.is_some(),
        moderation_reason: row.moderation_reason,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn moderate_comment_form(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let comment_id = path.into_inner();

    let comment_data = sqlx::query_as::<_, CommentModerateRow>(
        r#"
        SELECT c.id, c.answer_id, c.user_id, c.content, c.status, c.created_at, c.updated_at,
               u.name as user_name
        FROM comments c
        JOIN users u ON c.user_id = u.id
        WHERE c.id = $1
        "#,
    )
    .bind(comment_id)
    .fetch_optional(pool.get_ref())
    .await;

    let row = match comment_data {
        Ok(Some(data)) => data,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/comments"))
                .finish()
        }
    };

    let template = ModerateCommentTemplate {
        admin,
        comment: Comment {
            id: row.id,
            answer_id: row.answer_id,
            user_id: row.user_id,
            content: row.content,
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

pub async fn moderate_comment(
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
    let comment_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE comments
        SET moderated_at = NOW(),
            moderated_by = $2,
            moderation_reason = $3,
            status = 'moderated',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(comment_id)
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
                "comment",
                Some(comment_id),
                Some(serde_json::json!({ "reason": form.reason })),
                &req,
            )
            .await;
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/comments/{}", comment_id)))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to moderate comment: {}", e);
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/comments/{}/moderate", comment_id)))
                .finish()
        }
    }
}

pub async fn unmoderate_comment(
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
    let comment_id = path.into_inner();

    let _ = sqlx::query(
        r#"
        UPDATE comments
        SET moderated_at = NULL,
            moderated_by = NULL,
            moderation_reason = NULL,
            status = 'active',
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(comment_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "unmoderate", "comment", Some(comment_id), None, &req).await;

    HttpResponse::Found()
        .insert_header(("Location", format!("/admin/comments/{}", comment_id)))
        .finish()
}

pub async fn delete_comment(
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
    let comment_id = path.into_inner();

    // Get the answer_id before deleting
    let answer_id: Option<(Uuid,)> = sqlx::query_as(
        r#"SELECT answer_id FROM comments WHERE id = $1"#,
    )
    .bind(comment_id)
    .fetch_optional(pool.get_ref())
    .await
    .ok()
    .flatten();

    let _ = sqlx::query(
        r#"UPDATE comments SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(comment_id)
    .execute(pool.get_ref())
    .await;

    // Update answer comment count
    if let Some((aid,)) = answer_id {
        let _ = sqlx::query(
            r#"UPDATE answers SET comment_count = comment_count - 1 WHERE id = $1"#,
        )
        .bind(aid)
        .execute(pool.get_ref())
        .await;
    }

    log_audit(&pool, admin_id, "delete", "comment", Some(comment_id), None, &req).await;

    HttpResponse::Found()
        .insert_header(("Location", "/admin/comments"))
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
