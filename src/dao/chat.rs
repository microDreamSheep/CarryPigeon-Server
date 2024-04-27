use sqlx::types::chrono::Utc;
use tracing::instrument;

use super::PG_POOL;

#[instrument]
pub async fn get_line(
    from: sqlx::types::Uuid,
    to: sqlx::types::Uuid,
    timestamp: sqlx::types::chrono::DateTime<Utc>,
    id: i64,
) {
    let rows_temp = sqlx::query_as::<_, super::row::ChatPostTable>(
        "SELECT * FORM private_temp_message WHERE form = $1, to = $2, timestamp = $3, id = $4",
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
            super::row::ChatPostTable {
                from,
                to,
                text: "".to_string(),
                file_path: "".to_string(),
                json: serde_json::from_str(" ").unwrap(),
                timestamp: chrono::Utc::now(),
                id,
            }
        }
    };
}
