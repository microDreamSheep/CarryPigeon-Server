use rocket::{form::Form, post};

use crate::dao::{row::CreateAccountRequest, user::push_user};

#[post("/new_account", data = "<info>")]
pub async fn new_account(info: Form<CreateAccountRequest>) -> String {
    let result = push_user(&info).await;
    match result {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}
