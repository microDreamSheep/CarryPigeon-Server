use crate::model::dto::ws::{WebSocketDataDTO};
use crate::model::response::{WEBSOCKET_RESPONSE_ROUTE_ERROR, WebSocketResponse};
use crate::ws::WS_DISPATCHER;

pub async fn dispatcher_service(
    data: WebSocketDataDTO
)->WebSocketResponse{
    let handler =WS_DISPATCHER.dispatch(&data.route);
    return match handler {
        None => {
            WEBSOCKET_RESPONSE_ROUTE_ERROR.clone()
        }
        Some(handler) => {
            handler.call((data.data,))
        }
    }
}