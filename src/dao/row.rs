pub enum Status{
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
