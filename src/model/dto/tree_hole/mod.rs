use rocket::serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TreeHoleSendDTO {
    pub user_id: i64,
    pub data: String,
}
