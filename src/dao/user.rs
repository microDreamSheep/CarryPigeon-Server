use tracing::instrument;

use super::PG_POOL;

#[instrument]
pub async fn get_password(uuid: sqlx::types::Uuid) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT password FROM user WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    match rows_temp {
        Ok(v) => v.password,
        Err(e) => {
            tracing::error!("{}", e);
            return "".to_string();
        }
    }
}

#[instrument]
pub async fn get_status(uuid: sqlx::types::Uuid) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT status FROM user WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;
    match rows_temp {
        Ok(v) => v.status,
        Err(e) => {
            tracing::error!("{}", e);
            return "".to_string();
        }
    }
}

#[instrument]
pub async fn get_username(uuid: sqlx::types::Uuid) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT username FROM user WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;
    match rows_temp {
        Ok(v) => v.status,
        Err(e) => {
            tracing::error!("{}", e);
            return "".to_string();
        }
    }
}

#[instrument]
pub async fn update_status(uuid: sqlx::types::Uuid, status: String) -> bool {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("UPDATE user SET status = $1 uuid WHERE uuid = $2")
            .bind(status)
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    match rows_temp {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("Error updating user status to database : {}", e);
            false
        }
    }
}
