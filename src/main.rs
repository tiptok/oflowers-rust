//#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use actix_service::Service;
use actix_web::{web, HttpResponse};
use actix_web::{App, HttpServer, Responder};
use futures::FutureExt;
use log::debug;
use std::{env, io};

mod domain;
mod handler;
mod logic;
mod middleware;
mod pkg;
mod schema;
mod svc;
mod stdlib;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    //dotenv::dotenv().expect("Failed to read .env file");
    //env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_LOG", "debug");
    env::set_var("PQ_LIB_DIR", "C:\\libs");
    env_logger::init();

    let config = svc::config::Config::default();
    let app_url = format!("{}:{}", &config.listen_address, &config.listen_port);
    debug!("app listen at:{}", app_url);

    let service = svc::context::init_service(config);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::auth::Authentication)
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .wrap(middleware::cors::cors())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(service.clone()))
            .configure(handler::routes::config_services)
            .route("/", web::get().to(hello))
    })
    .workers(2) // 默认情况下，HttpServer 以多线程方式 启动 Server，线程为等于当先系统的核心数,指定线程数。
    .bind(&app_url)?
    .run()
    .await
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello rust!")
}
