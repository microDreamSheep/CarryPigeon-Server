// 一个完整的消息应该为两个包里类的组合模式
/**
tag用于对消息发送到的位置进行分类
 */
pub mod tag;
/**
type对不同的消息类型进行分类
 */
pub mod r#type;

use crate::dao::message::Message;

pub trait CPMessageTrait {
    fn to_message(self)->Message;
    fn set_data(&mut self,data:Box<dyn CPMessageDataTrait>);
}

pub trait CPMessageDataTrait {
    /// 获取消息的类型id
    fn get_message_type_id(&self)->i32;
    /// 获取具体的存储数据
    fn get_message_data(&self)->String;
}
