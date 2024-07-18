use crate::domain::user::entity::{ UserRepository};
use crate::pkg::{
    error::{self, InternalError},
    jwt,
    tool,
};
use crate::{
    domain::user::dto::{LoginRequest, LoginResponse, UserInfoRequest, UserInfoResponse},
    svc::context::ServiceContext,
};
use actix_web::web;
use crate::domain::user::dto::{UserDeleteRequest, UserDeleteResponse, UserGetRequest, UserGetResponse, UserSaveRequest, UserSaveResponse, UserSearchRequest, UserSearchResponse, UserUpdateRequest, UserUpdateResponse};
use crate::domain::user::dto;
use crate::domain::user::entity;

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
        .find_one(&mut svc.pool.get().unwrap(), req.id)
    {
        Ok(u) => {
            let mut user_token = crate::pkg::jwt::UserToken::default();
            user_token.user_id = u.id;
            Ok(UserInfoResponse {
                id: u.id.clone(),
                name: u.name.clone(),
                avatar: u.avatar.clone(),
                phone: u.phone.clone(),
            })
        }
        Err(_) => Err(error::new("phone not exists.".to_string())),
    }
}

pub fn save(req: UserSaveRequest,svc: &web::Data<ServiceContext>)->Result<UserSaveResponse,InternalError>{
    let user = svc.user_repository.insert(&mut svc.pool.get().unwrap(),new_domain_user_dto(req.user))?;
    Ok(UserSaveResponse{
        user:new_dto_user(user)
    })
}

pub fn delete(req: UserDeleteRequest,svc: &web::Data<ServiceContext>)->Result<UserDeleteResponse,InternalError>{
    // 方法一
    // match  svc.user_repository.find_one(&mut svc.pool.get().unwrap(),req.id){
    //     Ok(_)=>{
    //         let user = svc.user_repository.delete(&mut svc.pool.get().unwrap(),req.id)?;
    //         Ok(UserDeleteResponse{
    //             user:new_dto_user(user)
    //         })
    //     }
    //     Err(e) => Err(error::new("用户不存在".to_string()))
    // }
    // 方法二
    // svc.user_repository.find_one(&mut svc.pool.get().unwrap(),req.id)
    //     .map_err(|_|InternalError::new("用户不存在".to_string()))?;
    // 方法三
    let result =svc.user_repository.find_one(&mut svc.pool.get().unwrap(),req.id);
    if let Err(e) = result{
        return Err(InternalError::new("用户不存在".to_string()))
    }

    // 事务
    // svc.pool.get().unwrap().transaction(||{
    //     Ok(())
    // });

    let user = svc.user_repository.delete(&mut svc.pool.get().unwrap(),req.id)?;
    Ok(UserDeleteResponse{
        user:new_dto_user(user)
    })
}

pub fn update(req: UserUpdateRequest,svc: &web::Data<ServiceContext>)->Result<UserUpdateResponse,InternalError>{
    let mut user = svc.user_repository.find_one(&mut svc.pool.get().unwrap(),req.user.id)?;

    // 赋值
    user.avatar = req.user.avatar;
    user.name = req.user.name;
    user.phone = req.user.phone;
    user.updated_at = tool::time_now_unix();
    let user = svc.user_repository.update(&mut svc.pool.get().unwrap(),user)?;//new_domain_user(req.user)
    Ok(UserUpdateResponse{
        user:new_dto_user(user)
    })
}


pub fn get(req: UserGetRequest,svc: &web::Data<ServiceContext>)->Result<UserGetResponse,InternalError>{
    let user = svc.user_repository.find_one(&mut svc.pool.get().unwrap(),req.id)?;
    Ok(UserGetResponse{
        user:new_dto_user(user)
    })
}
pub fn search(req: UserSearchRequest,svc: &web::Data<ServiceContext>)->Result<UserSearchResponse,InternalError>{
    let  mut options = entity::UserQueryOptions
    {
        page: Some(req.page),
        size:Some(req.size),
        ..Default::default()
    };
    if req.name.len()>0{
        options.name = Some(req.name);
    }
    if req.phone.len()>0{
        options.phone = Some(req.phone);
    }
    let result = svc.user_repository.find_paginate(&mut svc.pool.get().unwrap(), options)?;
    let dto_users = new_dto_users(result.0);
    Ok(UserSearchResponse {
        list: dto_users,
        total: result.1, // 或者根据实际情况设置 total
    })
}


fn new_dto_user(user: entity::User)->dto::User{
    return dto::User{
        id:user.id,
        name:user.name,
        phone:user.phone,
        avatar:user.avatar,
    }
}

fn new_domain_user(user: dto::User)-> entity::User{
    let id = if user.id>0 {user.id}else{0};
    return entity::User{
        id,
        name:user.name,
        phone:user.phone,
        avatar:user.avatar,
        created_at: tool::time_now_unix(),
        updated_at: tool::time_now_unix(),
        ..Default::default()
    }
}

fn new_domain_user_dto(user: dto::User)-> entity::UserDTO{
    return entity::UserDTO{
        name:user.name,
        phone:user.phone,
        avatar:user.avatar,
        created_at: tool::time_now_unix(),
        updated_at: tool::time_now_unix(),
    }
}

fn new_dto_users(users: Vec<entity::User>)-> Vec<dto::User>{
    // 方法一： 遍历
    // let mut target_vec: Vec<dto::User> = Vec::new();
    // for item in users {
    //     target_vec.push(new_dto_user(item));
    // }
    // target_vec

    // 方法二：遍历
    users.into_iter().map(|user|new_dto_user(user)).collect()
}