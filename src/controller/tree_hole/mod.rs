use rocket::serde::json::Value;

use crate::model::response::{WEBSOCKET_RESPONSE_CONTENT_STRUCTURE_ERROR, WebSocketResponse};

pub fn tree_hole_send_controller(s:Value) -> WebSocketResponse{
    WEBSOCKET_RESPONSE_CONTENT_STRUCTURE_ERROR.clone()
}
