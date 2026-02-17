use actix_web::{dev::ServiceResponse, test, web, App};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

use gengoka_backend::config::{
    AdminConfig, AuthConfig, ChallengeGenerationConfig, Config, DatabaseConfig, GeminiConfig,
    PaginationConfig, ServerConfig, SocialAuthConfig,
};
use gengoka_backend::db::run_migrations;
use gengoka_backend::routes;

/// Build a Config suitable for integration tests.
/// Uses `gengoka_test` database on the Docker postgres host.
pub fn test_config() -> Config {
    let db_host = std::env::var("TEST_DB_HOST").unwrap_or_else(|_| "gengoka-db".to_string());
    let db_port: u16 = std::env::var("TEST_DB_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .unwrap_or(5432);

    Config {
        server: ServerConfig {
            port: 8080,
            host: "0.0.0.0".to_string(),
        },
        database: DatabaseConfig {
            host: db_host,
            port: db_port,
            user: "gengoka".to_string(),
            password: "gengoka_password".to_string(),
            database: "gengoka_test".to_string(),
            max_connections: 5,
        },
        pagination: PaginationConfig {
            default_page_size: 20,
            max_page_size: 100,
        },
        gemini: GeminiConfig {
            api_key: String::new(),
            model: "gemini-1.5-flash".to_string(),
            base_url: "https://generativelanguage.googleapis.com".to_string(),
            max_tokens: 2048,
            temperature: 0.7,
        },
        challenge_generation: ChallengeGenerationConfig {
            enabled: false,
            daily_count: 5,
            cron_schedule: "0 0 9 * * *".to_string(),
        },
        auth: AuthConfig {
            jwt_secret: "test-jwt-secret-for-integration-tests".to_string(),
            access_token_ttl_minutes: 30,
            refresh_token_ttl_days: 90,
        },
        social_auth: SocialAuthConfig {
            google_client_id_web: String::new(),
            google_client_id_ios: String::new(),
            google_client_id_android: String::new(),
            apple_client_id: "app.gengoka".to_string(),
            apple_team_id: String::new(),
            line_channel_id: String::new(),
            line_channel_secret: String::new(),
            google_jwks_url: "https://www.googleapis.com/oauth2/v3/certs".to_string(),
            apple_jwks_url: "https://appleid.apple.com/auth/keys".to_string(),
            line_verify_url: "https://api.line.me/oauth2/v2.1/verify".to_string(),
            line_profile_url: "https://api.line.me/v2/profile".to_string(),
        },
        admin: AdminConfig {
            session_secret: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string(),
            session_ttl_hours: 24,
        },
    }
}

/// Connect to `gengoka_test`, run migrations, and return the pool.
pub async fn setup_test_db() -> PgPool {
    let config = test_config();
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to test database");

    run_migrations(&pool).await;
    pool
}

/// TRUNCATE all application tables (respecting FK order via CASCADE).
pub async fn clean_db(pool: &PgPool) {
    sqlx::query(
        r#"
        TRUNCATE TABLE
            refresh_tokens,
            user_social_accounts,
            admin_2fa_backup_codes,
            admin_audit_logs,
            system_config,
            likes,
            comments,
            answers,
            follows,
            challenges,
            categories,
            admin_users,
            users
        CASCADE
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to truncate tables");
}

/// Build an `actix_web::test` service with the full route tree, pool, and config.
pub async fn create_test_app(
    pool: &PgPool,
) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = ServiceResponse,
    Error = actix_web::Error,
> {
    let config = test_config();

    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config))
            .configure(routes::configure),
    )
    .await
}

/// Register a test user via POST /api/v1/auth/register.
/// Returns (status_code, json_body).
pub async fn register_test_user(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
    email: &str,
    password: &str,
    name: &str,
) -> (u16, Value) {
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "password": password,
            "name": name,
            "device_info": "integration-test"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(app, req).await;
    let status = resp.status().as_u16();
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, json)
}

/// Login a test user via POST /api/v1/auth/login.
/// Returns (status_code, json_body).
pub async fn login_test_user(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
    email: &str,
    password: &str,
) -> (u16, Value) {
    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(serde_json::json!({
            "email": email,
            "password": password,
            "device_info": "integration-test"
        }))
        .to_request();

    let resp: ServiceResponse = test::call_service(app, req).await;
    let status = resp.status().as_u16();
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, json)
}

/// Register a user and return (user_id, access_token).
pub async fn register_and_login(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
    email: &str,
    password: &str,
    name: &str,
) -> (Uuid, String) {
    let (status, body) = register_test_user(app, email, password, name).await;
    assert_eq!(status, 201, "register_and_login failed: {:?}", body);
    let user_id = Uuid::parse_str(body["data"]["user"]["id"].as_str().unwrap()).unwrap();
    let token = body["data"]["access_token"].as_str().unwrap().to_string();
    (user_id, token)
}

/// Insert a category directly into the DB. Returns the category id.
pub async fn create_test_category(pool: &PgPool, name: &str) -> Uuid {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO categories (name, description, icon, color, char_limit, sort_order, status)
        VALUES ($1, 'Test category', 'icon', '#FF0000', 200, 0, 'active')
        RETURNING id
        "#,
    )
    .bind(name)
    .fetch_one(pool)
    .await
    .expect("Failed to create test category");
    row.0
}

/// Insert a challenge directly into the DB. Returns the challenge id.
pub async fn create_test_challenge(pool: &PgPool, category_id: Uuid, title: &str) -> Uuid {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO challenges (category_id, title, description, char_limit, release_date, status)
        VALUES ($1, $2, 'Test challenge', 200, CURRENT_DATE, 'active')
        RETURNING id
        "#,
    )
    .bind(category_id)
    .bind(title)
    .fetch_one(pool)
    .await
    .expect("Failed to create test challenge");
    row.0
}

/// Create an answer via the API. Returns (status_code, json_body).
pub async fn create_test_answer(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = ServiceResponse,
        Error = actix_web::Error,
    >,
    challenge_id: Uuid,
    access_token: &str,
    content: &str,
) -> (u16, Value) {
    let req = test::TestRequest::post()
        .uri(&format!("/api/v1/challenges/{}/answers", challenge_id))
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .set_json(serde_json::json!({ "content": content }))
        .to_request();

    let resp: ServiceResponse = test::call_service(app, req).await;
    let status = resp.status().as_u16();
    let body = test::read_body(resp).await;
    let json: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
    (status, json)
}

/// Insert a social account link directly into the DB.
pub async fn insert_social_account(
    pool: &PgPool,
    user_id: Uuid,
    provider: &str,
    provider_user_id: &str,
) {
    sqlx::query(
        r#"
        INSERT INTO user_social_accounts (user_id, provider, provider_user_id, provider_email, provider_name)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(user_id)
    .bind(provider)
    .bind(provider_user_id)
    .bind(format!("{}@example.com", provider))
    .bind(format!("{} User", provider))
    .execute(pool)
    .await
    .expect("Failed to insert social account");
}
