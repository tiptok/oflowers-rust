use actix_files::NamedFile;
use actix_http::{ResponseBuilder, StatusCode};
use actix_web::{
    error, web, Either, Error, FromRequest, HttpRequest, HttpResponse, Responder, Result,
};
use futures::future::ok;
use futures::stream::once;
use std::path::PathBuf;
use log::debug;

// curl http://localhost:8088/responder/str
pub async fn responder_str() -> &'static str {
    "responder_str"
}

// curl http://localhost:8088/responder/string
pub async fn responder_string() -> String {
    "responder_string".to_string()
}

// curl http://localhost:8088/responder/impl_responder
pub async fn responder_impl_responder() -> impl Responder {
    web::Bytes::from_static(b"responder_string")
}

// curl http://localhost:8088/responder/custom_responder
pub async fn responder_custom_responder() -> impl Responder {
    crate::pkg::result::HttpResponseWrapper {
        code: 0,
        msg: "success".to_string(),
        data: Some("custom_responder".to_string()),
    }
}

// curl http://localhost:8088/responder/stream
pub async fn responder_stream_responder() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

// curl http://localhost:8088/responder/either
pub async fn responder_either_responder() -> RegisterResult {
    Either::Left(HttpResponse::BadRequest().body("Bad data"))
    //Either::Right(Ok("Hello!"))
}

// curl http://localhost:8088/static/a.txt
pub async fn file_server(req: HttpRequest) -> Result<NamedFile> {
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut path: PathBuf = PathBuf::from("./public").join(filename);
    debug!("file path is:{}",path.to_str().unwrap());
    Ok(NamedFile::open(path)?)
}
