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

#[derive(sqlx::FromRow)]
pub struct ChatOfflineMessage {
    pub from: i64,
    pub to: i64,
    pub text: String,
    pub file_path: String,
    pub json: sqlx::types::JsonValue,
    pub timestamp: sqlx::types::chrono::DateTime<Utc>,
    pub id: i64,
}

#[derive(sqlx::FromRow)]
pub struct UserToken {
    pub uuid: i64,
    pub token: sqlx::types::JsonValue,
}
