use rocket::serde::json::{Value};

pub struct WebSocketDataDTO{
    /// route 用于进行路径分配
    pub route: String,
    /// 具体的数据
    pub data:Value
}