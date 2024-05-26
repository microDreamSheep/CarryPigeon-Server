use redis::Client;
use sqlx::{pool::PoolOptions, PgPool, Postgres};
use std::sync::OnceLock;

pub static PG_POOL: OnceLock<PgPool> = OnceLock::new();
pub static REDIS_POOL: OnceLock<Client> = OnceLock::new();

#[inline]
pub async fn make_pg_pool_connect() {
    // PostgreSQL连接
    match PoolOptions::<Postgres>::new()
        .max_connections(15)
        .connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon")
        //.connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon")
        .await
    {
        Ok(v) => match PG_POOL.set(v) {
            Ok(_) => {
                tracing::info!("Successfully linked PostgreSQL");
            }
            Err(e) => {
                tracing::error!("{:?}", e);
            }
        },
        Err(e) => {
            tracing::error!("{}", e);
        }
    }
}

pub mod group_message;
pub mod private_message;
pub mod row;
pub mod user;
pub mod user_token;
