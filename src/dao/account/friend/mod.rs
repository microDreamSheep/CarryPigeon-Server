use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/**
好友关系
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct Friend {
    /// 唯一id
    pub id: Option<i64>,
    /// 发出申请的用户id
    pub person_1:Option<i64>,
    /// 接受申请的用户id
    pub person_2:Option<i64>,
    /// 申请状态
    pub state:Option<i32>,
    /// 发出申请的时间
    pub application_time:Option<DateTime>
}