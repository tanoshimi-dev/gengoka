use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminCreateChallengeRequest, AdminPaginationParams, AdminUpdateChallengeRequest, AdminUser};
use crate::models::{Category, Challenge};

#[derive(FromRow)]
struct ChallengeRow {
    id: Uuid,
    category_id: Uuid,
    title: String,
    description: Option<String>,
    char_limit: i32,
    release_date: Option<NaiveDate>,
    answer_count: i32,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    category_name: String,
}

#[derive(Template)]
#[template(path = "admin/challenges/list.html")]
pub struct ChallengesListTemplate {
    pub admin: AdminUser,
    pub challenges: Vec<ChallengeWithCategoryName>,
    pub categories: Vec<Category>,
    pub search: String,
    pub category_filter: String,
    pub page: i64,
    pub total_pages: i64,
    pub has_prev: bool,
    pub has_next: bool,
}

pub struct ChallengeWithCategoryName {
    pub challenge: Challenge,
    pub category_name: String,
}

impl From<ChallengeRow> for ChallengeWithCategoryName {
    fn from(row: ChallengeRow) -> Self {
        Self {
            challenge: Challenge {
                id: row.id,
                category_id: row.category_id,
                title: row.title,
                description: row.description,
                char_limit: row.char_limit,
                release_date: row.release_date,
                answer_count: row.answer_count,
                status: row.status,
                created_at: row.created_at,
                updated_at: row.updated_at,
            },
            category_name: row.category_name,
        }
    }
}

#[derive(Template)]
#[template(path = "admin/challenges/form.html")]
pub struct ChallengeFormTemplate {
    pub admin: AdminUser,
    pub challenge: Option<Challenge>,
    pub categories: Vec<Category>,
    pub error: Option<String>,
    pub is_edit: bool,
}

#[derive(Template)]
#[template(path = "admin/challenges/detail.html")]
pub struct ChallengeDetailTemplate {
    pub admin: AdminUser,
    pub challenge: Challenge,
    pub category: Category,
    pub answer_count: i64,
}

pub async fn list_challenges(
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
    let category_filter: Option<Uuid> = query
        .status
        .as_ref()
        .and_then(|s| Uuid::parse_str(s).ok());

    let search_pattern = format!("%{}%", search);

    // Get all categories for filter dropdown
    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE status = 'active' ORDER BY name"#,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    // Build query based on filters
    let (total, challenges_data): ((i64,), Vec<ChallengeRow>) = if let Some(cat_id) = category_filter {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM challenges WHERE category_id = $1 AND (title ILIKE $2 OR description ILIKE $2)"#,
        )
        .bind(cat_id)
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, ChallengeRow>(
            r#"
            SELECT ch.id, ch.category_id, ch.title, ch.description, ch.char_limit, ch.release_date,
                   ch.answer_count, ch.status, ch.created_at, ch.updated_at, ca.name as category_name
            FROM challenges ch
            JOIN categories ca ON ch.category_id = ca.id
            WHERE ch.category_id = $1 AND (ch.title ILIKE $2 OR ch.description ILIKE $2)
            ORDER BY ch.release_date DESC NULLS LAST, ch.created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(cat_id)
        .bind(&search_pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

        (total, data)
    } else {
        let total = sqlx::query_as::<_, (i64,)>(
            r#"SELECT COUNT(*) FROM challenges WHERE title ILIKE $1 OR description ILIKE $1"#,
        )
        .bind(&search_pattern)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or((0,));

        let data = sqlx::query_as::<_, ChallengeRow>(
            r#"
            SELECT ch.id, ch.category_id, ch.title, ch.description, ch.char_limit, ch.release_date,
                   ch.answer_count, ch.status, ch.created_at, ch.updated_at, ca.name as category_name
            FROM challenges ch
            JOIN categories ca ON ch.category_id = ca.id
            WHERE ch.title ILIKE $1 OR ch.description ILIKE $1
            ORDER BY ch.release_date DESC NULLS LAST, ch.created_at DESC
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

    let challenges: Vec<ChallengeWithCategoryName> = challenges_data
        .into_iter()
        .map(ChallengeWithCategoryName::from)
        .collect();

    let template = ChallengesListTemplate {
        admin,
        challenges,
        categories,
        search,
        category_filter: category_filter.map(|u| u.to_string()).unwrap_or_default(),
        page,
        total_pages,
        has_prev: page > 1,
        has_next: page < total_pages,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn new_challenge_form(pool: web::Data<PgPool>, session: Session) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE status = 'active' ORDER BY name"#,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let template = ChallengeFormTemplate {
        admin,
        challenge: None,
        categories,
        error: None,
        is_edit: false,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn create_challenge(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    form: web::Form<AdminCreateChallengeRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;

    // Get category to use its char_limit if not specified
    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1"#,
    )
    .bind(form.category_id)
    .fetch_optional(pool.get_ref())
    .await;

    let char_limit = form.char_limit.unwrap_or_else(|| {
        category.as_ref().ok().and_then(|c| c.as_ref()).map(|c| c.char_limit).unwrap_or(30)
    });

    let status = form.status.clone().unwrap_or_else(|| "active".to_string());

    let result = sqlx::query_as::<_, Challenge>(
        r#"
        INSERT INTO challenges (category_id, title, description, char_limit, release_date, status)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(form.category_id)
    .bind(&form.title)
    .bind(&form.description)
    .bind(char_limit)
    .bind(form.release_date)
    .bind(&status)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(challenge) => {
            log_audit(&pool, admin_id, "create", "challenge", Some(challenge.id), &req).await;
            HttpResponse::Found()
                .insert_header(("Location", "/admin/challenges"))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to create challenge: {}", e);
            let categories = sqlx::query_as::<_, Category>(
                r#"SELECT * FROM categories WHERE status = 'active' ORDER BY name"#,
            )
            .fetch_all(pool.get_ref())
            .await
            .unwrap_or_default();

            let template = ChallengeFormTemplate {
                admin,
                challenge: None,
                categories,
                error: Some("Failed to create challenge".to_string()),
                is_edit: false,
            };
            HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap_or_default())
        }
    }
}

pub async fn get_challenge(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let challenge_id = path.into_inner();

    let challenge = sqlx::query_as::<_, Challenge>(
        r#"SELECT * FROM challenges WHERE id = $1"#,
    )
    .bind(challenge_id)
    .fetch_optional(pool.get_ref())
    .await;

    let challenge = match challenge {
        Ok(Some(c)) => c,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/challenges"))
                .finish()
        }
    };

    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1"#,
    )
    .bind(challenge.category_id)
    .fetch_one(pool.get_ref())
    .await;

    let category = match category {
        Ok(c) => c,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/challenges"))
                .finish()
        }
    };

    let answer_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE challenge_id = $1"#,
    )
    .bind(challenge_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let template = ChallengeDetailTemplate {
        admin,
        challenge,
        category,
        answer_count: answer_count.0,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn edit_challenge_form(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let challenge_id = path.into_inner();

    let challenge = sqlx::query_as::<_, Challenge>(
        r#"SELECT * FROM challenges WHERE id = $1"#,
    )
    .bind(challenge_id)
    .fetch_optional(pool.get_ref())
    .await;

    let challenge = match challenge {
        Ok(Some(c)) => c,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/challenges"))
                .finish()
        }
    };

    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE status = 'active' ORDER BY name"#,
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let template = ChallengeFormTemplate {
        admin,
        challenge: Some(challenge),
        categories,
        error: None,
        is_edit: true,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn update_challenge(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
    form: web::Form<AdminUpdateChallengeRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let challenge_id = path.into_inner();

    let result = sqlx::query(
        r#"
        UPDATE challenges
        SET category_id = COALESCE($2, category_id),
            title = COALESCE($3, title),
            description = COALESCE($4, description),
            char_limit = COALESCE($5, char_limit),
            release_date = COALESCE($6, release_date),
            status = COALESCE($7, status),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(challenge_id)
    .bind(form.category_id)
    .bind(&form.title)
    .bind(&form.description)
    .bind(form.char_limit)
    .bind(form.release_date)
    .bind(&form.status)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log_audit(&pool, admin_id, "update", "challenge", Some(challenge_id), &req).await;
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/challenges/{}", challenge_id)))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to update challenge: {}", e);
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/challenges/{}/edit", challenge_id)))
                .finish()
        }
    }
}

pub async fn delete_challenge(
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
    let challenge_id = path.into_inner();

    let _ = sqlx::query(
        r#"UPDATE challenges SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(challenge_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "delete", "challenge", Some(challenge_id), &req).await;

    HttpResponse::Found()
        .insert_header(("Location", "/admin/challenges"))
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
    req: &HttpRequest,
) {
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let _ = sqlx::query(
        r#"
        INSERT INTO admin_audit_logs (admin_user_id, action, entity_type, entity_id, ip_address)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(admin_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(&ip_address)
    .execute(pool)
    .await;
}
