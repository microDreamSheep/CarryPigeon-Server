#[cfg(test)]
mod group;

#[test]
fn test_init_pool() {
    tokio_test::block_on(impl_init_pool());
}

async fn impl_init_pool() {
    use crate::dao::init_pool;
    init_pool().await;
}
