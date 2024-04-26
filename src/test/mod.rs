use crate::dao::PG_POOL;

#[test]
fn test_connectpgsql_and_feedback() {
    tokio_test::block_on(impl_test_connectpgsql_and_feedback());
}

async fn impl_test_connectpgsql_and_feedback() {
    let _row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(PG_POOL.get().unwrap())
        .await
        .unwrap();
}
