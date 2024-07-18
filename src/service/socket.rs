use crate::controller::authenticator::to_user_status;
use crate::dao::row::{GlobalMessage, UserStatus, UserToken};
use crate::dao::user::update_status;
use rocket::futures::{SinkExt, StreamExt};
use serde_json::from_str;

use crate::dao::user_token::check_token;

use super::messages_service::{MessageService, SystemMessageService, WS_HASHMAP};

#[rocket::get("/socket")]
pub async fn websocket_service(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            // 最终可获得UserToken
            let info: Box<UserToken> = Box::new(match stream.next().await {
                Some(v) => match v {
                    Ok(v) => {
                        if v.is_close() {
                            return Ok(());
                        }
                        match from_str(v.to_text().unwrap()) {
                            Ok(v) => v,
                            Err(e) => {
                                tracing::error!("{}", e);
                                UserToken::default()
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("{}", e);
                        UserToken::default()
                    }
                },
                None => UserToken::default(),
            });

            let auth_result = Box::new(check_token(info.uuid, &info.public_key).await);
            if *auth_result {
                let _ = stream
                    .send(rocket_ws::Message::Text("Success".to_string()))
                    .await;
                // MessageService
                let service = Box::new(MessageService::new(info.uuid));
                // 接受离线信息
                service.receive_offline_message().await;

                socket_offline_message(info.uuid).await;
                update_status(info.uuid, &to_user_status(&UserStatus::Online).await).await;

                while let Some(message) = stream.next().await {
                    if message.is_err() {
                        drop_ws_hashmap(&info.uuid, service).await;
                        return Ok(());
                    }
                    // 获取message
                    let message = message.unwrap();
                    // 提前处理关闭连接信号和ping信号
                    if service.handle_close_message(&message).await {
                        drop_ws_hashmap(&info.uuid, service).await;
                        return Ok(());
                    } else if service.handle_ping_message(&message).await {
                        let _ = stream
                            .send(rocket_ws::Message::Pong(String::from("pong").into_bytes()))
                            .await;
                    }

                    // message_service
                    service.message_service(message.clone()).await;
                    let receive_message = Box::new(service.receive_message().await);

                    // 处理接收信息
                    match *receive_message {
                        Some(v) => {
                            let result = serde_json::to_string(&v).unwrap();
                            let _ = stream.send(rocket_ws::Message::Text(result)).await;
                        }
                        None => {
                            // Nothing to do
                        }
                    }
                }
                update_status(info.uuid, &to_user_status(&UserStatus::Offline).await).await;
                drop_ws_hashmap(&info.uuid, service).await;
                Ok(())
            } else {
                Ok(())
            }
        })
    })
}

async fn drop_ws_hashmap(uuid: &i64, service: Box<MessageService>) {
    drop(
        WS_HASHMAP
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .get(uuid)
            .unwrap()
            .to_owned(),
    );
    WS_HASHMAP.get().unwrap().lock().unwrap().remove(uuid);
    drop(service);
}

/// 获取离线时的信息
async fn socket_offline_message(uuid: i64) -> Vec<Vec<u8>> {
    let mut vec_messages_json = vec![];
    // private_message
    let messages = crate::dao::private_message::get_offline_message(uuid).await;

    for i in messages {
        let temp_chat_offline_message = GlobalMessage {
            from: i.from,
            to: i.to,
            text: i.text,
            file: i.file,
            json: i.json,
            timestamp: i.timestamp,
            message_id: i.message_id,
            aes_key: i.aes_key,
            aes_iv: i.aes_iv,
        };
        let temp_json = serde_json::to_vec(&temp_chat_offline_message).unwrap();
        vec_messages_json.push(temp_json);
    }
    vec_messages_json
}
