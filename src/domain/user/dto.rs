use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub client_type: String,
    pub login_type: String,
    pub sms_code: String,
    pub phone: String,
}

#[derive(Deserialize, Serialize, Default)]
pub struct LoginResponse {
    pub name: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfoRequest {
    pub id: i64,
}
#[derive(Deserialize, Serialize, Default)]
pub struct UserInfoResponse {
    pub id: i64,
    pub name: String,
    pub avatar: String,
    pub client_type: String,
    pub phone: String,
}
