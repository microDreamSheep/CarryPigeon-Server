use crate::dao::{group_message, private_message, row::GlobalMessage};

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
    /// 接受信息
    pub fn receive_message() {}
}
