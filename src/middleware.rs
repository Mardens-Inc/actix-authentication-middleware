use crate::user::User;
use actix_web::dev::{forward_ready, Service};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use futures::future::{ready, LocalBoxFuture, Ready};
use log::{debug, error, info, trace, warn};
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
        debug!("Processing authentication middleware request");
        Box::pin(async move {
            let user_agent = req.headers()
                                .get("User-Agent")
                                .and_then(|s| s.to_str().ok())
                                .unwrap_or("Mardens Actix Auth Library");
            let token = match req.headers().get("X-Authentication") {
                Some(auth_header) => {
                    trace!("Found X-Authentication header");
                    auth_header.to_str().ok().map(|s| {
                        trace!("Successfully parsed X-Authentication header");
                        s.to_string()
                    })
                }
                None => {
                    trace!("X-Authentication header not found, checking for token cookie");
                    let cookie_token = req.cookie("token").map(|c| {
                        trace!("Found token cookie");
                        c.value().to_string()
                    });
                    cookie_token
                }
            };

            match token {
                Some(token) => {
                    debug!("Authentication token found, attempting to authenticate");
                    if let Err(e) = User::authenticate_user_with_token(&token, &user_agent).await {
                        error!("Failed to authenticate user: {}", e.to_string());
                        Err(ErrorUnauthorized(e))
                    } else {
                        info!("User successfully authenticated, proceeding with request");
                        service.call(req).await
                    }
                }
                _ => {
                    warn!("Request rejected: Missing or invalid authentication token");
                    Err(ErrorUnauthorized("Missing or invalid authentication token"))
                }
            }
        })
    }
}
