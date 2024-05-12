use crate::dao::row::ChatOfflineMessage;
use sqlx::types::chrono::Utc;
use tracing::instrument;

use super::PG_POOL;

#[inline]
#[instrument]
pub async fn get_line(
    from: i64,
    to: i64,
    timestamp: sqlx::types::chrono::DateTime<Utc>,
    id: i64,
) -> ChatOfflineMessage {
    let rows_temp = sqlx::query_as::<_, super::row::ChatOfflineMessage>(
        "SELECT * FORM public.private_temp_message WHERE from = $1, to = $2, timestamp = $3, id = $4",
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
            ChatOfflineMessage {
                from,
                to,
                text: String::new(),
                file_path: String::new(),
                json: serde_json::from_str(" ").unwrap(),
                timestamp: chrono::Utc::now(),
                id,
            }
        }
    }
}

#[inline]
#[instrument]
pub async fn get_offline_message(uuid: i64) -> Vec<ChatOfflineMessage> {
    let row_temp = sqlx::query_as::<_, ChatOfflineMessage>(
        "SELECT * FORM public.private_temp_message WHERE to = $1",
    )
    .bind(uuid)
    .fetch_all(PG_POOL.get().unwrap())
    .await;

    match row_temp {
        Ok(v) => {
            return v;
        }
        Err(e) => {
            tracing::error!("{}", e);
            return vec![];
        }
    }
}
