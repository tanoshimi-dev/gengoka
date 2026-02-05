use actix_session::Session;
use actix_web::{web, HttpResponse};
use askama::Template;
use sqlx::PgPool;

use crate::middleware::admin_auth::get_admin_user_id;
use crate::models::admin::{AdminUser, AnswerStat, DashboardStats, UserStat};

#[derive(Template)]
#[template(path = "admin/dashboard.html")]
pub struct DashboardTemplate {
    pub admin: AdminUser,
    pub stats: DashboardStats,
}

pub async fn dashboard(pool: web::Data<PgPool>, session: Session) -> HttpResponse {
    let admin_id = match get_admin_user_id(&session) {
        Some(id) => id,
        None => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish()
        }
    };

    // Get admin user
    let admin = sqlx::query_as::<_, AdminUser>(
        r#"SELECT * FROM admin_users WHERE id = $1"#,
    )
    .bind(admin_id)
    .fetch_one(pool.get_ref())
    .await;

    let admin = match admin {
        Ok(a) => a,
        Err(_) => {
            return HttpResponse::Found()
                .insert_header(("Location", "/admin/login"))
                .finish()
        }
    };

    // Gather dashboard statistics
    let stats = get_dashboard_stats(&pool).await;

    let template = DashboardTemplate { admin, stats };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap_or_default())
}

async fn get_dashboard_stats(pool: &PgPool) -> DashboardStats {
    let mut stats = DashboardStats::default();

    // Total counts
    if let Ok((count,)) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await
    {
        stats.total_users = count;
    }

    if let Ok((count,)) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM categories WHERE status = 'active'")
            .fetch_one(pool)
            .await
    {
        stats.total_categories = count;
    }

    if let Ok((count,)) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM challenges WHERE status = 'active'")
            .fetch_one(pool)
            .await
    {
        stats.total_challenges = count;
    }

    if let Ok((count,)) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM answers WHERE status = 'active'")
            .fetch_one(pool)
            .await
    {
        stats.total_answers = count;
    }

    if let Ok((count,)) =
        sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM comments WHERE status = 'active'")
            .fetch_one(pool)
            .await
    {
        stats.total_comments = count;
    }

    // Today's counts
    if let Ok((count,)) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM users WHERE created_at >= CURRENT_DATE",
    )
    .fetch_one(pool)
    .await
    {
        stats.users_today = count;
    }

    if let Ok((count,)) = sqlx::query_as::<_, (i64,)>(
        "SELECT COUNT(*) FROM answers WHERE created_at >= CURRENT_DATE",
    )
    .fetch_one(pool)
    .await
    {
        stats.answers_today = count;
    }

    // Recent users
    if let Ok(users) = sqlx::query_as::<_, (uuid::Uuid, String, Option<String>, String, chrono::DateTime<chrono::Utc>)>(
        r#"
        SELECT id, name, email, status, created_at
        FROM users
        ORDER BY created_at DESC
        LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await
    {
        stats.recent_users = users
            .into_iter()
            .map(|(id, name, email, status, created_at)| UserStat {
                id,
                name,
                email,
                status,
                created_at,
            })
            .collect();
    }

    // Recent answers
    if let Ok(answers) = sqlx::query_as::<_, (uuid::Uuid, String, String, String, chrono::DateTime<chrono::Utc>)>(
        r#"
        SELECT a.id, a.content, u.name as user_name, c.title as challenge_title, a.created_at
        FROM answers a
        JOIN users u ON a.user_id = u.id
        JOIN challenges c ON a.challenge_id = c.id
        ORDER BY a.created_at DESC
        LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await
    {
        stats.recent_answers = answers
            .into_iter()
            .map(|(id, content, user_name, challenge_title, created_at)| AnswerStat {
                id,
                content,
                user_name,
                challenge_title,
                created_at,
            })
            .collect();
    }

    stats
}
