#[test]
fn test_get_member(){
    tokio_test::block_on(impl_test_get_member());
}

async fn impl_test_get_member(){
    use crate::dao::{init_pool,group::get_member};
    init_pool().await;
    let a = get_member(0).await;
    let c: Vec<i64> = Vec::new();
    assert_eq!(a, c);
}