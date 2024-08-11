use rocket::serde::{Deserialize, Serialize};
use rocket_json_response::serialize_to_json;

/**
消息提醒，用于提醒客户端有一条新消息,仅用于通知，具体数据由客户端通过http请求获取(需要进行权限校验)
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageUpdateNotification {
    pub message_id: i64,
}

serialize_to_json!(MessageUpdateNotification);
