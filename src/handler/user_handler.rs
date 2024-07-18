use actix_web::{error, web, Error, FromRequest, HttpRequest, HttpResponse, Responder};
use crate::domain::user::dto::*;
use crate::logic::user_logic;
use crate::pkg::{
    jwt,
    result::{fail, success},
};
use crate::svc::context::ServiceContext;

// login 登录
pub async fn login(req: web::Json<LoginRequest>, svc: web::Data<ServiceContext>) -> impl Responder {
    match user_logic::login(req.0, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}

// pub async fn login1(
//     req: HttpRequest,
//     payload: web::Payload,
//     svc: web::Data<ServiceContext>,
// ) -> Result<HttpResponse, Error> {
//     let login_request = json::unmarshal_from_payload::<LoginRequest>(payload).await;
//     match login_request {
//         Err(e) => Err(error::ErrorBadRequest(e.err)),
//         Ok(val) => match user_logic::login(val, &svc) {
//             Ok(response) => Ok(HttpResponse::Ok().json(response)),
//             Err(e) => Err(error::ErrorBadRequest(e.err)),
//         },
//     }
// }

// user_info 获取用户信息
pub async fn user_info(req: HttpRequest, svc: web::Data<ServiceContext>) -> impl Responder {
    let user_token = jwt::UserToken::decode_from_request(req, svc.config.jwt_secret.to_string());
    match user_logic::get_user_info(
        UserInfoRequest {
            id: user_token.unwrap().claims.user_id,
        },
        &svc,
    ) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}

// bind_wechat 绑定微信
pub async fn bind_wechat(
    req: HttpRequest,
    payload: web::Payload,
    svc: web::Data<ServiceContext>,
) -> Result<HttpResponse, Error> {
    let login_request = crate::pkg::json::unmarshal_from_payload::<LoginRequest>(payload);
    match user_logic::get_user_info(UserInfoRequest { id: 0 }, &svc) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Err(error::ErrorBadRequest(e.err)), //
    }
}

pub async fn save(req: HttpRequest,request: web::Json<UserSaveRequest>,svc: web::Data<ServiceContext>)->impl Responder{
    match user_logic::save(request.0, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}

pub async fn delete(req: HttpRequest,id: web::Path<i64>,svc: web::Data<ServiceContext>)->impl Responder{
    let request = UserDeleteRequest{id:id.into_inner()};
    match user_logic::delete(request, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}

pub async fn update(req: HttpRequest,id: web::Path<i64>,request: web::Json<UserUpdateRequest>,svc: web::Data<ServiceContext>)->impl Responder{
    let mut request = request.into_inner();
    request.user.id = *id;
    println!("Assigned ID: {}", request.user.id); // 调试信息
    match user_logic::update(request, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}


pub async fn search(req: HttpRequest,request: web::Json<UserSearchRequest>,svc: web::Data<ServiceContext>)->impl Responder{
    match user_logic::search(request.0, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
    // return Err(ServiceError::InternalServerError{
    //     error_message:"not found".to_string(),
    // } )
}

pub async fn get(req: HttpRequest,id: web::Path<i64>,svc: web::Data<ServiceContext>)->impl Responder{
    let request = UserGetRequest{id:id.into_inner()};
    match user_logic::get(request, &svc) {
        Ok(response) => success(response),
        Err(e) => fail(e, None),
    }
}