use tokio::sync::Mutex;
use rocket::futures::{SinkExt, StreamExt};
use std::sync::{Arc};
use tokio::io;
use crate::dao::account::user::User;
use crate::service::account::user::{login, push_user, remove_user};
use crate::model::vo::ws::{UserLoginVo, WebSocketDataVO};
use rocket::get;
use rocket_ws::Message;
use rocket_ws::result::Error;
use tokio_test::block_on;
use crate::service::ws::dispatcher;

#[get("/login?<username>&<password>")]
pub async fn websocket_service(ws: rocket_ws::WebSocket,username:&str,password:&str) -> rocket_ws::Channel<'static> {
    tracing::info!("{} try to login",username);
    let info = UserLoginVo{
        username:username.to_string(),
        password: password.to_string()
    };
    let user = login(info.to_dto()).await;
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            match user {
                None => {
                    let _ = stream
                        .send(Message::Text("login error".to_string()))
                        .await;
                    return Ok(());
                }
                Some(user) => {
                    // 对stream进行读写分离
                    let (sender,mut receiver) = stream.split();
                    let id = user.id.unwrap();
                    let sender = Arc::new(Mutex::new(sender));
                    let mut shut_flag = false;
                    push_user(user, Arc::clone(&sender)).await;
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
                                                let mut result = dispatcher(vo.to_dto()).await;
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
                            remove_user(id).await;
                            break;
                        }
                    }
                }
            }
            Ok(())
        })
    })
}
