use crate::dao::row::UserToken;
use rocket::form::Form;
use rocket::futures::{SinkExt, StreamExt};
use serde_json::from_str;

use crate::dao::user_token::check_token;

#[rocket::get("/socket")]
pub async fn websocket_service(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            // 提取信息
            /*let info: UserToken = from_str(
                stream
                    .next()
                    .await
                    .ok_or("")
                    .unwrap()
                    .unwrap()
                    .to_text()
                    .unwrap(),
            )
            .unwrap();*/
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
                while let Some(message) = stream.next().await {
                    let _ = stream.send(message?).await;
                }
                Ok(())
            } else {
                Ok(())
            }
        })
    })
}

#[rocket::post("/send_offline_message", data = "<info>")]
pub async fn socket_offline_message(
    info: Form<UserToken>,
    _ws: rocket_ws::WebSocket,
) -> rocket_ws::Stream!['static] {
    //todo!("验证Token");

    let messages = crate::dao::chat::get_offline_message(info.uuid).await;

    rocket_ws::Stream! { _ws =>
        for result in messages {
            yield result.text.into();
            yield result.file_path.into();
            yield result.json.to_string().into();
            yield result.timestamp.to_rfc2822().into();
        }
    }
}
