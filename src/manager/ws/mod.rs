use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::{Arc};
use lazy_static::lazy_static;
use rocket::futures::stream::SplitSink;
use rocket_ws::stream::DuplexStream;
use tokio_test::block_on;

lazy_static! {
    pub static ref WEB_SOCKET_MANAGER: WebSocketManager = WebSocketManager::new();
}

/**
 注册用户流
 */
pub async fn push_user_stream(
    id:i64,
    stream: Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>
){
    WEB_SOCKET_MANAGER.0.lock().await.push(id,stream)
}
/**
 弹出用户流
 */
pub async fn pop_user_stream(
    id:i64
){
    WEB_SOCKET_MANAGER.0.lock().await.pop(id)
}

#[derive(Clone)]
pub struct WebSocketManager(Arc<Mutex<WebSocketManagerInner>>);

impl WebSocketManager{
    pub fn new()->WebSocketManager{
        Self(Arc::new(Mutex::new(WebSocketManagerInner::new())))
    }
}

/**
通道管理
此为简易实现 TODO 完善管理器
 */
struct WebSocketManagerInner {
    pub socket_map:HashMap<i64,Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>>
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
        stream: Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>
    ){
        self.socket_map.insert(id,stream);
    }

    pub fn pop(
        &mut self,
        id:i64
    ){
        self.socket_map.remove(&id);
    }

    pub fn get(
        &self,
        id:i64
    ) -> Option<Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>> {
        if !self.socket_map.contains_key(&id) {
            return None;
        }
        Some(Arc::clone(self.socket_map.get(&id).unwrap()))
    }
}