#[test]
fn test_connectpgsql_and_feedback() {
    tokio_test::block_on(impl_test_connectpgsql_and_feedback());
}

async fn impl_test_connectpgsql_and_feedback() {
    match crate::dao::PG_POOL_PTR.await {
        Ok(v) => {
            let _row: (i64,) = sqlx::query_as("SELECT $1")
                .bind(150_i64)
                .fetch_one(v)
                .await
                .unwrap();
        }
        Err(_e) => {}
    }
}
