use rbatis::rbdc::DateTime;
use crate::dao::account::user::User;
use crate::utils::id::generate_id;

/**
 用于服务层使用的用户注册数据结构
 */
#[derive(Debug,Clone)]
pub struct UserRegisterDTO{
    pub username:String,
    pub password:String
}

impl UserRegisterDTO {
    pub fn to_do(self)->User{
        User{
            id: Some(generate_id()),
            username: Some(self.username),
            password: Some(self.password),
            data: None,
            register_time: Some(DateTime::now()),
        }
    }
}

/**
 用于服务层使用的用户登录数据结构
 */
#[derive(Debug,Clone)]
pub struct UserLoginDTO{
    pub username:String,
    pub password:String
}

impl UserLoginDTO {
    pub fn to_do(self)->User{
        User{
            id: None,
            username: Some(self.username),
            password: Some(self.password),
            data: None,
            register_time: None,
        }
    }
}