use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Category;
use crate::utils;

pub async fn list_categories(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as::<_, Category>(
        r#"
        SELECT * FROM categories
        WHERE status = 'active'
        ORDER BY sort_order ASC, name ASC
        "#,
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(categories) => utils::success(categories),
        Err(e) => {
            tracing::error!("Failed to fetch categories: {}", e);
            utils::internal_error("Failed to fetch categories")
        }
    }
}

pub async fn get_category(pool: web::Data<PgPool>, path: web::Path<Uuid>) -> HttpResponse {
    let category_id = path.into_inner();

    let result = sqlx::query_as::<_, Category>(
        r#"
        SELECT * FROM categories
        WHERE id = $1 AND status = 'active'
        "#,
    )
    .bind(category_id)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(category)) => utils::success(category),
        Ok(None) => utils::not_found("Category not found"),
        Err(e) => {
            tracing::error!("Failed to fetch category: {}", e);
            utils::internal_error("Failed to fetch category")
        }
    }
}
