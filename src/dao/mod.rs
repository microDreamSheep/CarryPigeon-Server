use once_cell::sync::OnceCell;
use sqlx::{PgPool, Pool, Postgres};

pub static PG_POOL: OnceCell<PgPool> = OnceCell::new();

#[inline]
pub async fn make_pg_pool_connect() {
    match Pool::<Postgres>::connect("postgres://postgres:2006@localhost/carrypigeon").await {
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

pub mod authenticator;
pub mod row;
