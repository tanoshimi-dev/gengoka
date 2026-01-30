use actix_cors::Cors;
use actix_web::http::header;

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            header::ORIGIN,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::AUTHORIZATION,
            header::HeaderName::from_static("x-user-id"),
        ])
        .expose_headers(vec![header::CONTENT_LENGTH])
        .supports_credentials()
        .max_age(3600)
}
