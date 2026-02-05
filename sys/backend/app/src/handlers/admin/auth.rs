use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use askama::Template;
use sqlx::PgPool;

use crate::middleware::admin_auth::{clear_admin_session, set_admin_session};
use crate::models::admin::{AdminUser, LoginRequest};

#[derive(Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate {
    pub error: Option<String>,
}

pub async fn login_page(session: Session) -> HttpResponse {
    // If already logged in, redirect to dashboard
    if session.get::<String>("admin_user_id").unwrap_or(None).is_some() {
        return HttpResponse::Found()
            .insert_header(("Location", "/admin"))
            .finish();
    }

    let template = LoginTemplate { error: None };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn login_submit(
    pool: web::Data<PgPool>,
    session: Session,
    req: HttpRequest,
    form: web::Form<LoginRequest>,
) -> HttpResponse {
    let render_error = |msg: &str| {
        let template = LoginTemplate {
            error: Some(msg.to_string()),
        };
        HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render().unwrap_or_default())
    };

    // Find admin user by email
    let admin = sqlx::query_as::<_, AdminUser>(
        r#"SELECT * FROM admin_users WHERE email = $1 AND status = 'active'"#,
    )
    .bind(&form.email)
    .fetch_optional(pool.get_ref())
    .await;

    let admin = match admin {
        Ok(Some(a)) => a,
        Ok(None) => return render_error("Invalid email or password"),
        Err(e) => {
            tracing::error!("Database error during login: {}", e);
            return render_error("An error occurred. Please try again.");
        }
    };

    // Verify password
    let parsed_hash = match PasswordHash::new(&admin.password_hash) {
        Ok(h) => h,
        Err(_) => return render_error("Invalid email or password"),
    };

    if Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return render_error("Invalid email or password");
    }

    // Update last login
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let _ = sqlx::query(
        r#"UPDATE admin_users SET last_login_at = NOW(), updated_at = NOW() WHERE id = $1"#,
    )
    .bind(admin.id)
    .execute(pool.get_ref())
    .await;

    // Log the login
    let _ = sqlx::query(
        r#"
        INSERT INTO admin_audit_logs (admin_user_id, action, entity_type, entity_id, ip_address)
        VALUES ($1, 'login', 'admin_user', $1, $2)
        "#,
    )
    .bind(admin.id)
    .bind(&ip_address)
    .execute(pool.get_ref())
    .await;

    // Set session
    set_admin_session(&session, admin.id);

    HttpResponse::Found()
        .insert_header(("Location", "/admin"))
        .finish()
}

pub async fn logout(pool: web::Data<PgPool>, session: Session, req: HttpRequest) -> HttpResponse {
    if let Some(admin_id) = session
        .get::<String>("admin_user_id")
        .ok()
        .flatten()
        .and_then(|s| uuid::Uuid::parse_str(&s).ok())
    {
        let ip_address = req
            .connection_info()
            .realip_remote_addr()
            .map(|s| s.to_string());

        let _ = sqlx::query(
            r#"
            INSERT INTO admin_audit_logs (admin_user_id, action, entity_type, entity_id, ip_address)
            VALUES ($1, 'logout', 'admin_user', $1, $2)
            "#,
        )
        .bind(admin_id)
        .bind(&ip_address)
        .execute(pool.get_ref())
        .await;
    }

    clear_admin_session(&session);

    HttpResponse::Found()
        .insert_header(("Location", "/admin/login"))
        .finish()
}

// Helper function to hash password
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}
