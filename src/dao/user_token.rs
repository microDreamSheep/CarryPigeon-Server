use chrono::Utc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use tracing::instrument;

use super::row::UserToken;
use super::PG_POOL;

#[instrument]
pub async fn push_token(uuid: i64, public_key: &String) -> bool {
    let rows_temp = sqlx::query("INSERT INTO public.user_token (uuid,public_key) VALUES($1 , $2)")
        .bind(uuid)
        .bind(public_key)
        .execute(PG_POOL.get().unwrap())
        .await;
    rows_temp.is_ok()
}

/// 获取该用户的所有token公钥
#[instrument]
async fn get_all_token(uuid: i64) -> Vec<String> {
    let rows_temp =
        match sqlx::query_as::<_, UserToken>("SELECT * FROM public.user_token WHERE uuid = $1")
            .bind(uuid)
            .fetch_all(PG_POOL.get().unwrap())
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("{}", e);
                vec![]
            }
        };
    let mut result: Vec<String> = vec![];
    for i in rows_temp.iter() {
        result.push(i.public_key.to_owned());
    }
    result
}

/// 匹配并验证token
#[instrument]
pub async fn check_token<'a>(uuid: i64, token: &'a String) -> bool {
    false
}
