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

#[derive(sqlx::FromRow, Clone, Default, Debug, Deserialize, Serialize)]
pub struct SocketMessage {
    pub message_type: i8,
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file: String,
    pub json: String,
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

pub enum MPSCMessage {
    GlobalMessage(GlobalMessage),
    GlobalMessageWithType(GlobalMessageWithType),
    SocketMessage(SocketMessage),
}
