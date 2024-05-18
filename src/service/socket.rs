use rocket::form::Form;
use rocket::futures::{SinkExt, StreamExt};
use rocket::FromForm;

#[derive(FromForm)]
pub struct GetOfflineMessage {
    uuid: i64,
    token: String,
}

#[rocket::get("/socket")]
pub async fn websocket_service(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    // 创建到client的websocket连接
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(message?).await;
            }
            Ok(())
        })
    })
}

#[rocket::post("/send_offline_message", data = "<info>")]
pub async fn socket_offline_message(
    info: Form<GetOfflineMessage>,
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

//#[rocket::post("/")]
