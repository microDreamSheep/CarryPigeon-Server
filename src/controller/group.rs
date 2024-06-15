use rocket::{form::Form, get};

use crate::dao::{group::push_new_group, row::Group};

#[get("/new_group", data = "<info>")]
pub async fn new_group(info: Form<Group>) -> String {
    let id = Box::from(push_new_group(&info).await);
    id.to_string()
}
