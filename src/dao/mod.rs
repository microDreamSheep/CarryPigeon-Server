use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

async_static::async_static! {
    pub static ref PG_POOL_PTR :Result<Pool<Postgres>,sqlx::Error> = PgPoolOptions::new()
     .max_connections(5)
     //.connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon").await;
     .connect("postgres://postgres:2006@localhost/carrypigeon").await;
}

pub mod authenticator;
pub mod row;
