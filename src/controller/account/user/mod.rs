use rocket::{form::Form, post};
use rocket_json_response::JSONResponse;
use tokio_test::block_on;
use crate::dao::{row::CreateAccountRequest, user::push_user};
use crate::dao::user::get_username;
use crate::model::response::Response;

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
pub async fn user_register(info: Form<CreateAccountRequest>) -> /*JSONResponse<'static, String>*/String {
/*    let result = push_user(&info).await;
    match result {
        Some(v) => Response::success(v.to_string()),
        None => Response::error( "".to_string()),
    }*/
    let a =get_username(5).await;
    println!("{}",a.username);
    "hello".to_string()
}