use std::sync::Arc;
use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::stream::DuplexStream;
use crate::dao::account::user::User;
use crate::manager::ws::WEB_SOCKET_MANAGER;
use crate::model::dto::account::user::{UserLoginDTO, UserRegisterDTO};
use crate::model::response::Response;
use crate::repository::account::user::{insert_user, select_user_by_name};

/**
校验用户名是否存在
user_name:注册的用户名
 */
pub async fn is_user_name_contained(
    user_name:&str
)->bool{
    select_user_by_name(user_name).await.is_empty()
}

/**
注册一个新用户
user_info 用户名和密码的聚合类型
 */
pub async fn new_user(
    user_info:UserRegisterDTO
)->Result<String,String>{
    if !is_user_name_contained(&user_info.username).await {
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
pub async fn login(
    user_info:UserLoginDTO,
    mut stream:DuplexStream
){
    let users = select_user_by_name(&user_info.username).await;
    if users.is_empty() {
        let _ = stream.send("login error:no user".into()).await;
        tracing::info!("{} login error:no such user",user_info.username);
        return;
    }
    for user in users {
        return match &user.password {
            None => {
                tracing::error!("password of {} is empty",user.username.unwrap())
            }
            Some(password) => {
                if !password.eq(&user_info.password) {
                    let _ = stream.send("login error: password wrong".into()).await;
                    tracing::info!("{} login error: password wrong",user.username.unwrap());
                    return;
                }
                push_user(user,stream)
            }
        }
    }
}
/**
user 数据库中的user结构
stream 两者连接的WebSocket
 */
pub async fn push_user(
    user:User,
    stream:DuplexStream
){
    // 对stream进行包装
    let stream = Arc::new(stream);
    let id = user.id.unwrap();
    // 将其放入web socket manager进行管理
    WEB_SOCKET_MANAGER.push(id, Arc::clone(&stream));
    // 放入自定义协议进行分流

}