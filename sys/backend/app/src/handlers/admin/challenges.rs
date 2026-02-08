use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::config::Config;
use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{
    AdminCreateChallengeRequest, AdminPaginationParams, AdminUpdateChallengeRequest, AdminUser,
    BulkGenerateRequest, BulkGenerateResponse, BulkGeneratedChallengeItem,
    BulkSaveRequest, BulkSaveResponse, PageLink,
};
use crate::models::{Category, Challenge};
use crate::services::gemini::{build_challenge_prompt, GeminiClient};

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
    pub page_size: i64,
    pub total_count: i64,
    pub total_pages: i64,
    pub showing_from: i64,
    pub showing_to: i64,
    pub has_prev: bool,
    pub has_next: bool,
    pub page_links: Vec<PageLink>,
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

    let showing_from = if total.0 == 0 { 0 } else { (page - 1) * page_size + 1 };
    let showing_to = (page * page_size).min(total.0);
    let page_links = PageLink::generate(page, total_pages);

    let template = ChallengesListTemplate {
        admin,
        challenges,
        categories,
        search,
        category_filter: category_filter.map(|u| u.to_string()).unwrap_or_default(),
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

// ============ Bulk Challenge Generation ============

#[derive(Template)]
#[template(path = "admin/challenges/bulk.html")]
pub struct ChallengeBulkTemplate {
    pub admin: AdminUser,
    pub categories: Vec<Category>,
}

pub async fn bulk_challenge_form(pool: web::Data<PgPool>, session: Session) -> HttpResponse {
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

    let template = ChallengeBulkTemplate { admin, categories };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn bulk_generate_challenges(
    pool: web::Data<PgPool>,
    session: Session,
    config: web::Data<Config>,
    body: web::Json<BulkGenerateRequest>,
) -> HttpResponse {
    if get_admin_from_session(&pool, &session).await.is_none() {
        return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Unauthorized"}));
    }

    if body.items.is_empty() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No categories specified"}));
    }

    let gemini_client = match GeminiClient::new(config.gemini.clone()) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create Gemini client: {}", e);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to initialize AI client"}));
        }
    };

    // Fetch all requested categories
    let category_ids: Vec<Uuid> = body.items.iter().map(|i| i.category_id).collect();
    let categories = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = ANY($1) AND status = 'active'"#,
    )
    .bind(&category_ids)
    .fetch_all(pool.get_ref())
    .await;

    let categories = match categories {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to fetch categories: {}", e);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": "Failed to fetch categories"}));
        }
    };

    let category_map: std::collections::HashMap<Uuid, &Category> =
        categories.iter().map(|c| (c.id, c)).collect();

    let mut all_challenges = Vec::new();

    for item in &body.items {
        let category = match category_map.get(&item.category_id) {
            Some(c) => *c,
            None => {
                tracing::warn!("Category not found: {}", item.category_id);
                continue;
            }
        };

        let prompt = build_challenge_prompt(category, item.count);

        match gemini_client.generate_challenges(prompt).await {
            Ok(response) => {
                for challenge in response.challenges {
                    all_challenges.push(BulkGeneratedChallengeItem {
                        category_id: category.id,
                        category_name: category.name.clone(),
                        title: challenge.title,
                        description: challenge.description,
                        char_limit: challenge.char_limit,
                    });
                }
            }
            Err(e) => {
                tracing::error!(
                    "Failed to generate challenges for category {}: {}",
                    category.name,
                    e
                );
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("AI generation failed for category '{}': {}", category.name, e)
                }));
            }
        }
    }

    let total = all_challenges.len();
    HttpResponse::Ok().json(BulkGenerateResponse {
        challenges: all_challenges,
        total,
    })
}

pub async fn bulk_save_challenges(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    body: web::Json<BulkSaveRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => {
            return HttpResponse::Unauthorized().json(serde_json::json!({"error": "Unauthorized"}))
        }
    };

    if body.challenges.is_empty() {
        return HttpResponse::BadRequest()
            .json(serde_json::json!({"error": "No challenges to save"}));
    }

    let today = Utc::now().date_naive();
    let now = Utc::now();
    let mut saved_count = 0usize;

    for challenge in &body.challenges {
        let result = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO challenges (category_id, title, description, char_limit, release_date, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, 'active', $6, $6)
            RETURNING id
            "#,
        )
        .bind(challenge.category_id)
        .bind(&challenge.title)
        .bind(&challenge.description)
        .bind(challenge.char_limit)
        .bind(today)
        .bind(now)
        .fetch_one(pool.get_ref())
        .await;

        match result {
            Ok(challenge_id) => {
                saved_count += 1;
                log_audit(
                    &pool,
                    admin.id,
                    "bulk_create",
                    "challenge",
                    Some(challenge_id),
                    &req,
                )
                .await;
            }
            Err(e) => {
                tracing::error!("Failed to save challenge '{}': {}", challenge.title, e);
            }
        }
    }

    HttpResponse::Ok().json(BulkSaveResponse { saved_count })
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
