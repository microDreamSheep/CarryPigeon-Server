use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use rocket_ws::stream::DuplexStream;

lazy_static! {
    pub static ref WEB_SOCKET_MANAGER: WebSocketManager = WebSocketManager::new();
}

#[derive(Clone)]
struct WebSocketManager(Arc<Mutex<WebSocketManagerInner>>);

impl WebSocketManager{
    pub fn new()->WebSocketManager{
        Self(Arc::new(Mutex::new(WebSocketManagerInner::new())))
    }

    pub fn push(
        &self,
        id:i64,
        stream: Arc<DuplexStream>
    ){
        match self.0.lock() {
            Ok(mut it) => {
                it.push(id,stream)
            }
            Err(e) => {
                panic!("there is some wrong in push socket,wrong msg:{}",e)
            }
        }
    }
}

/**
通道管理
此为简易实现 TODO 完善管理器
 */
struct WebSocketManagerInner {
    pub socket_map:HashMap<i64,Arc<DuplexStream>>
}

impl WebSocketManagerInner{
    pub fn new()->WebSocketManagerInner{
        WebSocketManagerInner{
            socket_map: HashMap::new(),
        }
    }

    pub fn push(
        &mut self,
        id:i64,
        stream: Arc<DuplexStream>
    ){
        self.socket_map.insert(id,stream);
    }

    pub fn get(
        &self,
        id:i64
    ) -> Option<Arc<DuplexStream>> {
        if !self.socket_map.contains_key(&id) {
            return None;
        }
        Some(Arc::clone(self.socket_map.get(&id).unwrap()))
    }
}