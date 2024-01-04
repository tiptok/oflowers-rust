use crate::pkg::jwt::UserToken;
use crate::svc::context::ServiceContext;
use actix_http::body::EitherBody;
use actix_http::{
    header::{HeaderName, HeaderValue},
    Method,
};
use actix_service::forward_ready;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, LocalBoxFuture, Ready};

pub struct Authentication;

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub const IGNORE_ROUTES: [&str; 6] = [
    "/v1/system/login",
    "/api/ping",
    "/api/auth/signup",
    "/api/auth/login",
    "/responder",
    "/static",
];

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticate_pass: bool = false;
        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        };
        if !authenticate_pass {
            for ignore_route in IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
        };
        if !authenticate_pass {
            if let Some(svc) = req.app_data::<Data<ServiceContext>>() {
                if let Some(token) = req.headers().get(crate::pkg::jwt::AUTHORIZATION) {
                    if let Ok(token_str) = token.to_str() {
                        if token_str.starts_with("bearer") || token_str.starts_with("Bearer") {
                            let token = token_str[6..token_str.len()].trim();
                            if let Ok(token_data) = UserToken::decode(
                                token.to_string(),
                                svc.config.jwt_secret.to_string(),
                            ) {
                                // Bypass some account routes
                                let mut headers = req.headers().clone();
                                headers.append(
                                    HeaderName::from_static("user_id"),
                                    HeaderValue::from(token_data.claims.user_id),
                                );
                                authenticate_pass = true;
                            }
                        }
                    }
                }
            }
        };
        if !authenticate_pass {
            let (request, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(crate::pkg::result::fail(
                    crate::pkg::error::new("token invalid.".to_string()),
                    Some(()),
                ))
                .map_into_right_body();

            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        };

        let res = self.service.call(req);
        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}
