use redis::aio::MultiplexedConnection;
use sqlx::{pool::PoolOptions, PgPool, Postgres};
use std::sync::OnceLock;

pub static PG_POOL: OnceLock<PgPool> = OnceLock::new();
pub static mut REDIS_POOL: OnceLock<MultiplexedConnection> = OnceLock::new();

#[inline]
pub async fn init_pool() {
    // PostgresSQL连接
    match PoolOptions::<Postgres>::new()
        .max_connections(15)
        .connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon")
        .await
    {
        Ok(v) => match PG_POOL.set(v) {
            Ok(_) => {
                tracing::info!("PostgresSQL is successfully linked");
            }
            Err(e) => {
                tracing::error!("Database link failure:{:?}", e);
            }
        },
        Err(e) => {
            tracing::error!("PostgresSQL: {}", e);
        }
    }
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

pub mod group;
pub mod group_message;
pub mod private;
pub mod private_message;
pub mod row;
pub mod user;
pub mod user_token;
