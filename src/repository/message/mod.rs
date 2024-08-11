use crate::dao::message::Message;
use crate::dao::MYSQL_POOL;
use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;

pub async fn push_message_repository(message: &Message) -> Result<ExecResult, Error> {
    Message::insert(MYSQL_POOL.get().unwrap(), message).await
}
