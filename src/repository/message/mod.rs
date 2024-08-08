use rbatis::rbdc::{Error};
use rbatis::rbdc::db::ExecResult;
use crate::dao::message::Message;
use crate::dao::MYSQL_POOL;

pub async fn push_message_repository(
    message: &Message
) -> Result<ExecResult, Error> {
    Message::insert(MYSQL_POOL.get().unwrap(),message).await
}