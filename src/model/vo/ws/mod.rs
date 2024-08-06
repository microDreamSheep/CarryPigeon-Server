use rocket::FromForm;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::serde_json;
use rocket_json_response::json_gettext::serde_json::Value;
use crate::model::dto::account::user::UserLoginDTO;
use crate::model::dto::ws::WebSocketDataDTO;

/**
用户登录，用于建立websocket连接
 */
#[derive(FromForm, Clone, Debug, Deserialize, Serialize)]
pub struct UserLoginVo {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

impl UserLoginVo {
    /**
    用于当前vo转化为controller与service间的数据结构
     */
    pub fn to_dto(self) ->UserLoginDTO{
        UserLoginDTO {
            username:self.username,
            password:self.password
        }
    }
}

/**
websocket数据模型，所有通过websocket的消息都必须满足此模型
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebSocketDataVO {
    /// 消息id，用于客户端进行本地异步处理
    pub request_id:i64,
    /// route 用于进行路径分配
    pub route: String,
    /// 具体的数据
    pub data:Value
}

impl WebSocketDataVO {
    pub fn new(
        text:&str
    ) ->serde_json::Result<WebSocketDataVO> {
        serde_json::from_str(text)
    }

    pub fn to_dto(self)->WebSocketDataDTO{
        WebSocketDataDTO{
            route: self.route,
            data: self.data,
        }
    }

}