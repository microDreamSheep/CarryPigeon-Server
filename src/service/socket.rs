use std::sync::mpsc::channel;

use crate::controller::authenticator::to_user_status;
use crate::dao::row::{GlobalMessage, MPSCMessage, UserStatus, UserToken};
use crate::dao::user::update_status;
use rocket::futures::{SinkExt, StreamExt};
use serde_json::from_str;

use crate::dao::user_token::check_token;

use super::messages_service::{MessageService, WS_HASHMAP};

#[rocket::get("/socket")]
pub async fn websocket_service(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            // 最终可获得UserToken
            let info: UserToken = match stream.next().await {
                Some(v) => match v {
                    Ok(v) => match from_str(v.to_text().unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::error!("{}", e);
                            UserToken::default()
                        }
                    },
                    Err(e) => {
                        tracing::error!("{}", e);
                        UserToken::default()
                    }
                },
                None => UserToken::default(),
            };

            let auth_result = check_token(info.uuid, info.public_key).await;
            if auth_result {
                let _ = stream
                    .send(rocket_ws::Message::Text("Success".to_string()))
                    .await;
                // MessageService
                let service = MessageService::new();
                let (tx, rx) = channel::<MPSCMessage>();
                WS_HASHMAP
                    .get()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .insert(info.uuid, (tx, rx));

                socket_offline_message(info.uuid).await;
                update_status(info.uuid, to_user_status(&UserStatus::Online).await).await;

                while let Some(message) = stream.next().await {
                    service.message_service(message?).await;
                    if WS_HASHMAP
                        .get()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .get(&info.uuid)
                        .unwrap()
                        .1
                        .try_recv()
                        .is_ok()
                    {
                        let text = serde_json::to_string(
                            &WS_HASHMAP
                                .get()
                                .unwrap()
                                .lock()
                                .unwrap()
                                .get(&info.uuid)
                                .unwrap()
                                .1
                                .try_recv()
                                .unwrap()
                                .clone(),
                        )
                        .unwrap();
                        let _ = stream.send(rocket_ws::Message::Text(text)).await;
                    }
                }
                update_status(info.uuid, to_user_status(&UserStatus::Offline).await).await;
                drop(
                    WS_HASHMAP
                        .get()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .get(&info.uuid)
                        .unwrap()
                        .0
                        .to_owned(),
                );
                WS_HASHMAP.get().unwrap().lock().unwrap().remove(&info.uuid);
                Ok(())
            } else {
                Ok(())
            }
        })
    })
}

/// 获取离线时的信息
async fn socket_offline_message(uuid: i64) -> Vec<Vec<u8>> {
    let messages = crate::dao::private_message::get_offline_message(uuid).await;
    let mut vec_messages_json = vec![];

    for i in messages {
        let temp_chat_offline_message = GlobalMessage {
            from: i.from,
            to: i.to,
            text: i.text,
            file: i.file,
            json: i.json,
            timestamp: i.timestamp,
            message_id: i.message_id,
        };
        let temp_json = serde_json::to_vec(&temp_chat_offline_message).unwrap();
        vec_messages_json.push(temp_json);
    }
    vec_messages_json
}
