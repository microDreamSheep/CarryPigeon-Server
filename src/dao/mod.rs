use std::fmt::{Debug, Formatter};
use redis::aio::MultiplexedConnection;
use std::sync::OnceLock;
use rbatis::{RBatis, rbdc};
use rbatis::rbdc::db::{Connection, ConnectOptions};
use rbatis::rbdc::Error;
use rbdc_mysql::MysqlDriver;
use rocket::futures::future::BoxFuture;

pub static MYSQL_POOL: OnceLock<RBatis> = OnceLock::new();
pub static mut REDIS_POOL: OnceLock<MultiplexedConnection> = OnceLock::new();

#[inline]
pub async fn init_pool() {
    // mysql 连接
    let rb = RBatis::new();
    rb.init(MysqlDriver {}, "mysql://localhost:3306/carrypigeon?useSSL=false&allowPublicKeyRetrieval=true&serverTimezone=UTC").unwrap();
    MYSQL_POOL.set(rb).expect("mysql link error");
    // Redis连接
    match redis::Client::open("redis://localhost:6379/0") {
        Ok(v) => match v.get_multiplexed_async_connection().await {
            Ok(v) => match unsafe { REDIS_POOL.set(v) } {
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
