use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

use crate::config::DatabaseConfig;

pub type DbPool = PgPool;

pub async fn init_pool(config: &DatabaseConfig) -> DbPool {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.connection_string())
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &DbPool) {
    info!("Running database migrations...");

    // Users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE,
            name VARCHAR(100) NOT NULL,
            avatar VARCHAR(500),
            bio TEXT,
            total_likes INTEGER NOT NULL DEFAULT 0,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");

    // Categories table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(100) NOT NULL,
            description TEXT,
            icon VARCHAR(50),
            color VARCHAR(50),
            char_limit INTEGER NOT NULL DEFAULT 30,
            sort_order INTEGER NOT NULL DEFAULT 0,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create categories table");

    // Challenges table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS challenges (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            category_id UUID NOT NULL REFERENCES categories(id),
            title TEXT NOT NULL,
            description TEXT,
            char_limit INTEGER NOT NULL DEFAULT 30,
            release_date DATE,
            answer_count INTEGER NOT NULL DEFAULT 0,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create challenges table");

    // Answers table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS answers (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            challenge_id UUID NOT NULL REFERENCES challenges(id),
            user_id UUID NOT NULL REFERENCES users(id),
            content VARCHAR(200) NOT NULL,
            score INTEGER,
            ai_feedback JSONB,
            like_count INTEGER NOT NULL DEFAULT 0,
            comment_count INTEGER NOT NULL DEFAULT 0,
            view_count INTEGER NOT NULL DEFAULT 0,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create answers table");

    // Comments table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS comments (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            answer_id UUID NOT NULL REFERENCES answers(id) ON DELETE CASCADE,
            user_id UUID NOT NULL REFERENCES users(id),
            content TEXT NOT NULL,
            status VARCHAR(20) NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create comments table");

    // Likes table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS likes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            answer_id UUID NOT NULL REFERENCES answers(id) ON DELETE CASCADE,
            user_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(answer_id, user_id)
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create likes table");

    // Follows table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS follows (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            follower_id UUID NOT NULL REFERENCES users(id),
            following_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            UNIQUE(follower_id, following_id)
        )
        "#,
    )
    .execute(pool)
    .await
    .expect("Failed to create follows table");

    // Indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_challenges_category ON challenges(category_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_challenges_release_date ON challenges(release_date)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_answers_challenge ON answers(challenge_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_answers_user ON answers(user_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_answers_created ON answers(created_at DESC)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_comments_answer ON comments(answer_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_likes_answer ON likes(answer_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_follows_follower ON follows(follower_id)")
        .execute(pool)
        .await
        .ok();

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_follows_following ON follows(following_id)")
        .execute(pool)
        .await
        .ok();

    info!("Migrations completed successfully");
}
