use tokio::sync::Mutex;
use rocket::futures::{SinkExt, StreamExt};
use std::sync::{Arc};
use base64::Engine;
use base64::engine::general_purpose;
use rbatis::rbatis_codegen::ops::AsProxy;
use tokio::io;
use crate::dao::account::user::User;
use crate::service::account::user::{user_login_service, push_user_service, remove_user_service};
use crate::model::vo::ws::{UserLoginResponseVo, UserLoginVo, WebSocketDataVO};
use rocket::get;
use rocket::serde::json::Json;
use rocket::serde::json::serde_json::json;
use rocket_ws::Message;
use rocket_ws::result::Error;
use tokio_test::block_on;
use crate::model::response::WebSocketResponse;
use crate::service::ws::dispatcher_service;
use crate::utils::id::generate_id;

#[get("/login?<username>&<password>")]
pub async fn websocket_service(ws: rocket_ws::WebSocket,username:&str,password:&str) -> rocket_ws::Channel<'static> {
    tracing::info!("{} try to login",username);
    let info = UserLoginVo{
        username:username.to_string(),
        password: password.to_string()
    };
    let user = user_login_service(info.to_dto()).await;
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            match user {
                None => {
                    let _ = stream.send(Message::Text(WebSocketResponse::error(json!("login error")).to_json())).await;
                    return Ok(());
                }
                Some(user) => {
                    // 对stream进行读写分离
                    let (sender,mut receiver) = stream.split();
                    let sender = Arc::new(Mutex::new(sender));
                    // 获取用户id
                    let id = user.id.unwrap();
                    // 生成用户token
                    let token = general_purpose::STANDARD.encode(generate_id().as_binary());
                    // 将token返回
                    let _ = sender.lock().await.send(
                        Message::Text(
                            WebSocketResponse::success(
                                json!(UserLoginResponseVo { token: token.clone() }),
                            ).to_json()
                        )
                    ).await;
                    push_user_service(user, Arc::clone(&sender), token).await;
                    let mut shut_flag = false;
                    while let Some(message) = receiver.next().await {
                        match message {
                            Err(error) => {
                                tracing::error!("{}",format!("websocket error,msg:{:?}",error));
                                shut_flag = true;
                            }
                            Ok(message) => {
                                match message {
                                    Message::Text(text) => {
                                        // 进行路径分配处理
                                        let data = WebSocketDataVO::new(&text);
                                        match data {
                                            Ok(vo) => {
                                                let mes_id = &vo.request_id.clone();
                                                let mut result = dispatcher_service(vo.to_dto()).await;
                                                // 对result进行标识
                                                result.id = *mes_id;
                                                let _ = sender.lock().await.send(Message::Text(result.to_json())).await;
                                            }
                                            Err(error) => {
                                                tracing::error!("{}",format!("websocket error,msg:{:?}",error));
                                                shut_flag = true;
                                            }
                                        }
                                    }
                                    Message::Binary(_) => {}
                                    Message::Ping(_) => {}
                                    Message::Pong(_) => {}
                                    Message::Close(_) => {
                                        shut_flag = true;
                                    }
                                    Message::Frame(_) => {}
                                }
                            }
                        }
                        // 检查flag
                        if(shut_flag){
                            // 执行清理工作
                            remove_user_service(id).await;
                            break;
                        }
                    }
                }
            }
            Ok(())
        })
    })
}
