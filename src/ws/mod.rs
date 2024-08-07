use lazy_static::lazy_static;
use crate::ws::dispatcher::WebSocketDispatcher;

/**
 用于分发websocket的数据
 */
pub mod dispatcher;
/**
 用于建立socket连接
 */
pub mod socket;
/*
定义全局的常量分发器，分发路径统一在此处进行注册，分发函数为
 fn(Value)->WebSocketResponse
*/
lazy_static!(
 pub static ref WS_DISPATCHER:WebSocketDispatcher = WebSocketDispatcher::new()
 //.attach_path("/chat/tree_hole/send",tree_hole_send_controller)
 ;
);