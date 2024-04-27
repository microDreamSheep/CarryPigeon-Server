use rocket::{form::Form, FromForm};

#[derive(Debug, FromForm)]
struct AuthInfo {
    uuid: u128,
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
#[tracing::instrument]
#[rocket::post("/authenticator", data = "<authinfo>")]
pub async fn post_authenticator(authinfo: Form<AuthInfo>) -> &'static str {
    // 构建uuid
    let id = sqlx::types::uuid::Builder::from_u128(authinfo.uuid).into_uuid();

    // 验证密码
    let matcher = crate::dao::user::get_password(id).await;
    if matcher == authinfo.password {
        if crate::dao::user::update_status(
            id,
            to_userstatus(crate::dao::row::Status::Online).await,
        )
        .await
        {
            "true"
        } else {
            "false"
        }
    } else {
        "false"
    }
}
