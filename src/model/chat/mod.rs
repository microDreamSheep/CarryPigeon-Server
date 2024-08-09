/*!
该包用于定义聊天中具体的消息数据的模型，一个完整的消息应该为tag包里封装type类型的数据
 */

/**
tag用于对消息发送到的位置进行分类，包含:群聊，私聊，树洞
 */
pub mod tag;
/**
type对不同的消息类型进行分类，包含：文本类型，文件类型，超链接类型
 */
pub mod r#type;

use crate::dao::message::Message;

pub trait CPMessageTrait {
    /// 将消息转换为存储在数据库中的数据格式
    fn to_message(self)->Message;
    /// 设置消息的具体数据
    fn set_data(&mut self,data:Box<dyn CPMessageDataTrait>);
}

pub trait CPMessageDataTrait {
    /// 获取消息的类型id
    fn get_message_type_id(&self)->i32;
    /// 获取具体的存储数据
    fn get_message_data(&self)->String;
}
