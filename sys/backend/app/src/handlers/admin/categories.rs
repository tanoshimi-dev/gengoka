use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminPaginationParams, AdminUser, CreateCategoryRequest, UpdateCategoryRequest};
use crate::models::Category;

#[derive(Template)]
#[template(path = "admin/categories/list.html")]
pub struct CategoriesListTemplate {
    pub admin: AdminUser,
    pub categories: Vec<Category>,
    pub search: String,
    pub page: i64,
    pub total_pages: i64,
    pub has_prev: bool,
    pub has_next: bool,
}

#[derive(Template)]
#[template(path = "admin/categories/form.html")]
pub struct CategoryFormTemplate {
    pub admin: AdminUser,
    pub category: Option<Category>,
    pub error: Option<String>,
    pub is_edit: bool,
}

#[derive(Template)]
#[template(path = "admin/categories/detail.html")]
pub struct CategoryDetailTemplate {
    pub admin: AdminUser,
    pub category: Category,
    pub challenge_count: i64,
}

pub async fn list_categories(
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

    let search_pattern = format!("%{}%", search);

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM categories WHERE name ILIKE $1 OR description ILIKE $1"#,
    )
    .bind(&search_pattern)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let total_pages = ((total.0 as f64) / (page_size as f64)).ceil() as i64;

    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT * FROM categories
        WHERE name ILIKE $1 OR description ILIKE $1
        ORDER BY sort_order ASC, name ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(&search_pattern)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let template = CategoriesListTemplate {
        admin,
        categories,
        search,
        page,
        total_pages,
        has_prev: page > 1,
        has_next: page < total_pages,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn new_category_form(pool: web::Data<PgPool>, session: Session) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let template = CategoryFormTemplate {
        admin,
        category: None,
        error: None,
        is_edit: false,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn create_category(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    form: web::Form<CreateCategoryRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;

    let char_limit = form.char_limit.unwrap_or(30);
    let sort_order = form.sort_order.unwrap_or(0);

    let result = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name, description, icon, color, char_limit, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
    )
    .bind(&form.name)
    .bind(&form.description)
    .bind(&form.icon)
    .bind(&form.color)
    .bind(char_limit)
    .bind(sort_order)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(category) => {
            log_audit(&pool, admin_id, "create", "category", Some(category.id), &req).await;
            HttpResponse::Found()
                .insert_header(("Location", "/admin/categories"))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to create category: {}", e);
            let template = CategoryFormTemplate {
                admin,
                category: None,
                error: Some("Failed to create category".to_string()),
                is_edit: false,
            };
            HttpResponse::Ok()
                .content_type("text/html")
                .body(template.render().unwrap_or_default())
        }
    }
}

pub async fn get_category(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let category_id = path.into_inner();

    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1"#,
    )
    .bind(category_id)
    .fetch_optional(pool.get_ref())
    .await;

    let category = match category {
        Ok(Some(c)) => c,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/categories"))
                .finish()
        }
    };

    let challenge_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM challenges WHERE category_id = $1"#,
    )
    .bind(category_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let template = CategoryDetailTemplate {
        admin,
        category,
        challenge_count: challenge_count.0,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn edit_category_form(
    pool: web::Data<PgPool>,
    session: Session,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let category_id = path.into_inner();

    let category = sqlx::query_as::<_, Category>(
        r#"SELECT * FROM categories WHERE id = $1"#,
    )
    .bind(category_id)
    .fetch_optional(pool.get_ref())
    .await;

    let category = match category {
        Ok(Some(c)) => c,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/categories"))
                .finish()
        }
    };

    let template = CategoryFormTemplate {
        admin,
        category: Some(category),
        error: None,
        is_edit: true,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn update_category(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    path: web::Path<Uuid>,
    form: web::Form<UpdateCategoryRequest>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;
    let category_id = path.into_inner();

    // Build dynamic update query
    let result = sqlx::query(
        r#"
        UPDATE categories
        SET name = COALESCE($2, name),
            description = COALESCE($3, description),
            icon = COALESCE($4, icon),
            color = COALESCE($5, color),
            char_limit = COALESCE($6, char_limit),
            sort_order = COALESCE($7, sort_order),
            status = COALESCE($8, status),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(category_id)
    .bind(&form.name)
    .bind(&form.description)
    .bind(&form.icon)
    .bind(&form.color)
    .bind(form.char_limit)
    .bind(form.sort_order)
    .bind(&form.status)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            log_audit(&pool, admin_id, "update", "category", Some(category_id), &req).await;
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/categories/{}", category_id)))
                .finish()
        }
        Err(e) => {
            tracing::error!("Failed to update category: {}", e);
            HttpResponse::Found()
                .insert_header(("Location", format!("/admin/categories/{}/edit", category_id)))
                .finish()
        }
    }
}

pub async fn delete_category(
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
    let category_id = path.into_inner();

    // Soft delete by setting status to 'deleted'
    let _ = sqlx::query(
        r#"UPDATE categories SET status = 'deleted', updated_at = NOW() WHERE id = $1"#,
    )
    .bind(category_id)
    .execute(pool.get_ref())
    .await;

    log_audit(&pool, admin_id, "delete", "category", Some(category_id), &req).await;

    HttpResponse::Found()
        .insert_header(("Location", "/admin/categories"))
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
