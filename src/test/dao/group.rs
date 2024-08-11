use crate::dao::init_pool;
use crate::repository::account::group::get_group_members_repository;

#[test]
fn test_get_user() {
    tokio_test::block_on(impl_test_get_member());
}

async fn impl_test_get_member() {
    init_pool().await;
    get_group_members_repository(&3515).await;
}
