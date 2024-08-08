use rbatis::rbdc::Error;
use crate::dao::account::friend::Friend;
use crate::dao::MYSQL_POOL;

pub async fn get_friends_repository(
    user_id:&i64
) ->Vec<Friend>{
    let result = Friend::select_all_by_id(MYSQL_POOL.get().unwrap(),user_id).await;
    return match result {
        Ok(friends) => {
            friends
        }
        Err(e) => {
            // 输出错误日志
            tracing::error!("{}",e.to_string());
            vec![]
        }
    }
}