use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminPaginationParams, AdminUser, PageLink, SuspendUserRequest};
use crate::models::User;

#[derive(Template)]
#[template(path = "admin/users/list.html")]
pub struct UsersListTemplate {
    pub admin: AdminUser,
    pub users: Vec<UserWithStats>,
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

pub struct UserWithStats {
    pub user: User,
    pub answer_count: i64,
    pub is_suspended: bool,
}

#[derive(Template)]
#[template(path = "admin/users/detail.html")]
pub struct UserDetailTemplate {
    pub admin: AdminUser,
    pub user: User,
    pub answer_count: i64,
    pub comment_count: i64,
    pub follower_count: i64,
    pub following_count: i64,
    pub is_suspended: bool,
    pub suspended_reason: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/users/suspend.html")]
pub struct SuspendUserTemplate {
    pub admin: AdminUser,
    pub user: User,
    pub error: Option<String>,
}

pub async fn list_users(
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

    // Build query based on status filter
    let (total, users): ((i64,), Vec<User>) = if status_filter == "suspended" {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM users WHERE suspended_at IS NOT NULL AND (name ILIKE $1 OR email ILIKE $1)"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE suspended_at IS NOT NULL AND (name ILIKE $1 OR email ILIKE $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, users)
    } else if status_filter == "active" {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM users WHERE suspended_at IS NULL AND status = 'active' AND (name ILIKE $1 OR email ILIKE $1)"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE suspended_at IS NULL AND status = 'active' AND (name ILIKE $1 OR email ILIKE $1)
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, users)
    } else {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM users WHERE name ILIKE $1 OR email ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE name ILIKE $1 OR email ILIKE $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, users)
    };

    let total_pages = ((total.0 as f64) / (page_size as f64)).ceil() as i64;

    // Get answer counts for each user
    let mut users_with_stats = Vec::new();
    for user in users {
        let answer_count: (i64,) = sqlx::query_as(
            r#"SELECT COUNT(*) FROM answers WHERE user_id = $1"#,
        )
        .bind(user.id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        // Check if user is suspended (we need to query the raw column)
        let suspended: (Option<chrono::DateTime<chrono::Utc>>,) = sqlx::query_as(
            r#"SELECT suspended_at FROM users WHERE id = $1"#,
        )
        .bind(user.id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((None,));

        users_with_stats.push(UserWithStats {
            user,
            answer_count: answer_count.0,
            is_suspended: suspended.0.is_some(),
        });
    }

    let showing_from = if total.0 == 0 { 0 } else { (page - 1) * page_size + 1 };
    let showing_to = (page * page_size).min(total.0);
    let page_links = PageLink::generate(page, total_pages);

    let template = UsersListTemplate {
        admin,
        users: users_with_stats,
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

pub async fn get_user(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let user_id = path.into_inner();

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/users"))
                .finish()
        }
    };

    // Get stats
    let answer_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE user_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let comment_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM comments WHERE user_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let follower_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE following_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let following_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE follower_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // Get suspension info
    let suspension_info: (Option<chrono::DateTime<chrono::Utc>>, Option<String>) = sqlx::query_as(
        r#"SELECT suspended_at, suspended_reason FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((None, None));

    let template = UserDetailTemplate {
        admin,
        user,
        answer_count: answer_count.0,
        comment_count: comment_count.0,
        follower_count: follower_count.0,
        following_count: following_count.0,
        is_suspended: suspension_info.0.is_some(),
        suspended_reason: suspension_info.1,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn suspend_user_form(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let user_id = path.into_inner();

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/users"))
                .finish()
        }
    };

    let template = SuspendUserTemplate {
        admin,
        user,
        error: None,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn suspend_user(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
    form: web::Form<SuspendUserRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let user_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE users
        SET suspended_at = NOW(),
            suspended_reason = $2,
            suspended_by = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(&form.reason)
    .bind(admin_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log_audit(
                &pool,
                admin_id,
                "suspend",
                "user",
                Some(user_id),
                Some(serde_json::json!({ "reason": form.reason })),
                &req,
            )
            .await;
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/users/{}", user_id)))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to suspend user: {}", e);
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/users/{}/suspend", user_id)))
                .finish()
        }
    }
}

pub async fn unsuspend_user(
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
    let user_id = path.into_inner();

    let _ = sqlx::query(
        r#"
        UPDATE users
        SET suspended_at = NULL,
            suspended_reason = NULL,
            suspended_by = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "unsuspend", "user", Some(user_id), None, &req).await;

    HttpResponse::Found()
        .insert_header(("Location", format!("/admin/users/{}", user_id)))
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
