#[test]
fn test_connect_pgsql_and_feedback() {
    tokio_test::block_on(impl_test_connect_pgsql_and_feedback());
}

async fn impl_test_connect_pgsql_and_feedback() {}
