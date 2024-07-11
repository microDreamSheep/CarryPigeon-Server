use std::future::Future;
use std::sync::mpsc::channel;
use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex, OnceLock,
    },
};

use crate::dao::group_message::delete_group_message;
use crate::dao::private_message::delete_private_message;
use crate::dao::row::SocketMessage;
use crate::dao::{
    group::get_all_member,
    group_message, private_message,
    row::{GlobalMessage, GlobalMessageWithType, MPSCMessage},
    REDIS_POOL,
};
use chrono::Utc;
use redis::AsyncCommands;
use rocket_ws::Message;

#[allow(clippy::type_complexity)]
pub static WS_HASHMAP: OnceLock<Mutex<HashMap<i64, Sender<MPSCMessage>>>> = OnceLock::new();

pub async fn init_ws_hashmap() {
    let _ = WS_HASHMAP.set(Mutex::new(HashMap::new()));
}

pub trait GroupMessageService {
    /// 向群内发送信息
    fn send_message(
        &self,
        group_id: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) -> impl Future<Output = ()> + Send;
    fn delete_message(&self, group_id: i64, message_id: i64) -> impl Future<Output = ()> + Send;
}

pub trait PrivateMessageService {
    /// 向个人发送信息
    fn send_message(
        &self,
        to: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) -> impl Future<Output = ()> + Send;
    fn delete_message(
        &self,
        from: i64,
        to: i64,
        message_id: i64,
    ) -> impl Future<Output = ()> + Send;
}

pub trait SystemMessageService {
    fn receive_message(&self) -> impl Future<Output = Option<GlobalMessageWithType>>;
    fn receive_offline_message(&self) -> impl Future<Output = Option<Vec<GlobalMessageWithType>>>;
}

pub struct MessageService {
    uuid: i64,
    receiver: Mutex<Receiver<MPSCMessage>>,
}

impl GroupMessageService for MessageService {
    async fn send_message(
        &self,
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
        let message_structure = Box::new(GlobalMessage {
            from,
            to: group_id,
            text,
            file: file_path,
            json,
            timestamp,
            message_id: id,
        });

        // 保存到数据库
        let _: () = unsafe {
            let mut temp: String = REDIS_POOL
                .get_mut()
                .unwrap()
                .get_del(message_structure.to)
                .await
                .unwrap();
            let new_message = Box::new(
                serde_json::to_string(&GlobalMessageWithType {
                    message_type: 0,
                    from,
                    to: group_id,
                    text: message_structure.text.clone(),
                    file: message_structure.file.clone(),
                    json: message_structure.json.clone(),
                    timestamp: message_structure.timestamp.clone(),
                    message_id: id,
                })
                .unwrap(),
            );
            temp.push(' ');
            temp.push_str((*new_message).as_str());

            let _: () = REDIS_POOL
                .get_mut()
                .unwrap()
                .set(message_structure.to, temp)
                .await
                .unwrap();
        };
        group_message::push_group_message(&message_structure).await;

        let vec_member = get_all_member(group_id).await;
        for i in vec_member {
            let _ = match WS_HASHMAP.get().unwrap().lock().unwrap().get(&i) {
                // 该用户在线
                Some(v) => v.send(MPSCMessage::GlobalMessage(*message_structure.clone())),
                // 该用户不在线
                None => return,
            };
        }
    }
    async fn delete_message(&self, group_id: i64, message_id: i64) {
        delete_group_message(group_id, message_id).await;
    }
}

impl PrivateMessageService for MessageService {
    async fn send_message(
        &self,
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
        let _: () = unsafe {
            let mut temp: String = REDIS_POOL
                .get_mut()
                .unwrap()
                .get_del(message_structure.to)
                .await
                .unwrap();
            let new_message = Box::new(
                serde_json::to_string(&GlobalMessageWithType {
                    message_type: 1,
                    from,
                    to,
                    text: message_structure.text.clone(),
                    file: message_structure.file.clone(),
                    json: message_structure.json.clone(),
                    timestamp: message_structure.timestamp.clone(),
                    message_id: id,
                })
                .unwrap(),
            );
            temp.push(' ');
            temp.push_str((*new_message).as_str());

            let _: () = REDIS_POOL
                .get_mut()
                .unwrap()
                .set(message_structure.to, temp)
                .await
                .unwrap();
        };
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
                v.send(MPSCMessage::GlobalMessageWithType(result_message_structure))
            }
            // 该用户不在线
            None => return,
        };
    }
    async fn delete_message(&self, from: i64, to: i64, message_id: i64) {
        delete_private_message(from, from, to, message_id).await;
        delete_private_message(to, from, to, message_id).await;
    }
}

impl SystemMessageService for MessageService {
    async fn receive_message(&self) -> Option<GlobalMessageWithType> {
        let receive_message = if self.receiver.try_lock().unwrap().try_recv().is_ok() {
            Box::new(
                self.receiver
                    .try_lock()
                    .unwrap()
                    .try_recv()
                    .unwrap()
                    .clone(),
            )
        } else {
            return None;
        };

        // 开始处理 MPSC 消息队列
        match *receive_message {
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
    async fn receive_offline_message(&self) -> Option<Vec<GlobalMessageWithType>> {
        let mut result: Vec<GlobalMessageWithType> = vec![];
        let mut from: i64;
        let mut text: String;
        let mut message_id: i64;
        let mut message_type: i8;
        unsafe {
            loop {
                match REDIS_POOL
                    .get_mut()
                    .unwrap()
                    .get::<i64, (i64, String, i64, i8)>(self.uuid)
                    .await
                {
                    Ok(_) => {
                        (from, text, message_id, message_type) = REDIS_POOL
                            .get_mut()
                            .unwrap()
                            .get_del(self.uuid)
                            .await
                            .unwrap();
                        let temp = Box::new(GlobalMessageWithType {
                            message_type,
                            from,
                            to: self.uuid,
                            text,
                            file: "".to_string(),
                            json: "".to_string(),
                            timestamp: "".to_string(),
                            message_id,
                        });
                        result.push(*temp);
                    }
                    Err(_) => {
                        return if result.is_empty() {
                            None
                        } else {
                            Some(result)
                        }
                    }
                }
            }
        }
    }
}

impl MessageService {
    pub fn new(uuid: i64) -> Self {
        let (tx, rx) = channel::<MPSCMessage>();
        WS_HASHMAP.get().unwrap().lock().unwrap().insert(uuid, tx);
        Self {
            uuid,
            receiver: Mutex::new(rx),
        }
    }

    /// 信息服务
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
        let timestamp = Utc::now().to_string();
        match json_value {
            // 处理信息的发送
            SocketMessage::SocketMessage(v) => {
                // 处理群聊的信息
                if v.message_type == 0 {
                    unsafe {
                        let _: () = REDIS_POOL.get_mut().unwrap().del(self.uuid).await.unwrap();
                    };
                    <MessageService as GroupMessageService>::send_message(
                        self, v.to, v.from, v.text, v.file, v.json, timestamp,
                    )
                    .await;
                }
                // 处理私聊信息
                else if v.message_type == 1 {
                    unsafe {
                        let _: () = REDIS_POOL.get_mut().unwrap().del(self.uuid).await.unwrap();
                    };
                    <MessageService as PrivateMessageService>::send_message(
                        self, v.to, v.from, v.text, v.file, v.json, timestamp,
                    )
                    .await;
                }
            }
            // 处理信息的删除
            SocketMessage::DeleteMessage(v) => {
                // 处理群聊的信息
                if v.message_type == 0 {
                    <MessageService as GroupMessageService>::delete_message(
                        self,
                        v.to,
                        v.message_id,
                    )
                    .await;
                } else if v.message_type == 1 {
                    <MessageService as PrivateMessageService>::delete_message(
                        self,
                        v.from,
                        v.to,
                        v.message_id,
                    )
                    .await;
                }
            }
        }
    }
    /// 接受信息
    pub async fn receive_message(&self) -> Option<GlobalMessageWithType> {
        <MessageService as SystemMessageService>::receive_message(self).await
    }
    pub async fn handle_ping_message(&self, message: Message) -> bool {
        if message.is_ping() {
            return true;
        }
        false
    }
}

impl Default for MessageService {
    fn default() -> Self {
        Self::new(-1)
    }
}
