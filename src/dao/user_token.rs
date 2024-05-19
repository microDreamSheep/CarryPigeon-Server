use super::PG_POOL;
use super::row::UserToken;
pub async fn check_token(uuid:i64,token: String) -> bool{
    let rows_temp = match sqlx::query_as::<_,UserToken>("SELECT * FORM public.user_token WHERE uuid = $1")
        .bind(uuid)
        .fetch_all(PG_POOL.get().unwrap())
        .await
    { 
        Ok(v) => v,
        Err(e) =>{
            tracing::error!("{}",e);
            vec![]
        },
    };
    let mut result = false;
    todo!("rewrite:此处逻辑不正确 ，应为检查数据库中的token公钥，解密对比结果");
    for i in rows_temp.iter(){
        if i.token == token{
            result = true;
        }
    }
    result
}