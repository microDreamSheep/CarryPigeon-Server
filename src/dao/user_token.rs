use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::repository::claims::RequiredClaims;

use super::row::UserToken;
use super::PG_POOL;

pub async fn push_token(uuid: i64, public_key: String) -> bool {
    let rows_temp = sqlx::query("INSERT INTO public.user_token VALUES($1 , $2)")
        .bind(uuid)
        .bind(public_key)
        .execute(PG_POOL.get().unwrap())
        .await;
    rows_temp.is_ok()
}

/// 获取该用户的所有token公钥
async fn get_all_token(uuid: i64) -> Vec<String> {
    let rows_temp =
        match sqlx::query_as::<_, UserToken>("SELECT * FORM public.user_token WHERE uuid = $1")
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
pub async fn check_token(uuid: i64, token: String) -> bool {
    let token_vec = get_all_token(uuid).await;

    // 匹配 token
    for i in token_vec {
        match decode::<RequiredClaims>(
            &token,
            &match DecodingKey::from_rsa_pem(i.as_bytes()) {
                Ok(v) => v,
                Err(e) => {
                    tracing::error!("{}", e);
                    return false;
                }
            },
            &Validation::new(Algorithm::RS256),
        ) {
            Ok(_v) => {
                return true;
            }
            Err(_) => {
                // 该PublicKey无法解密，但可以继续尝试其他PublicKey，故continue
                continue;
            }
        }
    }
    false
}
