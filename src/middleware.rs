use crate::user::User;
use actix_web::dev::{forward_ready, Service};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use futures::future::{ready, LocalBoxFuture, Ready};
use std::rc::Rc;

pub struct Authentication;

impl Authentication {
    pub fn new() -> Self {
        Authentication
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            let token = match req.headers().get("X-Authentication") {
                Some(auth_header) => auth_header.to_str().ok().map(|s| s.to_string()),
                None => req.cookie("token").map(|c| c.value().to_string()),
            };

            match token {
                Some(token) if User::authenticate_user_with_token(&token).await.is_ok() => {
                    service.call(req).await
                }
                _ => Err(ErrorUnauthorized("Missing or invalid authentication token")),
            }
        })
    }
}
