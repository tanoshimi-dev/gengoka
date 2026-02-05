use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use askama::Template;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminUser, GeminiSettings, SchedulerSettings, SystemConfig};

#[derive(Template)]
#[template(path = "admin/settings/gemini.html")]
pub struct GeminiSettingsTemplate {
    pub admin: AdminUser,
    pub settings: GeminiSettings,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/settings/scheduler.html")]
pub struct SchedulerSettingsTemplate {
    pub admin: AdminUser,
    pub settings: SchedulerSettings,
    pub success: bool,
    pub error: Option<String>,
}

pub async fn gemini_settings_page(
    pool: web::Data<PgPool>,
    session: Session,
    config: web::Data<Config>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    // Get saved settings or use defaults from config
    let saved_settings = get_system_config(&pool, "gemini_settings").await;
    let settings = if let Some(value) = saved_settings {
        serde_json::from_value(value).unwrap_or_else(|_| GeminiSettings {
            model: config.gemini.model.clone(),
            max_tokens: config.gemini.max_tokens,
            temperature: config.gemini.temperature,
        })
    } else {
        GeminiSettings {
            model: config.gemini.model.clone(),
            max_tokens: config.gemini.max_tokens,
            temperature: config.gemini.temperature,
        }
    };

    let template = GeminiSettingsTemplate {
        admin,
        settings,
        success: false,
        error: None,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn update_gemini_settings(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    form: web::Form<GeminiSettings>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;

    let settings = GeminiSettings {
        model: form.model.clone(),
        max_tokens: form.max_tokens,
        temperature: form.temperature,
    };

    let value = serde_json::to_value(&settings).unwrap_or_default();

    let result = save_system_config(&pool, "gemini_settings", value.clone(), Some(admin_id)).await;

    let template = match result {
        Ok(_) => {
            log_audit(
                &pool,
                admin_id,
                "update",
                "system_config",
                None,
                Some(serde_json::json!({ "key": "gemini_settings", "value": value })),
                &req,
            )
            .await;
            GeminiSettingsTemplate {
                admin,
                settings,
                success: true,
                error: None,
            }
        }
        Err(e) => {
            tracing::error!("Failed to save gemini settings: {}", e);
            GeminiSettingsTemplate {
                admin,
                settings,
                success: false,
                error: Some("Failed to save settings".to_string()),
            }
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn scheduler_settings_page(
    pool: web::Data<PgPool>,
    session: Session,
    config: web::Data<Config>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    // Get saved settings or use defaults from config
    let saved_settings = get_system_config(&pool, "scheduler_settings").await;
    let settings = if let Some(value) = saved_settings {
        serde_json::from_value(value).unwrap_or_else(|_| SchedulerSettings {
            enabled: config.challenge_generation.enabled,
            daily_count: config.challenge_generation.daily_count,
            cron_schedule: config.challenge_generation.cron_schedule.clone(),
        })
    } else {
        SchedulerSettings {
            enabled: config.challenge_generation.enabled,
            daily_count: config.challenge_generation.daily_count,
            cron_schedule: config.challenge_generation.cron_schedule.clone(),
        }
    };

    let template = SchedulerSettingsTemplate {
        admin,
        settings,
        success: false,
        error: None,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn update_scheduler_settings(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    form: web::Form<SchedulerSettings>,
) -> HttpResponse {
    let admin = match get_admin_from_session(&pool, &session).await {
        Some(a) => a,
        None => return redirect_to_login(),
    };

    let admin_id = admin.id;

    let settings = SchedulerSettings {
        enabled: form.enabled,
        daily_count: form.daily_count,
        cron_schedule: form.cron_schedule.clone(),
    };

    let value = serde_json::to_value(&settings).unwrap_or_default();

    let result = save_system_config(&pool, "scheduler_settings", value.clone(), Some(admin_id)).await;

    let template = match result {
        Ok(_) => {
            log_audit(
                &pool,
                admin_id,
                "update",
                "system_config",
                None,
                Some(serde_json::json!({ "key": "scheduler_settings", "value": value })),
                &req,
            )
            .await;
            SchedulerSettingsTemplate {
                admin,
                settings,
                success: true,
                error: None,
            }
        }
        Err(e) => {
            tracing::error!("Failed to save scheduler settings: {}", e);
            SchedulerSettingsTemplate {
                admin,
                settings,
                success: false,
                error: Some("Failed to save settings".to_string()),
            }
        }
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
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

async fn get_system_config(pool: &PgPool, key: &str) -> Option<serde_json::Value> {
    sqlx::query_as::<_, SystemConfig>(r#"SELECT * FROM system_config WHERE key = $1"#)
        .bind(key)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
        .map(|c| c.value)
}

async fn save_system_config(
    pool: &PgPool,
    key: &str,
    value: serde_json::Value,
    updated_by: Option<Uuid>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO system_config (key, value, updated_by, updated_at)
        VALUES ($1, $2, $3, NOW())
        ON CONFLICT (key)
        DO UPDATE SET value = $2, updated_by = $3, updated_at = NOW()
        "#,
    )
    .bind(key)
    .bind(value)
    .bind(updated_by)
    .execute(pool)
    .await?;
    Ok(())
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
