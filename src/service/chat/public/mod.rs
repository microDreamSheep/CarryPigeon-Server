use crate::dao::message::Message;
use crate::manager::ws::WebSocketManager;
use crate::model::protocol::ws::response::WebSocketResponse;
use crate::model::vo::chat::MessageUpdateNotification;
use crate::repository::message::push_message_repository;
use rocket::serde::json::serde_json::json;

/**
将消息放入数据库并对所有相关的在线用户发送消息进行消息发送提醒
 */
pub(crate) async fn push_message_and_notice_all_service(
    message: Message,
    ids: Vec<i64>,
) -> Result<(), String> {
    if let Err(e) = push_message_repository(&message).await {
        return Err(e.to_string());
    }
    for id in ids {
        match WebSocketManager::get_sender(&id).await {
            None => continue,
            Some(sender) => {
                sender
                    .lock()
                    .await
                    .send_ws_data(WebSocketResponse::send(
                        json!(MessageUpdateNotification {
                            message_id: message.id.unwrap()
                        }),
                        "/message_update_notification".to_string(),
                    ))
                    .await;
            }
        }
    }
    Ok(())
}
