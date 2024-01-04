use actix_web::http::StatusCode;
use actix_web::ResponseError;

use derive_more::Display;
use std::{default, fmt};

pub fn new(err: String) -> InternalError {
    InternalError {
        code: Code::InternalServerError,
        err: err,
    }
}

pub fn new_code_err(code: Code, err: String) -> InternalError {
    InternalError {
        code: code,
        err: err,
    }
}

// pub struct CodeError<T> {
//     pub code: i32,
//     pub err: Option<T>,
// }

// pub fn new<T>(err: T) -> CodeError<T> {
//     let mut code = 0;
//     if code == 0 {
//         code = Code::InternalServerError as i32
//     }
//     new_code_err(code, err)
// }

// pub fn new_code_err<T>(code: i32, err: T) -> CodeError<T> {
//     CodeError {
//         code: code,
//         err: Some(err),
//     }
// }

#[derive(Debug, Display)]
pub enum Code {
    InternalServerError = 1,
    Unauthorized = 1010,
    BadRequest = 1020,
    NotFound = 1030,
    DBError = 1040,
}
#[derive(Debug, Display)]
#[display(fmt = "Internal Error code:{} error:{}", code, err)]
pub struct InternalError {
    pub code: Code,
    pub err: String,
}

impl InternalError {
    pub fn new(msg: String) -> Self {
        InternalError {
            code: Code::InternalServerError,
            err: msg,
        }
    }
}
impl ResponseError for InternalError {
    fn status_code(&self) -> StatusCode {
        match &self.code {
            Code::Unauthorized => StatusCode::UNAUTHORIZED,
            Code::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Code::BadRequest => StatusCode::BAD_REQUEST,
            Code::NotFound => StatusCode::NOT_FOUND,
            Code::DBError => StatusCode::BAD_REQUEST,
            _ => StatusCode::OK,
        }
    }
}
