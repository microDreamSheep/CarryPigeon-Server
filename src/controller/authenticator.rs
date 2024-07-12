use crate::dao::row::UserStatus;
use crate::repository::jwt::authenticator_encrypt;
use chrono::{Duration, Utc};
use rocket::{form::Form, FromForm};

#[derive(Debug, FromForm)]
pub struct AuthInfo {
    uuid: i64,
    password: String,
}
#[inline]
pub async fn to_user_status(matcher: &crate::dao::row::UserStatus) -> String {
    match matcher {
        crate::dao::row::UserStatus::Online => String::from("Online"),
        crate::dao::row::UserStatus::Offline => String::from("Offline"),
    }
}

#[rocket::post("/authenticator", data = "<info>")]
pub async fn post_authenticator(info: Form<AuthInfo>) -> String {
    // 验证密码
    let matcher = crate::dao::user::get_password(info.uuid).await;
    if matcher == info.password {
        if crate::dao::user::update_status(info.uuid, &to_user_status(&UserStatus::Online).await)
            .await
        {
            let iat = Utc::now();
            let exp = iat + Duration::days(72);
            authenticator_encrypt(info.uuid, iat.timestamp(), exp.timestamp()).await
        } else {
            String::from("false")
        }
    } else {
        String::from("false")
    }
}
