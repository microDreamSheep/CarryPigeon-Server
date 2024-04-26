use tracing::instrument;

use super::PG_POOL;

#[instrument]
pub async fn match_password(uuid: sqlx::types::Uuid, password: String) -> bool {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT password FROM user WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    match rows_temp {
        Ok(v) => {
            if v.password == password {
                return true;
            }
            false
        }
        Err(e) => {
            tracing::error!("{}", e);
            false
        }
    }
}
