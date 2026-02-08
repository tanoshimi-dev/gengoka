use actix_web::web;

use crate::handlers::admin;
use crate::middleware::admin_auth::AdminAuth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            // Public routes (no auth required)
            .route("/login", web::get().to(admin::login_page))
            .route("/login", web::post().to(admin::login_submit))
            // Protected routes (auth required)
            .service(
                web::scope("")
                    .wrap(AdminAuth)
                    // Dashboard
                    .route("", web::get().to(admin::dashboard))
                    .route("/", web::get().to(admin::dashboard))
                    .route("/logout", web::get().to(admin::logout))
                    // Categories
                    .route("/categories", web::get().to(admin::list_categories))
                    .route("/categories/new", web::get().to(admin::new_category_form))
                    .route("/categories", web::post().to(admin::create_category))
                    .route("/categories/{id}", web::get().to(admin::get_category))
                    .route("/categories/{id}/edit", web::get().to(admin::edit_category_form))
                    .route("/categories/{id}", web::post().to(admin::update_category))
                    .route("/categories/{id}/delete", web::post().to(admin::delete_category))
                    // Challenges
                    .route("/challenges", web::get().to(admin::list_challenges))
                    .route("/challenges/new", web::get().to(admin::new_challenge_form))
                    .route("/challenges/bulk", web::get().to(admin::bulk_challenge_form))
                    .route("/challenges/bulk-generate", web::post().to(admin::bulk_generate_challenges))
                    .route("/challenges/bulk-save", web::post().to(admin::bulk_save_challenges))
                    .route("/challenges", web::post().to(admin::create_challenge))
                    .route("/challenges/{id}", web::get().to(admin::get_challenge))
                    .route("/challenges/{id}/edit", web::get().to(admin::edit_challenge_form))
                    .route("/challenges/{id}", web::post().to(admin::update_challenge))
                    .route("/challenges/{id}/delete", web::post().to(admin::delete_challenge))
                    // Users
                    .route("/users", web::get().to(admin::list_users))
                    .route("/users/{id}", web::get().to(admin::get_user))
                    .route("/users/{id}/suspend", web::get().to(admin::suspend_user_form))
                    .route("/users/{id}/suspend", web::post().to(admin::suspend_user))
                    .route("/users/{id}/unsuspend", web::post().to(admin::unsuspend_user))
                    // Answers
                    .route("/answers", web::get().to(admin::list_answers))
                    .route("/answers/{id}", web::get().to(admin::get_answer))
                    .route("/answers/{id}/moderate", web::get().to(admin::moderate_answer_form))
                    .route("/answers/{id}/moderate", web::post().to(admin::moderate_answer))
                    .route("/answers/{id}/unmoderate", web::post().to(admin::unmoderate_answer))
                    .route("/answers/{id}/delete", web::post().to(admin::delete_answer))
                    // Comments
                    .route("/comments", web::get().to(admin::list_comments))
                    .route("/comments/{id}", web::get().to(admin::get_comment))
                    .route("/comments/{id}/moderate", web::get().to(admin::moderate_comment_form))
                    .route("/comments/{id}/moderate", web::post().to(admin::moderate_comment))
                    .route("/comments/{id}/unmoderate", web::post().to(admin::unmoderate_comment))
                    .route("/comments/{id}/delete", web::post().to(admin::delete_comment))
                    // Settings
                    .route("/settings/gemini", web::get().to(admin::gemini_settings_page))
                    .route("/settings/gemini", web::post().to(admin::update_gemini_settings))
                    .route("/settings/scheduler", web::get().to(admin::scheduler_settings_page))
                    .route("/settings/scheduler", web::post().to(admin::update_scheduler_settings)),
            ),
    );
}
