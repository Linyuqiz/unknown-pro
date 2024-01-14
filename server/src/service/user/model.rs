use serde::{Deserialize, Serialize};

use crate::entity::user;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutUserResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListUserResponse {
    pub users: Vec<user::Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: Option<user::Model>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddUserResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserResponse {
    pub success: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    pub success: bool,
}
