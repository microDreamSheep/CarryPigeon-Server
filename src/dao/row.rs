use sqlx::types::chrono::Utc;

pub enum Status {
    Online,
    Offline,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub uuid: sqlx::types::Uuid,
    pub username: String,
    pub password: String,
    pub status: String,
}

#[derive(sqlx::FromRow)]
pub struct ChatPostTable {
    pub from: sqlx::types::Uuid,
    pub to: sqlx::types::Uuid,
    pub text: String,
    pub file_path: String,
    pub json: sqlx::types::JsonValue,
    pub timestamp: sqlx::types::chrono::DateTime<Utc>,
    pub id: i64,
}
