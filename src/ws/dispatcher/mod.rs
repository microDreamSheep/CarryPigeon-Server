use std::collections::HashMap;
use rocket::serde::json::Value;
use crate::model::protocol::ws::request::WebSocketRequest;
use crate::model::protocol::ws::response::{WEBSOCKET_RESPONSE_ROUTE_ERROR, WebSocketResponse};
use crate::ws::WS_DISPATCHER;

/**
 请求分发
 */
pub async fn ws_dispatcher(
    request_data: WebSocketRequest
) ->WebSocketResponse{
    let handler =WS_DISPATCHER.dispatch(&request_data.route);
    return match handler {
        None => {
            WEBSOCKET_RESPONSE_ROUTE_ERROR.clone()
        }
        Some(handler) => {
            handler.call((request_data.data,))
        }
    }
}

/**
 websocket请求分发器，内部维护了一张路径的hash表，且初始化后不应该对路径进行更新，只读操作
 */
pub struct WebSocketDispatcher {
    pub route_map:HashMap<String,fn(Value) ->WebSocketResponse>
}

impl WebSocketDispatcher {
    pub fn new()->WebSocketDispatcher{
        WebSocketDispatcher{
            route_map: HashMap::new(),
        }
    }
    /**
     获取路径的引用
     */
    pub fn dispatch(
        &self,
        path:&str
    ) -> Option<&fn(Value) -> WebSocketResponse> {
        self.route_map.get(path)
    }

    /**
     注册route，应只在初始化时被调用
     */
    pub fn attach_route(mut self, route:&str, handler:fn(Value) ->WebSocketResponse) ->WebSocketDispatcher{
        self.route_map.insert(route.to_string(), handler);
        self
    }
}