use std::sync::Arc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::instrument;

struct PgPool{
    pool_ptr :Arc<Pool<Postgres>>,
}

impl PgPool {
    #[instrument]
    async fn new(&mut self) -> Result<(),sqlx::Error>{
        // 连接池初始化
        self.pool_ptr = Arc::from(PgPoolOptions::new()
            .max_connections(5)
            .connect("postgres://carrypigeon:carrypigeon@localhost/carrypigeon")
            .await?);

        //TODO

        Ok(())
    }
}