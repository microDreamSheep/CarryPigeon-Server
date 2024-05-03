use chrono::{DateTime, Utc};
use rocket::{form::Form, FromForm};

#[derive(Debug, FromForm)]
struct GetMessageInfo {
    from: i64,
    to: i64,
    timestamp: rocket::time::PrimitiveDateTime,
    id: i64,
}

#[tracing::instrument]
#[rocket::get("/private/getmessage", data = "<get_message>")]
pub async fn get_message(get_message: Form<GetMessageInfo>) {
    let timestamp =
        DateTime::<Utc>::from_timestamp_millis(get_message.timestamp.assume_utc().unix_timestamp())
            .unwrap();

    crate::dao::chat::get_line(get_message.from, get_message.to, timestamp, get_message.id).await;
}
