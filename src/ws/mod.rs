use lazy_static::lazy_static;
use crate::ws::dispatcher::WebSocketDispatcher;

/**
 用于分发websocket的数据
 */
pub mod dispatcher;
/**
 自定义协议，用于处理websocket数据
 */
pub mod protocol;

lazy_static!(
 pub static ref WS_DISPATCHER:WebSocketDispatcher = WebSocketDispatcher::new()
 //.attach_path("/chat/tree_hole/send",tree_hole_send_controller)
 ;
);
