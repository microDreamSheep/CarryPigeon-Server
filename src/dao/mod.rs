use once_cell::sync::OnceCell;
use sqlx::{pool::PoolOptions, PgPool, Postgres};

pub static PG_POOL: OnceCell<PgPool> = OnceCell::new();

#[inline]
pub async fn make_pg_pool_connect() {
    match PoolOptions::<Postgres>::new()
        .max_connections(15)
        .connect("postgres://postgres:2006@localhost/carrypigeon")
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
    //.connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon").await
}

pub mod chat;
pub mod row;
pub mod user;
