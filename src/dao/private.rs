use super::PG_POOL;

/// 新建用户私信表
pub async fn new_private_message_table(id: i64) {
    // 创建表
    let sql = format!(
        r#"create table private_message.{}
(
    "from"     bigint  not null,
    "to"       bigint  not null,
    text       text,
    file_path  text,
    json       json,
    timestamp  varchar not null,
    message_id bigint
);"#,
        id
    );
    let rows_temp = sqlx::query(&sql)
    .execute(PG_POOL.get().unwrap())
    .await;
    match rows_temp {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
}

/// 删除用户私信表
/// 注意：如果不是销号清理数据的地方不能使用
pub async fn drop_private_message_table(id:i64){
    let sql = format!(r#"drop table private_message.{}"#,id);
    let rows_temp = sqlx::query(&sql)
    .execute(PG_POOL.get().unwrap())
    .await;
    match rows_temp {
        Ok(_) => (),
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
}
