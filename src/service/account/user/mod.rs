use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::dao::account::user::User;
use crate::manager::ws::{WebSocketManager};
use crate::model::dto::account::user::{UserLoginDTO, UserRegisterDTO};
use crate::model::ws::CPSender;
use crate::repository::account::user::{insert_user, select_user_by_name};

/**
校验用户名是否存在
user_name:注册的用户名
 */
pub async fn is_user_name_contained_service(
    user_name:&str
)->bool{
    select_user_by_name(user_name).await.is_empty()
}

/**
注册一个新用户
user_info 用户名和密码的聚合类型
 */
pub async fn user_register_service(
    user_info:UserRegisterDTO
)->Result<String,String>{
    if !is_user_name_contained_service(&user_info.username).await {
        return Err("username already exists".to_string());
    }
    if insert_user(user_info.to_do()).await {
        return Ok("".to_string());
    }
    Err("register has some wrong".to_string())
}

/**
 用于用户进行登录
 */
pub async fn user_login_service(
    user_info:UserLoginDTO
)->Option<User>{
    let users = select_user_by_name(&user_info.username).await;
    if users.is_empty() {
        tracing::info!("{} login error:no such user",user_info.username);
        return None;
    }
    for user in users {
        return match &user.password {
            None => {
                tracing::error!("password of {} is empty",user.username.unwrap());
                None
            }
            Some(password) => {
                if !password.eq(&user_info.password) {
                    tracing::info!("{} login error: password wrong",user.username.unwrap());
                    return None;
                }
                Some(user)
            }
        }
    }
    return None;
}
/**
将用户放入
user 数据库中的user结构
stream 两者连接的WebSocket
 */
pub async fn push_user_service(
    user:User,
    sender:Arc<Mutex<CPSender>>,
    token:String
){
    // 对stream进行包装
    let id = user.id.unwrap();
    // 将其放入web ws manager进行管理
    WebSocketManager::push_user(id, sender, token).await;
    // 通知全局用户上线 TODO
}

/**
id:用户id
 */
pub async fn remove_user_service(id:i64){
    // 删除通道
    WebSocketManager::pop_user(id).await;
    // 通知全局用户注销 TODO
}

/**
权限校验
 */
pub async fn user_authority_check_service(id:&i64,token:String)->bool{
    let user_token = WebSocketManager::get_user_token(id).await;
    return match user_token {
        None => {
            false
        }
        Some(user_token) => {
            user_token.eq(&token)
        }
    }
}