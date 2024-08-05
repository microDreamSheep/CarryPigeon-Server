use std::collections::HashMap;

pub struct WebSocketDispatcher {
    pub path_map:HashMap<String,fn(String)->String>
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
    )->fn(&str)->&str{
        return |s| {
            return s;
        }
    }

    pub fn attach_path(mut self, path:&str, handler:fn(String) ->String) ->WebSocketDispatcher{
        self.path_map.insert(path.to_string(), handler);
        self
    }
}