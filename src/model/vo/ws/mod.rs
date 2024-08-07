use rocket::serde::{Deserialize, Serialize};
use crate::model::dto::account::user::UserLoginDTO;

/**
用户登录，用于建立websocket连接
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
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
 用户登录的返回类型
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserLoginResponseVo {
    pub token:String
}