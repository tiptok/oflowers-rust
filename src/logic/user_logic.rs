use crate::domain::user::entity::UserRepository;
use crate::pkg::{
    error::{self, InternalError},
    jwt,
};
use crate::{
    domain::user::dto::{LoginRequest, LoginResponse, UserInfoRequest, UserInfoResponse},
    svc::context::ServiceContext,
};
use actix_web::web;

pub fn login(
    req: LoginRequest,
    svc: &web::Data<ServiceContext>,
) -> Result<LoginResponse, InternalError> {
    match &svc
        .user_repository
        .find_one_by_phone(&mut svc.pool.get().unwrap(), &req.phone)
    {
        Ok(u) => {
            // 构建UserToken对象
            let mut user_token = jwt::UserToken::default();
            user_token.user_id = u.id;
            Ok(LoginResponse {
                name: u.name.to_string(),
                token: user_token.encode(svc.config.jwt_secret.to_string(), svc.config.jwt_expire), // JWT编码
            })
        }
        Err(_) => Err(error::new("phone not exists.".to_string())),
    }
}

pub fn get_user_info(
    req: UserInfoRequest,
    svc: &web::Data<ServiceContext>,
) -> Result<UserInfoResponse, InternalError> {
    match &svc
        .user_repository
        .find_one(&mut svc.pool.get().unwrap(), &req.id)
    {
        Ok(u) => {
            let mut user_token = crate::pkg::jwt::UserToken::default();
            user_token.user_id = u.id;
            Ok(UserInfoResponse {
                id: u.id.clone(),
                name: u.name.clone(),
                avatar: u.avatar.clone(),
                client_type: u.client_type.clone(),
                phone: u.phone.clone(),
            })
        }
        Err(_) => Err(error::new("phone not exists.".to_string())),
    }
}
