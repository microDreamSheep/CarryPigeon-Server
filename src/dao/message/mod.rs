use rbatis::crud;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/**
具体聊天数据的Do模型
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct Message {
    /// 消息唯一id
    pub id:Option<i64>,
    ///  消息发送者id
    pub from_id:Option<i64>,
    /// 消息发送到的位置，根据消息tag决定指向的为群聊id
    pub to_id:Option<i64>,
    /// 消息tag，决定消息的类型是群聊类型还是私聊类型还是树洞类型
    pub message_tag:Option<i32>,
    /// 消息的具体数据，通过解释引擎进行解释
    pub data:Option<String>,
    /// 消息类型 默认为0文本类型
    pub message_type:Option<i32>,
    ///  消息发送时间
    pub time:Option<DateTime>
}

crud!(Message{});