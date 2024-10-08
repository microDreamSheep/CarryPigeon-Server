/*!
全局通道管理，用于管理当前在线的用户及其通道

所有对外提供的方法封装在WebSocketManager中
 */

use crate::model::ws::{CPSender, WSUser};
use std::sync::Arc;
use std::{collections::HashMap, sync::OnceLock};
use tokio::sync::Mutex;

pub static WEB_SOCKET_MANAGER: OnceLock<WebSocketManager> = OnceLock::new();

pub fn init_web_socket_manager() {
    let _ = WEB_SOCKET_MANAGER.set(WebSocketManager::new());
}

#[derive(Clone)]
pub struct WebSocketManager(Arc<Mutex<WebSocketManagerInner>>);

impl WebSocketManager {
    fn new() -> WebSocketManager {
        Self(Arc::new(Mutex::new(WebSocketManagerInner::new())))
    }

    /**
    注册用户流
     */
    pub async fn push_user(id: i64, sender: Arc<Mutex<CPSender>>, token: String) {
        let ws_user = WSUser::new(token, sender);
        WEB_SOCKET_MANAGER
            .get()
            .unwrap()
            .0
            .lock()
            .await
            .push(id, ws_user)
    }
    /**
    弹出用户流
     */
    pub async fn pop_user(id: i64) {
        WEB_SOCKET_MANAGER.get().unwrap().0.lock().await.pop(id)
    }
    /**
    获取用户
    */
    pub async fn get_user_token(id: &i64) -> Option<String> {
        WEB_SOCKET_MANAGER
            .get()
            .unwrap()
            .0
            .lock()
            .await
            .get_ws_user_token(id)
    }
    /**
    获取消息发送器
    */
    pub async fn get_sender(id: &i64) -> Option<Arc<Mutex<CPSender>>> {
        WEB_SOCKET_MANAGER
            .get()
            .unwrap()
            .0
            .lock()
            .await
            .get_sender(id)
    }

    /**
    判断用户是否在线
    */
    pub async fn is_online(id: &i64) -> bool {
        Self::get_user_token(id).await.is_some()
    }
}

/**
通道管理
此为简易实现 TODO 完善管理器
 */
struct WebSocketManagerInner {
    socket_map: HashMap<i64, WSUser>,
}

impl WebSocketManagerInner {
    fn new() -> WebSocketManagerInner {
        WebSocketManagerInner {
            socket_map: HashMap::new(),
        }
    }

    fn push(&mut self, id: i64, ws_user: WSUser) {
        self.socket_map.insert(id, ws_user);
    }

    fn pop(&mut self, id: i64) {
        self.socket_map.remove(&id);
    }

    fn get_sender(&self, id: &i64) -> Option<Arc<Mutex<CPSender>>> {
        if !self.socket_map.contains_key(id) {
            return None;
        }
        Some(Arc::clone(&self.socket_map.get(id).unwrap().sender))
    }

    fn get_ws_user_token(&self, id: &i64) -> Option<String> {
        if !self.socket_map.contains_key(id) {
            return None;
        }
        Some(self.socket_map.get(id).unwrap().token.clone())
    }
}
