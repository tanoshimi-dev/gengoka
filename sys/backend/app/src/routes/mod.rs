use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Categories
            .route("/categories", web::get().to(handlers::list_categories))
            .route("/categories/{id}", web::get().to(handlers::get_category))
            .route(
                "/categories/{id}/challenges",
                web::get().to(handlers::get_challenges_by_category),
            )
            // Challenges
            .route("/challenges", web::get().to(handlers::list_challenges))
            .route("/challenges", web::post().to(handlers::create_challenge))
            .route("/challenges/daily", web::get().to(handlers::get_daily_challenges))
            .route("/challenges/{id}", web::get().to(handlers::get_challenge))
            .route(
                "/challenges/{id}/answers",
                web::get().to(handlers::get_challenge_answers),
            )
            .route(
                "/challenges/{id}/answers",
                web::post().to(handlers::create_answer),
            )
            // Answers
            .route("/answers/{id}", web::get().to(handlers::get_answer))
            .route("/answers/{id}", web::put().to(handlers::update_answer))
            .route("/answers/{id}", web::delete().to(handlers::delete_answer))
            .route("/answers/{id}/like", web::post().to(handlers::like_answer))
            .route("/answers/{id}/like", web::delete().to(handlers::unlike_answer))
            .route(
                "/answers/{id}/comments",
                web::get().to(handlers::get_answer_comments),
            )
            .route(
                "/answers/{id}/comments",
                web::post().to(handlers::create_comment),
            )
            // Comments
            .route("/comments/{id}", web::delete().to(handlers::delete_comment))
            // Users
            .route("/users", web::post().to(handlers::create_user))
            .route("/users/{id}", web::get().to(handlers::get_user))
            .route("/users/{id}", web::put().to(handlers::update_user))
            .route("/users/{id}/answers", web::get().to(handlers::get_user_answers))
            .route("/users/{id}/follow", web::post().to(handlers::follow_user))
            .route("/users/{id}/follow", web::delete().to(handlers::unfollow_user))
            .route("/users/{id}/followers", web::get().to(handlers::get_followers))
            .route("/users/{id}/following", web::get().to(handlers::get_following))
            // Feed & Rankings
            .route("/feed", web::get().to(handlers::get_feed))
            .route("/trending", web::get().to(handlers::get_trending))
            .route("/rankings/daily", web::get().to(handlers::get_daily_ranking))
            .route("/rankings/weekly", web::get().to(handlers::get_weekly_ranking))
            .route("/rankings/all-time", web::get().to(handlers::get_alltime_ranking)),
    )
    // Health check (outside /api/v1)
    .route("/health", web::get().to(handlers::health_check));
}
