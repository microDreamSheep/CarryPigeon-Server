use std::collections::HashMap;
use tokio::sync::Mutex;
use std::sync::{Arc};
use lazy_static::lazy_static;
use rocket::futures::stream::SplitSink;
use rocket_ws::stream::DuplexStream;
use crate::model::ws::WSUser;

lazy_static! {
    pub static ref WEB_SOCKET_MANAGER: WebSocketManager = WebSocketManager::new();
}

#[derive(Clone)]
pub struct WebSocketManager(Arc<Mutex<WebSocketManagerInner>>);

impl WebSocketManager{
    pub fn new()->WebSocketManager{
        Self(Arc::new(Mutex::new(WebSocketManagerInner::new())))
    }

    /**
    注册用户流
     */
    pub async fn push_user(
        id:i64,
        stream: Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>,
        token:String
    ){
        let ws_user = WSUser::new(token,stream);
        WEB_SOCKET_MANAGER.0.lock().await.push(id,ws_user)
    }
    /**
    弹出用户流
     */
    pub async fn pop_user(
        id:i64
    ){
        WEB_SOCKET_MANAGER.0.lock().await.pop(id)
    }
    /**
     获取用户
     */
    pub async fn get_user_token(
        id:&i64
    ) -> Option<String> {
        WEB_SOCKET_MANAGER.0.lock().await.get_ws_user_token(id)
    }
}

/**
通道管理
此为简易实现 TODO 完善管理器
 */
struct WebSocketManagerInner {
    pub socket_map:HashMap<i64,WSUser>
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
        ws_user: WSUser
    ){
        self.socket_map.insert(id, ws_user);
    }

    pub fn pop(
        &mut self,
        id:i64
    ){
        self.socket_map.remove(&id);
    }

    pub fn get_sender(
        &self,
        id:i64
    ) -> Option<Arc<Mutex<SplitSink<DuplexStream, rocket_ws::Message>>>> {
        if !self.socket_map.contains_key(&id) {
            return None;
        }
        Some(Arc::clone(&self.socket_map.get(&id).unwrap().sender))
    }

    pub fn get_ws_user_token(
        &self,
        id:&i64
    ) -> Option<String> {
        if !self.socket_map.contains_key(id) {
            return None;
        }
        Some(self.socket_map.get(id).unwrap().token.clone())
    }
}