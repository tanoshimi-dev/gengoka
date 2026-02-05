use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub pagination: PaginationConfig,
    pub gemini: GeminiConfig,
    pub challenge_generation: ChallengeGenerationConfig,
    pub admin: AdminConfig,
}

#[derive(Debug, Clone)]
pub struct AdminConfig {
    pub session_secret: String,
    pub session_ttl_hours: u64,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct PaginationConfig {
    pub default_page_size: i64,
    pub max_page_size: i64,
}

#[derive(Debug, Clone)]
pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
    pub max_tokens: i32,
    pub temperature: f32,
}

#[derive(Debug, Clone)]
pub struct ChallengeGenerationConfig {
    pub enabled: bool,
    pub daily_count: u32,
    pub cron_schedule: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        Self {
            server: ServerConfig {
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()
                    .unwrap_or(8080),
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            },
            database: DatabaseConfig {
                host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                port: env::var("DB_PORT")
                    .unwrap_or_else(|_| "5432".to_string())
                    .parse()
                    .unwrap_or(5432),
                user: env::var("DB_USER").unwrap_or_else(|_| "gengoka".to_string()),
                password: env::var("DB_PASSWORD").unwrap_or_else(|_| "gengoka_password".to_string()),
                database: env::var("DB_NAME").unwrap_or_else(|_| "gengoka_db".to_string()),
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .unwrap_or(10),
            },
            pagination: PaginationConfig {
                default_page_size: env::var("DEFAULT_PAGE_SIZE")
                    .unwrap_or_else(|_| "20".to_string())
                    .parse()
                    .unwrap_or(20),
                max_page_size: env::var("MAX_PAGE_SIZE")
                    .unwrap_or_else(|_| "100".to_string())
                    .parse()
                    .unwrap_or(100),
            },
            gemini: GeminiConfig {
                api_key: env::var("GEMINI_API_KEY").unwrap_or_default(),
                model: env::var("GEMINI_MODEL")
                    .unwrap_or_else(|_| "gemini-1.5-flash".to_string()),
                base_url: env::var("GEMINI_BASE_URL")
                    .unwrap_or_else(|_| "https://generativelanguage.googleapis.com".to_string()),
                max_tokens: env::var("GEMINI_MAX_TOKENS")
                    .unwrap_or_else(|_| "2048".to_string())
                    .parse()
                    .unwrap_or(2048),
                temperature: env::var("GEMINI_TEMPERATURE")
                    .unwrap_or_else(|_| "0.7".to_string())
                    .parse()
                    .unwrap_or(0.7),
            },
            challenge_generation: ChallengeGenerationConfig {
                enabled: env::var("CHALLENGE_GENERATION_ENABLED")
                    .unwrap_or_else(|_| "false".to_string())
                    .parse()
                    .unwrap_or(false),
                daily_count: env::var("CHALLENGE_GENERATION_COUNT")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .unwrap_or(5),
                cron_schedule: env::var("CHALLENGE_GENERATION_CRON")
                    .unwrap_or_else(|_| "0 0 9 * * *".to_string()),
            },
            admin: AdminConfig {
                session_secret: env::var("ADMIN_SESSION_SECRET")
                    .unwrap_or_else(|_| {
                        // Default secret for development - MUST be set in production!
                        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_string()
                    }),
                session_ttl_hours: env::var("ADMIN_SESSION_TTL_HOURS")
                    .unwrap_or_else(|_| "24".to_string())
                    .parse()
                    .unwrap_or(24),
            },
        }
    }
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}
