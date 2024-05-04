use rocket::{form::Form, FromForm};

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
#[rocket::post("/authenticator", data = "<authinfo>")]
pub async fn post_authenticator(authinfo: Form<AuthInfo>) -> &'static str {
    // 验证密码
    let matcher = crate::dao::user::get_password(authinfo.uuid).await;
    let result;
    if matcher == authinfo.password {
        if crate::dao::user::update_status(
            authinfo.uuid,
            to_userstatus(crate::dao::row::Status::Online).await,
        )
        .await
        {
            result = "true";
        } else {
            result = "false";
        }
    } else {
        result = "false";
    }
    return result;
}
