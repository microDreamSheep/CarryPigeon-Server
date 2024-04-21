#[rocket::post("/authenticator/<uuid>/<password>")]
pub async fn post_authenticator(uuid: u128, password: String) -> &'static str {
    // 构建uuid
    let id = sqlx::types::uuid::Builder::from_u128(uuid).into_uuid();

    // 验证密码
    let matcher = crate::dao::authenticator::match_password(id, password).await;
    if matcher {
        "true"
    } else {
        "false"
    }
}
