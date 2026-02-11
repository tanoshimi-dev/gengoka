use actix_session::Session;
use actix_web::{web, HttpResponse};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use askama::Template;
use rand::Rng;
use sqlx::PgPool;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::middleware::admin_auth::{
    complete_2fa_session, get_2fa_pending_user_id, get_admin_user_id,
};
use crate::models::admin::{AdminUser, TotpVerifyRequest};

// ============ Templates ============

#[derive(Template)]
#[template(path = "admin/two_factor/verify.html")]
pub struct TwoFactorVerifyTemplate {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "admin/two_factor/setup.html")]
pub struct TwoFactorSetupTemplate {
    pub admin: AdminUser,
    pub totp_enabled: bool,
    pub qr_code_base64: Option<String>,
    pub secret: Option<String>,
    pub backup_codes: Option<Vec<String>>,
    pub success: Option<String>,
    pub error: Option<String>,
}

// ============ Verify (public, for login flow) ============

pub async fn two_factor_verify_page(session: Session) -> HttpResponse {
    if get_2fa_pending_user_id(&session).is_none() {
        return HttpResponse::Found()
            .insert_header(("Location", "/admin/login"))
            .finish();
    }

    let template = TwoFactorVerifyTemplate { error: None };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn two_factor_verify_submit(
    pool: web::Data<PgPool>,
    session: Session,
    form: web::Form<TotpVerifyRequest>,
) -> HttpResponse {
    let user_id = match get_2fa_pending_user_id(&session) {
        Some(id) => id,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let render_error = |msg: &str| {
        let template = TwoFactorVerifyTemplate {
            error: Some(msg.to_string()),
        };
        HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render().unwrap_or_default())
    };

    let admin = match sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1 AND status = 'active'",
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(a)) => a,
        _ => return render_error("User not found. Please login again."),
    };

    let secret = match &admin.totp_secret {
        Some(s) => s.clone(),
        None => return render_error("2FA is not configured. Please login again."),
    };

    let code = form.code.trim().replace(" ", "").replace("-", "");

    // Try TOTP code first
    if let Ok(totp) = build_totp(&secret, &admin.email) {
        if totp.check_current(&code).unwrap_or(false) {
            complete_2fa_session(&session, admin.id);
            return HttpResponse::Found()
                .insert_header(("Location", "/admin"))
                .finish();
        }
    }

    // Try backup code
    if try_use_backup_code(pool.get_ref(), admin.id, &code).await {
        complete_2fa_session(&session, admin.id);
        return HttpResponse::Found()
            .insert_header(("Location", "/admin"))
            .finish();
    }

    render_error("Invalid verification code. Please try again.")
}

// ============ Setup (protected, requires auth) ============

pub async fn two_factor_setup_page(
    pool: web::Data<PgPool>,
    session: Session,
) -> HttpResponse {
    let admin_id = match get_admin_user_id(&session) {
        Some(id) => id,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let admin = match sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1",
    )
    .bind(admin_id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(a)) => a,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    if admin.totp_enabled {
        let template = TwoFactorSetupTemplate {
            totp_enabled: true,
            qr_code_base64: None,
            secret: None,
            backup_codes: None,
            success: None,
            error: None,
            admin,
        };
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render().unwrap_or_default());
    }

    // Generate new secret
    let secret = Secret::generate_secret();
    let secret_base32 = secret.to_encoded().to_string();

    let qr_code_base64 = match build_totp(&secret_base32, &admin.email) {
        Ok(totp) => totp.get_qr_base64().ok(),
        Err(_) => None,
    };

    // Store secret temporarily in DB (not yet enabled)
    let _ = sqlx::query(
        "UPDATE admin_users SET totp_secret = $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(&secret_base32)
    .bind(admin_id)
    .execute(pool.get_ref())
    .await;

    let template = TwoFactorSetupTemplate {
        totp_enabled: false,
        qr_code_base64,
        secret: Some(secret_base32),
        backup_codes: None,
        success: None,
        error: None,
        admin,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn two_factor_setup_confirm(
    pool: web::Data<PgPool>,
    session: Session,
    form: web::Form<TotpVerifyRequest>,
) -> HttpResponse {
    let admin_id = match get_admin_user_id(&session) {
        Some(id) => id,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let admin = match sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1",
    )
    .bind(admin_id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(a)) => a,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let secret = match &admin.totp_secret {
        Some(s) => s.clone(),
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/settings/2fa"))
                .finish();
        }
    };

    let code = form.code.trim().replace(" ", "").replace("-", "");

    let valid = match build_totp(&secret, &admin.email) {
        Ok(totp) => totp.check_current(&code).unwrap_or(false),
        Err(_) => false,
    };

    if !valid {
        let qr_code_base64 = match build_totp(&secret, &admin.email) {
            Ok(totp) => totp.get_qr_base64().ok(),
            Err(_) => None,
        };

        let template = TwoFactorSetupTemplate {
            totp_enabled: false,
            qr_code_base64,
            secret: Some(secret),
            backup_codes: None,
            success: None,
            error: Some("Invalid code. Please try again.".to_string()),
            admin,
        };
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render().unwrap_or_default());
    }

    // Enable 2FA
    let _ = sqlx::query(
        "UPDATE admin_users SET totp_enabled = TRUE, totp_enabled_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(admin_id)
    .execute(pool.get_ref())
    .await;

    // Generate backup codes
    let backup_codes = generate_backup_codes();
    store_backup_codes(pool.get_ref(), admin_id, &backup_codes).await;

    // Re-fetch admin for template
    let admin = sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1",
    )
    .bind(admin_id)
    .fetch_optional(pool.get_ref())
    .await
    .ok()
    .flatten()
    .unwrap_or(admin);

    let template = TwoFactorSetupTemplate {
        totp_enabled: true,
        qr_code_base64: None,
        secret: None,
        backup_codes: Some(backup_codes),
        success: Some("Two-factor authentication has been enabled successfully!".to_string()),
        error: None,
        admin,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

pub async fn two_factor_disable(
    pool: web::Data<PgPool>,
    session: Session,
    form: web::Form<TotpVerifyRequest>,
) -> HttpResponse {
    let admin_id = match get_admin_user_id(&session) {
        Some(id) => id,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let admin = match sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1",
    )
    .bind(admin_id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(a)) => a,
        _ => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish();
        }
    };

    let secret = match &admin.totp_secret {
        Some(s) => s.clone(),
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/settings/2fa"))
                .finish();
        }
    };

    let code = form.code.trim().replace(" ", "").replace("-", "");

    let valid = match build_totp(&secret, &admin.email) {
        Ok(totp) => totp.check_current(&code).unwrap_or(false),
        Err(_) => false,
    };

    if !valid {
        let template = TwoFactorSetupTemplate {
            totp_enabled: true,
            qr_code_base64: None,
            secret: None,
            backup_codes: None,
            success: None,
            error: Some("Invalid code. 2FA was not disabled.".to_string()),
            admin,
        };
        return HttpResponse::Ok()
            .content_type("text/html")
            .body(template.render().unwrap_or_default());
    }

    // Disable 2FA
    let _ = sqlx::query(
        "UPDATE admin_users SET totp_enabled = FALSE, totp_secret = NULL, totp_enabled_at = NULL, updated_at = NOW() WHERE id = $1",
    )
    .bind(admin_id)
    .execute(pool.get_ref())
    .await;

    // Delete backup codes
    let _ = sqlx::query("DELETE FROM admin_2fa_backup_codes WHERE admin_user_id = $1")
        .bind(admin_id)
        .execute(pool.get_ref())
        .await;

    // Re-fetch admin
    let admin = sqlx::query_as::<_, AdminUser>(
        "SELECT * FROM admin_users WHERE id = $1",
    )
    .bind(admin_id)
    .fetch_optional(pool.get_ref())
    .await
    .ok()
    .flatten()
    .unwrap_or(admin);

    let template = TwoFactorSetupTemplate {
        totp_enabled: false,
        qr_code_base64: None,
        secret: None,
        backup_codes: None,
        success: Some("Two-factor authentication has been disabled.".to_string()),
        error: None,
        admin,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

// ============ Helpers ============

fn build_totp(secret_base32: &str, email: &str) -> Result<TOTP, totp_rs::TotpUrlError> {
    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        Secret::Encoded(secret_base32.to_string())
            .to_bytes()
            .unwrap(),
        Some("Gengoka Admin".to_string()),
        email.to_string(),
    )
}

fn generate_backup_codes() -> Vec<String> {
    let mut rng = rand::thread_rng();
    (0..10)
        .map(|_| {
            let code: u32 = rng.gen_range(10000000..99999999);
            format!("{:08}", code)
        })
        .collect()
}

async fn store_backup_codes(pool: &PgPool, admin_user_id: uuid::Uuid, codes: &[String]) {
    // Delete existing backup codes
    let _ = sqlx::query("DELETE FROM admin_2fa_backup_codes WHERE admin_user_id = $1")
        .bind(admin_user_id)
        .execute(pool)
        .await;

    let argon2 = Argon2::default();
    for code in codes {
        let salt = SaltString::generate(&mut OsRng);
        if let Ok(hash) = argon2.hash_password(code.as_bytes(), &salt) {
            let _ = sqlx::query(
                "INSERT INTO admin_2fa_backup_codes (admin_user_id, code_hash) VALUES ($1, $2)",
            )
            .bind(admin_user_id)
            .bind(hash.to_string())
            .execute(pool)
            .await;
        }
    }
}

async fn try_use_backup_code(pool: &PgPool, admin_user_id: uuid::Uuid, code: &str) -> bool {
    let rows = match sqlx::query_as::<_, (uuid::Uuid, String)>(
        "SELECT id, code_hash FROM admin_2fa_backup_codes WHERE admin_user_id = $1 AND used_at IS NULL",
    )
    .bind(admin_user_id)
    .fetch_all(pool)
    .await
    {
        Ok(rows) => rows,
        Err(_) => return false,
    };

    for (id, hash_str) in rows {
        if let Ok(parsed_hash) = PasswordHash::new(&hash_str) {
            if Argon2::default()
                .verify_password(code.as_bytes(), &parsed_hash)
                .is_ok()
            {
                let _ = sqlx::query(
                    "UPDATE admin_2fa_backup_codes SET used_at = NOW() WHERE id = $1",
                )
                .bind(id)
                .execute(pool)
                .await;
                return true;
            }
        }
    }

    false
}
