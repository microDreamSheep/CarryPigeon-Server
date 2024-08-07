use rbatis::{crud};
use rbatis::rbdc::DateTime;
use serde::{Deserialize, Serialize};

/**
用户
 */
#[derive(Clone, Default, Debug,Deserialize,Serialize)]
pub struct User {
    /// 用户唯一id
    pub id:Option<i64>,
    /// 用户名
    pub username:Option<String>,
    /// 用户密码
    pub password:Option<String>,
    /// 用户其他相关数据
    pub data:Option<String>,
    /// 用户注册时间
    pub register_time:Option<DateTime>
}

crud!(User{});