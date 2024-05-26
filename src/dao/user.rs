use tracing::instrument;

use super::PG_POOL;

#[inline]
#[instrument]
pub async fn get_password(uuid: i64) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT * FROM public.user WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(PG_POOL.get().unwrap())
            .await;

    return match rows_temp {
        Ok(v) => v.password,
        Err(e) => {
            tracing::error!("{}", e);
            "".to_string()
        }
    };
}

#[inline]
#[instrument]
pub async fn get_status(uuid: i64) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT * FROM public.user WHERE uuid = $1")
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

#[inline]
#[instrument]
pub async fn get_username(uuid: i64) -> String {
    let rows_temp =
        sqlx::query_as::<_, super::row::User>("SELECT * FROM public.user WHERE uuid = $1")
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

#[inline]
#[instrument]
pub async fn update_status(uuid: i64, status: &str) -> bool {
    let rows_temp = sqlx::query("UPDATE public.user SET status = $1 WHERE uuid = $2")
        .bind(status)
        .bind(uuid)
        .execute(PG_POOL.get().unwrap())
        .await;

    match rows_temp {
        Ok(_) => true,
        Err(e) => {
            tracing::error!("Error updating user status to database : {}", e);
            false
        }
    }
}
