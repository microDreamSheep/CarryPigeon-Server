use rbatis::rbdc::{DateTime, Error};
use rbatis::rbdc::db::ExecResult;
use crate::dao::account::user::User;
use crate::dao::{init_pool, MYSQL_POOL};
use crate::utils::id::generate_id;

#[test]
fn test_get_user() {
    tokio_test::block_on(impl_test_get_member());
}

async fn impl_test_get_member() {
    init_pool().await;
    let user = User {
        id: Some(generate_id()),
        username: Some("awdawdwadad".to_string()),
        password: Some("awdawdawd".to_string()),
        data: Some("{}".to_string()),
        register_time: Some(DateTime::now()),
    };
    let result = User::select_all(MYSQL_POOL.get().unwrap()).await;
    match result {
        Ok(users) => {
            print!("{:?}",users)
        }
        Err(r) => {
            println!("{}",r.to_string())
        }
    }
}
