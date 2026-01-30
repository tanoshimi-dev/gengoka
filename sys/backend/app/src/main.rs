mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod utils;

use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let pool = web::Data::new(pool);
    let config = web::Data::new(config);

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(config.clone())
            .wrap(middleware::cors())
            .wrap(tracing_actix_web::TracingLogger::default())
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
