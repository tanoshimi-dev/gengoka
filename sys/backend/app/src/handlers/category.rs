use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Category, CategoryResponse};
use crate::utils;

pub async fn list_categories(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as::<_, (Uuid, String, Option<String>, Option<String>, Option<String>, i64)>(
        r#"
        SELECT c.id, c.name, c.description, c.icon, c.color,
               COUNT(ch.id) AS challenge_count
        FROM categories c
        LEFT JOIN challenges ch ON ch.category_id = c.id AND ch.status = 'active'
        WHERE c.status = 'active'
        GROUP BY c.id, c.name, c.description, c.icon, c.color, c.sort_order
        ORDER BY c.sort_order ASC, c.name ASC
        "#,
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(rows) => {
            let categories: Vec<CategoryResponse> = rows
                .into_iter()
                .map(|(id, name, description, icon, color, challenge_count)| CategoryResponse {
                    id,
                    name,
                    description,
                    icon_name: icon,
                    color_hex: color,
                    challenge_count,
                })
                .collect();
            utils::success(categories)
        }
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
