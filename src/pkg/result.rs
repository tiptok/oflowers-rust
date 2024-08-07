use actix_web::{error, http::StatusCode, HttpRequest, HttpResponse, Responder};
use actix_web::http::header::ContentType;
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
use serde_derive::Deserialize;
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
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ResponseBody::new(&self.to_string(), String::from("")))
    }
}

pub fn success<T>(data: T) -> HttpResponseWrapper<T> {
    HttpResponseWrapper {
        code: 0,
        msg: "OK".to_string(),
        data: Some(data),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub msg: String,
    pub data: T,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: T) -> ResponseBody<T> {
        ResponseBody {
            msg: message.to_string(),
            data,
        }
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
