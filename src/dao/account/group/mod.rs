use rbatis::crud;
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/**
群聊
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct Group {
    /// 群聊唯一id
    pub id:Option<i64>,
    /// 群聊名
    pub name:Option<String>,
    /// 群聊拥有者id
    pub own_user_id:Option<i64>,
    /// 群聊其他相关数据
    pub data:Option<String>,
    /// 群聊创建时间
    pub create_time:Option<DateTime>
}

crud!(Group{});