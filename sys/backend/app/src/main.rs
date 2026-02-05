mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use std::sync::Arc;

use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use sqlx::Executor;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use services::scheduler::ChallengeScheduler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Check for CLI commands
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "create-admin" {
        return create_admin_user().await;
    }
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env();
    info!("Starting server on port {}", config.server.port);

    // Initialize database
    let pool = db::init_pool(&config.database).await;
    info!("Database connected");

    // Run migrations
    db::run_migrations(&pool).await;
    info!("Migrations completed");

    // Initialize scheduler if challenge generation is enabled
    let scheduler = if config.challenge_generation.enabled {
        if config.gemini.api_key.is_empty() {
            warn!("Challenge generation is enabled but GEMINI_API_KEY is not set");
            None
        } else {
            let pool_arc = Arc::new(pool.clone());
            let config_arc = Arc::new(config.clone());
            match ChallengeScheduler::new(pool_arc, config_arc).await {
                Ok(scheduler) => {
                    if let Err(e) = scheduler.start().await {
                        warn!("Failed to start challenge scheduler: {}", e);
                        None
                    } else {
                        info!("Challenge generation scheduler started with cron: {}", config.challenge_generation.cron_schedule);
                        Some(scheduler)
                    }
                }
                Err(e) => {
                    warn!("Failed to create challenge scheduler: {}", e);
                    None
                }
            }
        }
    } else {
        info!("Challenge generation is disabled");
        None
    };

    let pool = web::Data::new(pool);

    // Create session key from config secret (must be at least 64 bytes when hex-decoded)
    let key_bytes = hex::decode(&config.admin.session_secret)
        .unwrap_or_else(|_| {
            // If not valid hex, use the raw bytes padded to 64
            let mut key = vec![0u8; 64];
            let src = config.admin.session_secret.as_bytes();
            let len = src.len().min(64);
            key[..len].copy_from_slice(&src[..len]);
            key
        });
    let session_key = Key::from(&key_bytes);

    let config = web::Data::new(config);

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(config.clone())
            .wrap(middleware::cors())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), session_key.clone())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .build(),
            )
            .configure(routes::configure)
            .configure(routes::admin::configure)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    // Shutdown scheduler gracefully
    if let Some(mut scheduler) = scheduler {
        if let Err(e) = scheduler.shutdown().await {
            warn!("Error shutting down scheduler: {}", e);
        }
    }

    Ok(())
}

async fn create_admin_user() -> std::io::Result<()> {
    use std::io::{self, Write};

    println!("=== Create Admin User ===\n");

    // Get email
    print!("Email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim().to_string();

    // Get name
    print!("Name: ");
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    // Get password
    print!("Password: ");
    io::stdout().flush()?;
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;
    let password = password.trim().to_string();

    if email.is_empty() || name.is_empty() || password.is_empty() {
        eprintln!("Error: All fields are required");
        return Ok(());
    }

    // Hash password
    let password_hash = match handlers::admin::auth::hash_password(&password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return Ok(());
        }
    };

    // Initialize database
    let config = config::Config::from_env();
    let pool = db::init_pool(&config.database).await;

    // Run migrations first
    db::run_migrations(&pool).await;

    // Insert admin user
    let result = sqlx::query(
        r#"
        INSERT INTO admin_users (id, email, password_hash, name, role, status, created_at, updated_at)
        VALUES (gen_random_uuid(), $1, $2, $3, 'super_admin', 'active', NOW(), NOW())
        "#
    )
    .bind(&email)
    .bind(&password_hash)
    .bind(&name)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            println!("\nAdmin user created successfully!");
            println!("Email: {}", email);
            println!("Name: {}", name);
        }
        Err(e) => {
            eprintln!("\nError creating admin user: {}", e);
            if e.to_string().contains("duplicate") {
                eprintln!("An admin with this email already exists.");
            }
        }
    }

    Ok(())
}
