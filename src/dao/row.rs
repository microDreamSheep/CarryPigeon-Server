use rocket::FromForm;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;

pub enum Status {
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

#[derive(sqlx::FromRow, Clone, Default)]
pub struct ChatOfflineMessage {
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file_path: String,
    pub json: sqlx::types::JsonValue,
    pub timestamp: chrono::DateTime<Utc>,
    pub id: i64,
}

#[derive(sqlx::FromRow, Deserialize, Serialize, FromForm, Default)]
pub struct UserToken {
    pub uuid: i64,
    pub public_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct GobalMessage {
    pub message_type: String,
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file: String,
    pub json: String,
    pub timestamp: String,
    pub id: i64,
}
