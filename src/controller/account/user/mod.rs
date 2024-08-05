use rocket::{form::Form, FromForm, get, post};
use rocket::async_stream::stream;
use rocket::response::content::RawJson;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_json_response::JSONResponse;
use tokio_test::block_on;
use crate::model::response::Response;
use crate::model::vo::account::user::{UserLoginVo, UserRegisterResponseVo, UserRegisterVo};
use crate::service::account::user::{is_user_name_contained, new_user, login};
use rocket::futures::{SinkExt, StreamExt};
use tracing::info;

/**
新建一个账户
数据传入格式：
```json
{
    "username":"",
    "password":""
}
返回值
```
 */
#[post("/register", data = "<info>")]
pub async fn user_register_controller(info:  Json<UserRegisterVo>) -> JSONResponse<'static, UserRegisterResponseVo> {
    let result = new_user(info.into_inner().to_dto()).await;
    match result {
        Ok(_) => Response::success(UserRegisterResponseVo::success()),
        Err(e) => Response::error(UserRegisterResponseVo::error(&e)),
    }
}

/**
 登录
 */
#[get("/login?<username>&<password>")]
pub async fn user_login_controller(ws: rocket_ws::WebSocket,username:&str,password:&str) -> rocket_ws::Channel<'static>{
    tracing::info!("{} try to login",username);
    let info = UserLoginVo{
        username:username.to_string(),
        password: password.to_string()
    };
    ws.channel(move |mut stream| {
        Box::pin(async move {
            login(info.to_dto(), stream).await;
            Ok(())
        })
    })
}
