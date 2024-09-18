use crate::{
    dao::{message::Message, MYSQL_POOL},
    model::protocol::ws::response::WebSocketResponse,
};
use rocket::serde::json::{from_value, Value};

/**
聊天

数据传入格式:

```json
*/
pub async fn chat_sender_controller(info: Value) -> WebSocketResponse {
    let value: Message = from_value(info).unwrap();
    match Message::insert(MYSQL_POOL.get().unwrap(), &value).await {
        Ok(_) => {
            WebSocketResponse::success(Value::String(r#"{status: "success")}"#.to_string()));
        }
        Err(e) => {
            WebSocketResponse::error(Value::String(e.to_string()));
        }
    }
    WebSocketResponse::error("".into())
}
