use crate::model::protocol::http::response::HttpResponse;
use rocket::{post};
use rocket::serde::json::Json;
use rocket_json_response::JSONResponse;
use crate::model::vo::account::user::{UserRegisterResponseVo, UserRegisterVo};
use crate::service::account::user::{user_register_service};

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
    let result = user_register_service(info.into_inner().to_dto()).await;
    match result {
        Ok(_) => HttpResponse::success(UserRegisterResponseVo::success()),
        Err(e) => HttpResponse::error(UserRegisterResponseVo::error(&e)),
    }
}