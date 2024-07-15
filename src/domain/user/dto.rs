use serde_derive::{Deserialize, Serialize};

// 登录
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    //pub client_type: String,
    pub login_type: String,
    pub sms_code: String,
    pub phone: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct LoginResponse {
    pub name: String,
    pub token: String,
}

// 用户信息
#[derive(Deserialize, Serialize)]
pub struct UserInfoRequest {
    pub id: i64,
}
#[derive(Deserialize, Serialize, Default)]
pub struct UserInfoResponse {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    //pub client_type: String,
    pub phone: String,
}

// 用户-保存
#[derive(Deserialize, Serialize, Default)]
pub struct UserSaveRequest {
    pub user: User
}
#[derive(Deserialize, Serialize, Default)]
pub struct UserSaveResponse {
    pub user: User
}

// 用户-更新
#[derive(Deserialize, Serialize, Default)]
pub struct UserUpdateRequest {
    pub user: User
}
#[derive(Deserialize, Serialize, Default)]
pub struct UserUpdateResponse {
    pub user: User
}

// 用户-详情
#[derive(Deserialize, Serialize, Default)]
pub struct UserGetRequest {
    pub id: i64,
}
#[derive(Deserialize, Serialize)]
pub struct UserGetResponse {
    pub user: User
}

// 用户-删除
#[derive(Deserialize, Serialize)]
pub struct UserDeleteRequest {
    pub id: i64,
}
#[derive(Deserialize, Serialize)]
pub struct UserDeleteResponse {
    pub user: User
}

// 用户-搜索
#[derive(Deserialize, Serialize)]
pub struct UserSearchRequest{
    pub page: i64,
    pub size: i64,
    pub name: String,
    pub phone: String,
}
#[derive(Deserialize, Serialize)]
pub struct UserSearchResponse{
    pub list: Vec<User>,
    pub total: i64,
}

#[derive(Deserialize, Serialize, Default)]
pub struct  User{
    #[serde(default)]
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub phone: String,
}