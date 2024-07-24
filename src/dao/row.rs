use rocket::FromForm;
use serde::{Deserialize, Serialize};

pub enum UserStatus {
    Online,
    Offline,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub uuid: i64,
    pub username: String,
    pub password: String,
    pub status: String,
}

#[derive(sqlx::FromRow, Clone, Default, Deserialize, Serialize)]
pub struct ChatOfflineMessage {
    pub from: i64,
    pub to: i64,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, FromForm, Default)]
pub struct UserToken {
    pub uuid: i64,
    pub public_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SocketMessage {
    SocketMessage(SocketMessageInfo),
    DeleteMessage(DeleteMessage),
}

#[derive(sqlx::FromRow, Clone, Default, Debug, Deserialize, Serialize)]
pub struct SocketMessageInfo {
    pub message_type: i8,
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file: String,
    pub json: String,
    pub aes_key: String,
    pub aes_iv: String,
}

#[derive(sqlx::FromRow, Clone, Default, Debug, Deserialize, Serialize)]
pub struct GlobalMessage {
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file: String,
    pub json: String,
    pub timestamp: String,
    pub message_id: i64,
    pub aes_key: String,
    pub aes_iv: String,
}

#[derive(sqlx::FromRow, Clone, Default, Debug, Deserialize, Serialize)]
pub struct GlobalMessageWithType {
    // message_type
    // -1 -> all
    // 0 -> group
    // 1 -> private_message
    pub message_type: i8,
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file: String,
    pub json: String,
    pub timestamp: String,
    pub message_id: i64,
}

#[derive(sqlx::FromRow, Clone, Default, Debug, Deserialize, Serialize)]
pub struct DeleteMessage {
    pub message_type: i8,
    pub from: i64,
    pub to: i64,
    pub message_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MPSCMessage {
    GlobalMessage(GlobalMessage),
    GlobalMessageWithType(GlobalMessageWithType),
    SocketMessage(SocketMessageInfo),
}

#[derive(sqlx::FromRow, FromForm, Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub owner: i64,
    pub admin: Vec<i64>,
    pub member: Vec<i64>,
}

#[derive(sqlx::FromRow, FromForm, Clone, Debug, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    pub owner_id: i64,
    pub token: String,
    pub member: Vec<i64>,
}

#[derive(sqlx::FromRow, FromForm, Clone, Debug, Deserialize, Serialize)]
pub struct CreateAccountRequest {
    pub username: String,
    pub password: String,
}
