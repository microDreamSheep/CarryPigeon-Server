use rocket::data::ToByteUnit;
use rocket::form::Form;
use rocket::futures::{SinkExt, StreamExt};
use rocket::FromForm;

#[derive(FromForm)]
pub struct UUID {
    uuid: i64,
}

#[rocket::get("/socket")]
pub async fn websocket_service(ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
    let matcher = true;

    if matcher {
        // 创建到client的websocket连接
        ws.channel(move |mut stream| {
            Box::pin(async move {
                while let Some(message) = stream.next().await {
                    let _ = stream.send(message?).await;
                }
                Ok(())
            })
        })
    } else {
        ws.channel(move |_| Box::pin(async move { Err(rocket_ws::result::Error::AttackAttempt) }))
    }
}

#[allow(unused_variables)]
#[rocket::post("/send_offline_message", data = "<uuid>")]
pub async fn socket_offline_message(
    uuid: Form<UUID>,
    ws: rocket_ws::WebSocket,
) -> rocket_ws::Stream!['static] {
    let ws = ws.config(rocket_ws::Config {
        max_message_size: Some(5.mebibytes().as_u64() as usize),
        max_write_buffer_size: 5_usize,
        max_frame_size: Some(1.mebibytes().as_u64() as usize),
        ..Default::default()
    });

    let messages = crate::dao::chat::get_offline_message(uuid.uuid).await;

    rocket_ws::Stream! { ws =>
        for result in messages {
            yield result.text.into();
            yield result.file_path.into();
            yield result.json.to_string().into();
            yield result.timestamp.to_rfc2822().into();
        }
    }
}

//#[rocket::post("/")]
