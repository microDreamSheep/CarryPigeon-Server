use chrono::{DateTime, Utc};
use rocket::{form::Form, FromForm};

#[derive(Debug, FromForm)]
struct GetMessageInfo {
    from: u128,
    to: u128,
    timestamp: rocket::time::PrimitiveDateTime,
    id: i64,
}

#[tracing::instrument]
#[rocket::get("/private/getmessage", data = "<get_message>")]
pub async fn get_message(get_message: Form<GetMessageInfo>) {
    let from = sqlx::types::uuid::Builder::from_u128(get_message.from).into_uuid();
    let to = sqlx::types::uuid::Builder::from_u128(get_message.to).into_uuid();
    let timestamp =
        DateTime::<Utc>::from_timestamp_millis(get_message.timestamp.assume_utc().unix_timestamp())
            .unwrap();

    crate::dao::chat::get_line(from, to, timestamp, get_message.id).await;
}
