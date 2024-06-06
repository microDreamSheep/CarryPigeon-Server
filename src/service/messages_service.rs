use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, OnceLock,
    },
};

use rocket_ws::Message;

use crate::dao::{
    group_message, private_message,
    row::{GlobalMessage, MPSCMessage, SocketMessage},
};

#[allow(clippy::type_complexity)]
pub static WS_HASHMAP: OnceLock<Mutex<HashMap<i64, (Sender<MPSCMessage>, Receiver<MPSCMessage>)>>> =
    OnceLock::new();

pub async fn init_ws_hashmap() {
    let _ = WS_HASHMAP.set(Mutex::new(HashMap::new()));
}

pub trait GroupMessageService {
    /// 向群内发送信息
    fn send_message(
        group_id: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) -> impl std::future::Future<Output = ()> + Send;
}

pub trait PrivateMessageService {
    /// 向个人发送信息
    fn send_message(
        to: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) -> impl std::future::Future<Output = ()> + Send;
}

pub trait SystemMessageService {
    fn receive_message();
}

#[derive(Clone, Copy)]
pub struct MessageService;

impl GroupMessageService for MessageService {
    async fn send_message(
        group_id: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) {
        // 查表找出最后一条信息的id
        let id = group_message::get_latest_message_id(group_id).await;

        // 构造数据
        let message_structure = GlobalMessage {
            from,
            to: group_id,
            text,
            file: file_path,
            json,
            timestamp,
            message_id: id,
        };

        // 保存到数据库
        group_message::update_group_message(&message_structure).await;

        // todo!("通知所有群内的人，故此处的实现逻辑错误");
        let _ = match WS_HASHMAP.get().unwrap().lock().unwrap().get(&group_id) {
            // 该用户在线
            Some(v) => v.0.send(MPSCMessage::GlobalMessage(message_structure)),
            // 该用户不在线
            None => return,
        };
    }
}

impl PrivateMessageService for MessageService {
    async fn send_message(
        to: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) {
        // 查表找出最后一条信息的id
        let id = private_message::get_latest_message_id(from, to).await;

        // 构造数据
        let message_structure = GlobalMessage {
            from,
            to,
            text,
            file: file_path,
            json,
            timestamp,
            message_id: id,
        };

        // 保存到数据库
        private_message::update_private_message(&message_structure).await;

        let _ = match WS_HASHMAP.get().unwrap().lock().unwrap().get(&to) {
            // 该用户在线
            Some(v) => v.0.send(MPSCMessage::GlobalMessage(message_structure)),
            // 该用户不在线
            None => return,
        };
    }
}

impl SystemMessageService for MessageService {
    fn receive_message() {
        todo!()
    }
}

impl MessageService {
    pub fn new() -> Self {
        Self
    }

    /// 发送信息
    pub async fn message_service(&self, message: Message) {
        // 判断消息类型

        // 当message为close or empty信号时
        if message.is_close() || message.is_empty() {
            return;
        }
        // 当message为binary信号时
        else if message.is_binary() {
        }

        // 处理消息
        let message_value = message.to_string();

        // 这里可以放心解析为SocketMessage
        let json_value: SocketMessage = match serde_json::from_str(message_value.as_str()) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("{}", e);
                return;
            }
        };
        let timestamp = chrono::Utc::now().to_string();
        // 处理群聊的信息
        if json_value.message_type == 0 {
            let _result = <MessageService as GroupMessageService>::send_message(
                json_value.to,
                json_value.from,
                json_value.text,
                json_value.file,
                json_value.json,
                timestamp,
            )
            .await;
        }
        // 处理私聊信息
        else if json_value.message_type == 1 {
            let _result = <MessageService as PrivateMessageService>::send_message(
                json_value.to,
                json_value.from,
                json_value.text,
                json_value.file,
                json_value.json,
                timestamp,
            )
            .await;
        }
    }
    /// 接受信息
    pub fn receive_message(&self) {}
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new()
    }
}
