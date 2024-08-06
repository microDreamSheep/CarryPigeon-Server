use rbatis::rbdc::{DateTime, Error};
use rbatis::rbdc::db::ExecResult;
use crate::dao::message::Message;
use crate::dao::MYSQL_POOL;
use crate::utils::id::generate_id;

pub async fn push_tree_hole_message_repository(
    from_id:i64,
    data:String
) -> Result<ExecResult, Error> {
    let msg = Message{
        id: Some(generate_id()),
        from_id: Some(from_id),
        to_id: Some(-1),
        message_tag: Some(3),
        data: Some(data),
        message_type: Some(0),
        time: Some(DateTime::now()),
    };
    let result = Message::insert(
        MYSQL_POOL.get().unwrap(),
        &msg
    ).await;
    return result;
}