use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::models::{
    Answer, AnswerWithDetails, Challenge, CreateUserRequest, MyPageResponse, PaginationParams,
    UpdateUserRequest, User, UserProfile, UserStats, UserSummary,
};
use crate::utils;

pub async fn get_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let current_user_id = utils::get_user_id(&req);

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return utils::not_found("User not found"),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return utils::internal_error("Failed to fetch user");
        }
    };

    // Get counts
    let answer_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE user_id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let follower_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE following_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let following_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE follower_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let is_following = if let Some(uid) = current_user_id {
        sqlx::query_scalar::<_, bool>(
            r#"SELECT EXISTS(SELECT 1 FROM follows WHERE follower_id = $1 AND following_id = $2)"#,
        )
        .bind(uid)
        .bind(user_id)
        .fetch_one(pool.get_ref())
        .await
        .unwrap_or(false)
    } else {
        false
    };

    utils::success(UserProfile {
        id: user.id,
        name: user.name,
        avatar: user.avatar,
        bio: user.bio,
        total_likes: user.total_likes,
        answer_count: answer_count.0,
        follower_count: follower_count.0,
        following_count: following_count.0,
        is_following,
    })
}

pub async fn get_user_answers(
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    query: web::Query<PaginationParams>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let current_user_id = utils::get_user_id(&req);
    let (page, page_size, offset) = utils::normalize_pagination(
        query.page,
        query.page_size,
        config.pagination.default_page_size,
        config.pagination.max_page_size,
    );

    let total: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE user_id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let answers = sqlx::query_as::<_, Answer>(
        r#"
        SELECT * FROM answers
        WHERE user_id = $1 AND status = 'active'
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool.get_ref())
    .await;

    let answers = match answers {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Failed to fetch user answers: {}", e);
            return utils::internal_error("Failed to fetch user answers");
        }
    };

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await;

    let user = match user {
        Ok(u) => u,
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return utils::internal_error("Failed to fetch user answers");
        }
    };

    let user_summary = UserSummary {
        id: user.id,
        name: user.name,
        avatar: user.avatar,
    };

    let mut results: Vec<AnswerWithDetails> = Vec::new();
    for answer in answers {
        let challenge = sqlx::query_as::<_, Challenge>(
            r#"SELECT * FROM challenges WHERE id = $1"#,
        )
        .bind(answer.challenge_id)
        .fetch_one(pool.get_ref())
        .await;

        let is_liked = if let Some(uid) = current_user_id {
            sqlx::query_scalar::<_, bool>(
                r#"SELECT EXISTS(SELECT 1 FROM likes WHERE answer_id = $1 AND user_id = $2)"#,
            )
            .bind(answer.id)
            .bind(uid)
            .fetch_one(pool.get_ref())
            .await
            .unwrap_or(false)
        } else {
            false
        };

        if let Ok(challenge) = challenge {
            results.push(AnswerWithDetails {
                answer,
                user: user_summary.clone(),
                challenge,
                is_liked,
            });
        }
    }

    utils::paginated(results, page, page_size, total.0)
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    body: web::Json<CreateUserRequest>,
) -> HttpResponse {
    let result = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, name, avatar, bio)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
    )
    .bind(&body.email)
    .bind(&body.name)
    .bind(&body.avatar)
    .bind(&body.bio)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => utils::created(user),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            if e.to_string().contains("duplicate") {
                utils::conflict("Email already exists")
            } else {
                utils::internal_error("Failed to create user")
            }
        }
    }
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let current_user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    if user_id != current_user_id {
        return utils::forbidden("You can only update your own profile");
    }

    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return utils::not_found("User not found"),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return utils::internal_error("Failed to update user");
        }
    };

    let name = body.name.as_ref().unwrap_or(&user.name);
    let avatar = body.avatar.as_ref().or(user.avatar.as_ref());
    let bio = body.bio.as_ref().or(user.bio.as_ref());

    let result = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET name = $1, avatar = $2, bio = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(avatar)
    .bind(bio)
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(user) => utils::success(user),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            utils::internal_error("Failed to update user")
        }
    }
}

pub async fn get_me(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // Fetch user
    let user = sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_optional(pool.get_ref())
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return utils::not_found("User not found"),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            return utils::internal_error("Failed to fetch user");
        }
    };

    // Social counts
    let answer_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM answers WHERE user_id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let follower_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE following_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let following_count: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(*) FROM follows WHERE follower_id = $1"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // Learning stats
    let total_challenges: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(DISTINCT challenge_id) FROM answers WHERE user_id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let completed_today: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(DISTINCT challenge_id) FROM answers WHERE user_id = $1 AND status = 'active' AND created_at::date = CURRENT_DATE"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    let average_score: (Option<f64>,) = sqlx::query_as(
        r#"SELECT AVG(score::float8) FROM answers WHERE user_id = $1 AND status = 'active' AND score IS NOT NULL"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((None,));

    // Streak calculation
    let dates: Vec<(chrono::NaiveDate,)> = sqlx::query_as(
        r#"SELECT DISTINCT created_at::date AS d FROM answers WHERE user_id = $1 AND status = 'active' ORDER BY d DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let today = chrono::Utc::now().date_naive();
    let mut current_streak: i64 = 0;
    let mut best_streak: i64 = 0;
    let mut streak: i64 = 0;
    let mut expected = today;

    for (i, (date,)) in dates.iter().enumerate() {
        if i == 0 {
            if *date == today || *date == today - chrono::Duration::days(1) {
                streak = 1;
                expected = *date - chrono::Duration::days(1);
            } else {
                streak = 1;
                expected = *date - chrono::Duration::days(1);
                current_streak = 0;
            }
        } else if *date == expected {
            streak += 1;
            expected = *date - chrono::Duration::days(1);
        } else {
            if i <= 1 || (dates[0].0 == today || dates[0].0 == today - chrono::Duration::days(1)) {
                if current_streak == 0 {
                    current_streak = streak;
                }
            }
            best_streak = best_streak.max(streak);
            streak = 1;
            expected = *date - chrono::Duration::days(1);
        }
    }

    if !dates.is_empty() {
        best_streak = best_streak.max(streak);
        if current_streak == 0 && (dates[0].0 == today || dates[0].0 == today - chrono::Duration::days(1)) {
            current_streak = streak;
        }
    }

    utils::success(MyPageResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        avatar: user.avatar,
        bio: user.bio,
        total_likes: user.total_likes,
        answer_count: answer_count.0,
        follower_count: follower_count.0,
        following_count: following_count.0,
        total_challenges: total_challenges.0,
        completed_today: completed_today.0,
        current_streak,
        best_streak,
        average_score: average_score.0.unwrap_or(0.0),
    })
}

pub async fn get_user_stats(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    let user_id = match utils::get_user_id(&req) {
        Some(id) => id,
        None => return utils::unauthorized("User ID required"),
    };

    // total_challenges: distinct challenges the user has answered
    let total_challenges: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(DISTINCT challenge_id) FROM answers WHERE user_id = $1 AND status = 'active'"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // completed_today: distinct challenges answered today
    let completed_today: (i64,) = sqlx::query_as(
        r#"SELECT COUNT(DISTINCT challenge_id) FROM answers WHERE user_id = $1 AND status = 'active' AND created_at::date = CURRENT_DATE"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((0,));

    // average_score
    let average_score: (Option<f64>,) = sqlx::query_as(
        r#"SELECT AVG(score::float8) FROM answers WHERE user_id = $1 AND status = 'active' AND score IS NOT NULL"#,
    )
    .bind(user_id)
    .fetch_one(pool.get_ref())
    .await
    .unwrap_or((None,));

    // Streak calculation: get distinct dates with answers, ordered descending
    let dates: Vec<(chrono::NaiveDate,)> = sqlx::query_as(
        r#"SELECT DISTINCT created_at::date AS d FROM answers WHERE user_id = $1 AND status = 'active' ORDER BY d DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    let today = chrono::Utc::now().date_naive();
    let mut current_streak: i64 = 0;
    let mut best_streak: i64 = 0;
    let mut streak: i64 = 0;
    let mut expected = today;

    for (i, (date,)) in dates.iter().enumerate() {
        if i == 0 {
            // Allow today or yesterday as the start
            if *date == today || *date == today - chrono::Duration::days(1) {
                streak = 1;
                expected = *date - chrono::Duration::days(1);
            } else {
                // No current streak, but still calculate best
                streak = 1;
                expected = *date - chrono::Duration::days(1);
                current_streak = 0;
            }
        } else if *date == expected {
            streak += 1;
            expected = *date - chrono::Duration::days(1);
        } else {
            if i <= 1 || (dates[0].0 == today || dates[0].0 == today - chrono::Duration::days(1)) {
                if current_streak == 0 {
                    current_streak = streak;
                }
            }
            best_streak = best_streak.max(streak);
            streak = 1;
            expected = *date - chrono::Duration::days(1);
        }
    }

    // Finalize streaks
    if !dates.is_empty() {
        best_streak = best_streak.max(streak);
        if current_streak == 0 && (dates[0].0 == today || dates[0].0 == today - chrono::Duration::days(1)) {
            current_streak = streak;
        }
    }

    utils::success(UserStats {
        total_challenges: total_challenges.0,
        completed_today: completed_today.0,
        current_streak,
        best_streak,
        average_score: average_score.0.unwrap_or(0.0),
    })
}
