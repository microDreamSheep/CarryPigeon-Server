#[test]
fn test_connect_pgsql_and_feedback() {
    tokio_test::block_on(impl_test_connect_pgsql_and_feedback());
}

async fn impl_test_connect_pgsql_and_feedback() {}

#[test]
fn test_update_group_message() {
    tokio_test::block_on(impl_update_group_message());
}

async fn impl_update_group_message() {
    crate::dao::make_pg_pool_connect().await;
    let message = crate::dao::row::GlobalMessage {
        message_type: 0,
        from: 1,
        to: 1,
        text: "hello".to_string(),
        file: String::new(),
        json: String::new(),
        timestamp: String::from("fe"),
        message_id: 589,
    };
    crate::dao::group_message::update_group_message(&message).await;
}
