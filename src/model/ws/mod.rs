use std::sync::Arc;
use rocket::futures::stream::SplitSink;
use rocket_ws::stream::DuplexStream;
use tokio::sync::Mutex;

/**
ws通道管理的数据结构
包含用于权限校验的token和通信的sender
 */
pub struct WSUser{
    pub token:String,
    pub sender: Arc<Mutex<SplitSink<DuplexStream,rocket_ws::Message>>>
}

impl WSUser {
    pub fn new(
        token:String,
        sender: Arc<Mutex<SplitSink<DuplexStream,rocket_ws::Message>>>
    ) ->WSUser{
        WSUser{
            token,sender
        }
    }
}