use crate::model::protocol::http::response::HttpResponse;
use crate::model::vo::tree_hole::{TreeHoleSendResponseVO, TreeHoleSendVO};
use crate::service::chat::tree_hole::tree_hole_send_service;
use rocket::post;
use rocket::serde::json::Json;
use rocket_json_response::JSONResponse;

/**
发送树洞消息

请求格式
```json
{
    "user_id:,
    "data":"",
    "token":""
}
```
 */
#[post("/send", data = "<info>")]
pub async fn tree_hole_send_controller(
    info: Json<TreeHoleSendVO>,
) -> JSONResponse<'static, TreeHoleSendResponseVO> {
    let result = tree_hole_send_service(info.0.token.clone(), info.0.to_dto()).await;
    return match result {
        Ok(_) => HttpResponse::success(TreeHoleSendResponseVO::success()),
        Err(e) => HttpResponse::error(TreeHoleSendResponseVO::error(e)),
    };
}
