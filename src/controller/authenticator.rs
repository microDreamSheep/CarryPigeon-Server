#[rocket::post("/authenticator/<uuid>/<password>")]
pub async fn post_authenticator(uuid: String, password: String) -> &'static str{
    let matcher = true;
    if matcher == true {
        "true"
    }else {
        "false"
    }
}