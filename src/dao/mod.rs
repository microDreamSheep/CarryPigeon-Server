/*!
dao层，用于提供对数据库最基本的操作，通过rbatis进行提供
dao层只能由repository层进行调用

### 示例
```rust
// 定义示例
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BizActivity {}
rbatis::crud!(BizActivity{});//crud = insert+select_by_column+update_by_column+delete_by_column
rbatis::impl_select_page!(BizActivity{select_page(name:&str) => "`where name != #{name}`"});
// 调用示例
async fn fun(){
    use carrypigeon_server::dao::MYSQL_POOL;
    let result = BizActivity::select_all(MYSQL_POOL.get().unwrap()).await;
}
```

 */

use rbatis::RBatis;
use rbdc_mysql::MysqlDriver;
use redis::aio::MultiplexedConnection;
use std::sync::OnceLock;

pub static MYSQL_POOL: OnceLock<RBatis> = OnceLock::new();
pub static REDIS_POOL: OnceLock<MultiplexedConnection> = OnceLock::new();

/**
初始化数据库连接，连接mysql和redis并生成连接池供调用
*/
#[inline]
pub async fn init_pool() {
    // mysql 连接
    let rb = RBatis::new();
    rb.init(MysqlDriver {}, "mysql://carrypigeon:carrypigeon@localhost:3306/carrypigeon?useSSL=false&allowPublicKeyRetrieval=true&serverTimezone=UTC").unwrap();
    MYSQL_POOL.set(rb).expect("mysql link error");
    // Redis连接
    match redis::Client::open("redis://localhost:6379/0") {
        Ok(v) => match v.get_multiplexed_async_connection().await {
            Ok(v) => match REDIS_POOL.set(v) {
                Ok(_) => {
                    tracing::info!("Redis is successfully linked");
                }
                Err(e) => {
                    tracing::error!("Database link failure:{:?}", e);
                }
            },
            Err(e) => {
                tracing::error!("Redis: {}", e);
            }
        },
        Err(e) => {
            tracing::error!("Redis: {}", e);
        }
    }
}

/**
账户相关dao操作
*/
pub mod account;
/**
消息相关操作
*/
pub mod message;
