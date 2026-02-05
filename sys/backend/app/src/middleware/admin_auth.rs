use actix_session::SessionExt;
use actix_web::{
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::LOCATION,
    Error, HttpResponse,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::task::{Context, Poll};

pub struct AdminAuth;

impl<S, B> Transform<S, ServiceRequest> for AdminAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AdminAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminAuthMiddleware { service })
    }
}

pub struct AdminAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        let is_authenticated = session
            .get::<String>("admin_user_id")
            .unwrap_or(None)
            .is_some();

        if is_authenticated {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res.map_into_left_body())
            })
        } else {
            Box::pin(async move {
                let response = HttpResponse::Found()
                    .insert_header((LOCATION, "/admin/login"))
                    .finish()
                    .map_into_right_body();
                Ok(req.into_response(response))
            })
        }
    }
}

pub fn get_admin_user_id(session: &actix_session::Session) -> Option<uuid::Uuid> {
    session
        .get::<String>("admin_user_id")
        .ok()
        .flatten()
        .and_then(|s| uuid::Uuid::parse_str(&s).ok())
}

pub fn set_admin_session(session: &actix_session::Session, admin_user_id: uuid::Uuid) {
    let _ = session.insert("admin_user_id", admin_user_id.to_string());
}

pub fn clear_admin_session(session: &actix_session::Session) {
    session.purge();
}
