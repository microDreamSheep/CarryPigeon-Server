use chrono::{Duration, Utc};
use rocket::{form::Form, FromForm};

use crate::{dao::user::get_username, repository::jwt::authenticator_encrypt};

#[derive(Debug, FromForm)]
struct AuthInfo {
    uuid: i64,
    password: String,
}
#[inline]
async fn to_userstatus(matcher: crate::dao::row::Status) -> String {
    match matcher {
        crate::dao::row::Status::Online => "Online".to_string(),
        crate::dao::row::Status::Offline => "Offline".to_string(),
    }
}

#[allow(private_interfaces)]
#[rocket::post("/authenticator", data = "<info>")]
pub async fn post_authenticator(info: Form<AuthInfo>) -> String {
    // 验证密码
    let matcher = crate::dao::user::get_password(info.uuid).await;
    if matcher == info.password {
        if crate::dao::user::update_status(
            info.uuid,
            to_userstatus(crate::dao::row::Status::Online).await,
        )
        .await
        {
            let iat = Utc::now();
            let exp = iat + Duration::days(72);
            authenticator_encrypt(
                get_username(info.uuid).await,
                iat.timestamp(),
                exp.timestamp(),
            )
            .await
        } else {
            String::from("false")
        }
    } else {
        String::from("false")
    }
}
