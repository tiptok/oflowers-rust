use std::default;

use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse, Responder};
//use futures::future::{ready, Ready};
use serde::Serialize;
#[derive(Serialize)]
pub struct HttpResponseWrapper<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> Responder for HttpResponseWrapper<T>
where
    T: Serialize,
{
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

use derive_more::{Display, Error};

use super::error::InternalError;

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[display(fmt = "{error_message}")]
    Unauthorized { error_message: String },

    #[display(fmt = "{error_message}")]
    InternalServerError { error_message: String },

    #[display(fmt = "{error_message}")]
    BadRequest { error_message: String },

    #[display(fmt = "{error_message}")]
    NotFound { error_message: String },
}

impl error::ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ServiceError::NotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}

pub fn success<T>(data: T) -> HttpResponseWrapper<T> {
    HttpResponseWrapper {
        code: 0,
        msg: "OK".to_string(),
        data: Some(data),
    }
}

struct EmptyStruct;

pub fn fail<T>(e: InternalError, v: Option<T>) -> HttpResponseWrapper<T> {
    HttpResponseWrapper {
        code: e.code as i32,
        msg: e.err.clone(),
        data: v,
    }
}
