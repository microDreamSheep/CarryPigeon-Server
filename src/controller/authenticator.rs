use rocket::{form::Form, FromForm};

#[derive(Debug, FromForm)]
struct AuthInfo {
    uuid: u128,
    password: String,
}

#[allow(private_interfaces)]
#[tracing::instrument]
#[rocket::post("/authenticator", data = "<authinfo>")]
pub async fn post_authenticator(authinfo: Form<AuthInfo>) -> &'static str {
    // 构建uuid
    let id = sqlx::types::uuid::Builder::from_u128(authinfo.uuid).into_uuid();

    // 验证密码
    let matcher =
        crate::dao::authenticator::match_password(id, authinfo.password.to_string()).await;
    if matcher {
        "true"
    } else {
        "false"
    }
}
