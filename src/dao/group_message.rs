use super::{row::GlobalMessage, PG_POOL};

pub async fn get_latest_message_id(group_id: i64) -> Option<i64> {
    let sql = format!(
        r#"SELECT MAX(message_id) message_id FROM "group".group_{}"#,
        group_id
    );
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(&sql)
        .bind(group_id)
        .fetch_one(PG_POOL.get().unwrap())
        .await;
    match rows_temp {
        Ok(v) => Some(v.message_id),
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            None
        }
    }
}

pub async fn push_group_message(message: &GlobalMessage) {
    let sql = format!(
        r#"INSERT INTO "group".group_{} ("from", group_id, text, file_path, json, timestamp, message_id) VALUES($1, $2, $3, $4, $5, $6, $7)"#,
        message.to
    );
    let rows_temp = sqlx::query(&sql)
        .bind(message.from)
        .bind(message.to)
        .bind(message.text.clone())
        .bind(message.file.clone())
        .bind(message.json.clone())
        .bind(message.timestamp.clone())
        .bind(message.message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
    match rows_temp {
        Ok(_) => {}
        Err(e) => tracing::error!("{}", e),
    }
}

pub async fn delete_message(group_id: i64, message_id: i64) {
    let sql = format!(r#"DELETE "group".group_{} WHERE $1"#, group_id);
    let _rows_temp = sqlx::query(&sql)
        .bind(message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
}
