use tracing::instrument;

use super::{row::GlobalMessage, PG_POOL};

#[instrument]
pub async fn get_latest_message_id(group_id: i64) -> i64 {
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(
        "SELECT MAX(message_id) message_id FROM public.group_message WHERE group_id = $1",
    )
    .bind(group_id)
    .fetch_one(PG_POOL.get().unwrap())
    .await;
    match rows_temp {
        Ok(v) => v.message_id,
        Err(e) => {
            tracing::error!("Missing group_id:{} or other error.Info:{}", group_id, e);
            // 表示查询失败
            -1
        }
    }
}

#[instrument]
pub async fn push_group_message(message: &GlobalMessage) {
    let rows_temp =
        sqlx::query(r#"INSERT INTO public.group_message ("from", group_id, text, file_path, json, timestamp, message_id) VALUES($1, $2, $3, $4, $5, $6, $7)"#)
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

#[instrument]
pub async fn delete_message(message_id: i64){
    let _rows_temp = 
    sqlx::query(r#"DELETE public.group_message WHERE $1"#)
        .bind(message_id)
        .execute(PG_POOL.get().unwrap())
        .await;
}
