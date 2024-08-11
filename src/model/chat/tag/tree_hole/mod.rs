use crate::dao::message::Message;
use crate::model::chat::{CPMessageDataTrait, CPMessageTrait};
use crate::utils::id::generate_id;
use rbatis::rbdc::DateTime;

pub struct TreeHoleMessage {
    pub from_id: i64,
    pub data: Box<dyn CPMessageDataTrait>,
}

impl CPMessageTrait for TreeHoleMessage {
    fn to_message(self) -> Message {
        Message {
            id: Some(generate_id()),
            from_id: Some(self.from_id),
            to_id: Some(-1),
            message_tag: Some(3),
            data: Some(self.data.get_message_data()),
            message_type: Some(self.data.get_message_type_id()),
            time: Some(DateTime::now()),
        }
    }

    fn set_data(&mut self, data: Box<dyn CPMessageDataTrait>) {
        self.data = data
    }
}
