#[test]
fn test_connect_pgsql_and_feedback() {
    tokio_test::block_on(impl_test_connect_pgsql_and_feedback());
}

async fn impl_test_connect_pgsql_and_feedback() {}

#[test]
fn test_push_group_message() {
    tokio_test::block_on(impl_push_group_message());
}

async fn impl_push_group_message() {
    crate::dao::init_pg_pool().await;
    let message = crate::dao::row::GlobalMessage {
        from: 1,
        to: 1,
        text: "hello".to_string(),
        file: String::new(),
        json: String::new(),
        timestamp: String::from("fe"),
        message_id: 589,
    };
    crate::dao::group_message::push_group_message(&message).await;
}

#[test]
fn test_push_new_group() {
    tokio_test::block_on(impl_push_new_group());
}

async fn impl_push_new_group(){
    crate::dao::init_pg_pool().await;
    let temp = crate::dao::row::Group{
        id: 0,
        name: String::from("test_group"),
        admin:vec![0],
        owner:0,
        member:vec![0],
    };
    crate::dao::group::push_new_group(&temp).await;
}