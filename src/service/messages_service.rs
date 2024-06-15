use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, OnceLock,
    },
};

use chrono::Utc;
use rocket_ws::Message;

use crate::dao::{
    group::get_all_member,
    group_message, private_message,
    row::{GlobalMessage, GlobalMessageWithType, MPSCMessage, SocketMessage},
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
    fn receive_message(&self) -> impl std::future::Future<Output = Option<GlobalMessageWithType>>;
}

#[derive(Clone, Copy)]
pub struct MessageService {
    uuid: i64,
}

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
        let id = group_message::get_latest_message_id(group_id)
            .await
            .unwrap();

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
        group_message::push_group_message(&message_structure).await;

        let vec_member = get_all_member(group_id).await;
        for i in vec_member {
            let _ = match WS_HASHMAP.get().unwrap().lock().unwrap().get(&i) {
                // 该用户在线
                Some(v) => {
                    v.0.send(MPSCMessage::GlobalMessage(message_structure.clone()))
                }
                // 该用户不在线
                None => return,
            };
        }
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
        let id = private_message::get_latest_message_id(from, to)
            .await
            .unwrap();

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
        private_message::push_private_message(&message_structure).await;

        let _ = match WS_HASHMAP.get().unwrap().lock().unwrap().get(&to) {
            // 该用户在线
            Some(v) => {
                let result_message_structure = GlobalMessageWithType {
                    message_type: 1,
                    from,
                    to,
                    text: message_structure.text,
                    file: message_structure.file,
                    json: message_structure.json,
                    timestamp: message_structure.timestamp,
                    message_id: id,
                };
                v.0.send(MPSCMessage::GlobalMessageWithType(result_message_structure))
            }
            // 该用户不在线
            None => return,
        };
    }
}

impl SystemMessageService for MessageService {
    async fn receive_message(&self) -> Option<GlobalMessageWithType> {
        // 防止 binding 在 get_latest_message_id() 执行前就释放内存
        // 如果 binding 被 forget 就会内存泄漏
        // 如果 binding 不被 forget 就有可能 binding 被释放了但异步函数还为执行
        let receive_message;
        {
            let binding;
            loop {
                if WS_HASHMAP.get().unwrap().try_lock().is_ok() {
                    binding = WS_HASHMAP.get().unwrap().try_lock().unwrap();
                    break;
                }
            }
            let receiver = &binding.get(&self.uuid).unwrap().1;
            if receiver.try_recv().is_ok() {
                receive_message = receiver.try_recv().unwrap().clone();
            } else {
                return None;
            }
        }
        match receive_message {
            MPSCMessage::GlobalMessage(_) => {
                tracing::warn!("The accepted type is GlobalMessage, which lacks message_type and therefore does not know the sent object");
                None
            }
            MPSCMessage::GlobalMessageWithType(v) => Some(v),
            MPSCMessage::SocketMessage(v) => {
                let message_id;
                if v.message_type == 0 {
                    message_id = group_message::get_latest_message_id(v.to).await.unwrap();
                } else if v.message_type == 1 {
                    message_id = private_message::get_latest_message_id(v.from, v.to)
                        .await
                        .unwrap();
                } else {
                    tracing::warn!(
                        "Which lacks message_type and therefore does not know the sent object"
                    );
                    return None;
                }
                let result = GlobalMessageWithType {
                    message_type: v.message_type,
                    from: v.from,
                    to: v.to,
                    text: v.text,
                    file: v.file,
                    json: v.json,
                    timestamp: Utc::now().to_string(),
                    message_id,
                };
                Some(result)
            }
        }
    }
}

impl MessageService {
    pub fn new(uuid: i64) -> Self {
        Self { uuid }
    }

    /// 发送信息
    pub async fn message_service(&self, message: Message) {
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
    pub async fn receive_message(&self) -> Option<GlobalMessageWithType> {
        <MessageService as SystemMessageService>::receive_message(self).await
    }
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new(-1)
    }
}
