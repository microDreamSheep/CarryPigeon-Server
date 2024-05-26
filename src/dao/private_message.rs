use sqlx::types::chrono::Utc;
use tracing::instrument;

use super::{row::GlobalMessage, PG_POOL};

#[instrument]
pub async fn get_line(
    from: i64,
    to: i64,
    timestamp: chrono::DateTime<Utc>,
    id: i64,
) -> GlobalMessage {
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(
        "SELECT * FORM public.private_message WHERE from = $1, to = $2, timestamp = $3, message_id = $4",
    )
    .bind(from)
    .bind(to)
    .bind(timestamp)
    .bind(id)
    .fetch_one(PG_POOL.get().unwrap())
    .await;
    match rows_temp {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e);
            GlobalMessage::default()
        }
    }
}

#[instrument]
pub async fn get_offline_message(uuid: i64) -> Vec<GlobalMessage> {
    let row_temp =
        sqlx::query_as::<_, GlobalMessage>("SELECT * FORM public.private_message WHERE to = $1")
            .bind(uuid)
            .fetch_all(PG_POOL.get().unwrap())
            .await;

    return match row_temp {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("{}", e);
            vec![]
        }
    };
}

#[instrument]
pub async fn get_latest_message_id(from: i64, to: i64) -> i64 {
    let rows_temp = sqlx::query_as::<_, GlobalMessage>(
        "SELECT MAX(message_id) message_id FROM public.private_message WHERE from = $1 , to = $2",
    )
    .bind(from)
    .bind(to)
    .fetch_one(PG_POOL.get().unwrap())
    .await;
    match rows_temp {
        Ok(v) => v.message_id,
        Err(e) => {
            tracing::error!("Missing from:{} to:{} or other error.Info:{}", from, to, e);
            // 表示查询失败
            -1
        }
    }
}

#[instrument]
pub async fn update_private_message(message: &GlobalMessage) {
    let rows_temp =
        sqlx::query("INSERT INTO public.private_message VALUES($1, $2, $3, $4, $5, $6, $7)")
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
