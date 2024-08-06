use rocket::serde::json::serde_json::json;
use crate::model::dto::ws::{WebSocketDataDTO};
use crate::model::response::WebSocketResponse;
use crate::ws::WS_DISPATCHER;

pub async fn dispatcher(
    data: WebSocketDataDTO
)->WebSocketResponse{
    let handler =WS_DISPATCHER.dispatch(&data.route);
    return match handler {
        None => {
            WebSocketResponse {
                code: 200,
                id: -1,
                data: Some(json!(format!("no such path {}",data.route))),
            }
        }
        Some(handler) => {
            println!("hello");
            handler.call((data.data,))
        }
    }
}