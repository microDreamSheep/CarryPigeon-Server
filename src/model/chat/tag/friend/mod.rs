use rbatis::rbdc::DateTime;
use crate::dao::message::Message;
use crate::model::chat::{CPMessageDataTrait, CPMessageTrait};
use crate::utils::id::generate_id;

pub struct FriendMessage{
    pub from_id:i64,
    pub to_id:i64,
    pub data: Box<dyn CPMessageDataTrait>
}

impl CPMessageTrait for FriendMessage{
    fn to_message(self) -> Message {
        Message{
            id: Some(generate_id()),
            from_id: Some(self.from_id),
            to_id: Some(self.to_id),
            message_tag: Some(2),
            data: Some(self.data.get_message_data()),
            message_type: Some(self.data.get_message_type_id()),
            time: Some(DateTime::now()),
        }
    }

    fn set_data(&mut self, data: Box<dyn CPMessageDataTrait>) {
        self.data = data
    }
}