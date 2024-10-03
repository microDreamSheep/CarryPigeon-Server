use crate::{dao::{message::Message, MYSQL_POOL}, model::{protocol::ws::response::WebSocketResponse, vo::chat::ChatSendResponseVO}};
use rocket::serde::json::{from_value, Value};

/**
聊天

数据传入格式:

```rust
use rbatis::rbdc::DateTime;
pub struct Message {
    /// 消息唯一id
    pub id: Option<i64>,
    ///  消息发送者id
    pub from_id: Option<i64>,
    /// 消息发送到的位置，根据消息tag决定指向的为群聊id
    pub to_id: Option<i64>,
    /// 消息tag，决定消息的类型是群聊类型还是私聊类型还是树洞类型
    pub message_tag: Option<i32>,
    /// 消息的具体数据，通过解释引擎进行解释
    pub data: Option<String>,
    /// 消息类型 默认为0文本类型
    pub message_type: Option<i32>,
    ///  消息发送时间
    pub time: Option<DateTime>,
}
```
*/
pub async fn chat_send_controller(info: Value) -> WebSocketResponse {
    let value: Message = from_value(info).unwrap();
    match Message::insert(MYSQL_POOL.get().unwrap(), &value).await{
        Ok(_) => {
            WebSocketResponse::success(Value::String(ChatSendResponseVO::success().msg))
        },
        Err(e) => {
            WebSocketResponse::error(Value::String(e.to_string()))
        }
    }
}
