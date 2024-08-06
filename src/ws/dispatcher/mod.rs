use std::collections::HashMap;
use rocket::serde::json::Value;
use crate::model::response::WebSocketResponse;

pub struct WebSocketDispatcher {
    pub path_map:HashMap<String,fn(Value)->WebSocketResponse>
}

impl WebSocketDispatcher {
    pub fn new()->WebSocketDispatcher{
        WebSocketDispatcher{
            path_map: HashMap::new(),
        }
    }

    pub fn dispatch(
        &self,
        path:&str
    ) -> Option<&fn(Value) -> WebSocketResponse> {
        self.path_map.get(path)
    }

    pub fn attach_path(mut self, path:&str, handler:fn(Value) ->WebSocketResponse) ->WebSocketDispatcher{
        self.path_map.insert(path.to_string(), handler);
        self
    }
}