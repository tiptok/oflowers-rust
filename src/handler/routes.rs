use crate::handler::other_handler::{
    file_server, responder_custom_responder, responder_either_responder, responder_impl_responder,
    responder_str, responder_stream_responder, responder_string,
};
use crate::handler::user_handler;

use actix_web::web;
use log::info;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    //info!("Configuring routes...");
    // cfg.service(
    //     web::scope("/v1/system")
    //         .service(web::resource("/login").route(web::post().to(user_handler::login)))
    //         .service(web::resource("/user_info").route(web::post().to(user_handler::user_info)))
    //         .service(web::resource("/user/search").route(web::post().to(user_handler::search)))
    //         .service(web::resource("/user").route(web::post().to(user_handler::save)))
    //         .service(web::resource("/user/{id}")
    //             .route(web::get().to(user_handler::get))
    //             .route(web::put().to(user_handler::update))
    //             .route(web::delete().to(user_handler::delete)))
    // );

    cfg.service(web::scope("/v1/system")
        .route("/login",web::post().to(user_handler::login))
        .route("/user_info",web::post().to(user_handler::user_info))

        .route("/user/search",web::post().to(user_handler::search))
        .route("/user",web::post().to(user_handler::save))
        .route("/user/{id}",web::get().to(user_handler::get))
        .route("/user/{id}",web::put().to(user_handler::update))
        .route("/user/{id}",web::delete().to(user_handler::delete))
    );

    // 测试路由
    cfg.service(
        web::scope("responder")
            .route("/str", web::get().to(responder_str))
            .route("/string", web::get().to(responder_string))
            .route("/impl_responder", web::get().to(responder_impl_responder))
            .route(
                "/custom_responder",
                web::get().to(responder_custom_responder),
            )
            .route("/stream", web::get().to(responder_stream_responder))
            .route("/either", web::get().to(responder_either_responder)),
    );
    // 静态文件
    // cfg.route("/static/{filename:.*}", web::get().to(file_server));
    // 目录  
    // curl http://127.0.0.1:8088/static
    cfg.service(actix_files::Files::new("/static", "./public").show_files_listing());
}
