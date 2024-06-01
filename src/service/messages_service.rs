use crate::dao::{group_message, private_message, row::{GlobalMessage, SocketMessage}};

pub trait GroupMessageService {
    /// 向群内发送信息
    fn send_message(
        group_id: i64,
        from: i64,
        text: String,
        file_path: String,
        json: String,
        timestamp: String,
    ) -> impl std::future::Future<Output = bool> + Send;
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
    ) -> impl std::future::Future<Output = bool> + Send;
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
    ) -> bool {
        // 查表找出最后一条信息的id
        let id = group_message::get_latest_message_id(group_id).await;

        // 构造数据
        let message_structure = GlobalMessage {
            message_type: 0,
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

        // todo!("通知群内的人接收");

        false
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
    ) -> bool {
        // 查表找出最后一条信息的id
        let id = private_message::get_latest_message_id(from, to).await;

        // 构造数据
        let message_structure = GlobalMessage {
            message_type: 1,
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

        //todo!("通知对方的人接收");

        false
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
    pub async fn send_message(&self, message: String) {
        let json_value: SocketMessage = serde_json::from_str(message.as_str()).unwrap();
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
