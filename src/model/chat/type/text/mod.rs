use crate::model::chat::CPMessageDataTrait;

/**
文本类型数据
 */
pub struct CPTextMessageData {
    pub msg: String,
}

impl CPMessageDataTrait for CPTextMessageData {
    fn get_message_type_id(&self) -> i32 {
        0
    }

    fn get_message_data(&self) -> String {
        format!("{{\"text\":\"{}\" }}", self.msg.clone())
    }
}

impl CPTextMessageData {
    pub(crate) fn new(msg: &str) -> CPTextMessageData {
        CPTextMessageData {
            msg: msg.to_string(),
        }
    }
}
