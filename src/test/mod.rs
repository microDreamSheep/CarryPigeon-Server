use chrono::Utc;

use crate::dao::PG_POOL;

#[test]
fn test_connect_pgsql_and_feedback() {
    tokio_test::block_on(impl_test_connect_pgsql_and_feedback());
}

async fn impl_test_connect_pgsql_and_feedback() {}

#[test]
fn test_push_new_group() {
    tokio_test::block_on(impl_push_new_group());
}

async fn impl_push_new_group() {
    crate::dao::init_pg_pool().await;
    let temp = crate::dao::row::Group {
        id: 0,
        name: String::from("test_group"),
        admin: vec![0],
        owner: 0,
        member: vec![0],
    };
    crate::dao::group::push_new_group(&temp).await;
}

#[test]
fn test_new_table() {
    //tokio_test::block_on(impl_test_new_table());
}

#[allow(dead_code)]
async fn impl_test_new_table() {
    crate::dao::init_pg_pool().await;
    let _ = sqlx::query(
        r#"CREATE TABLE test_table(
    id bigint,
    "user" bigint
)"#,
    )
    .execute(PG_POOL.get().unwrap())
    .await;
}

#[test]
fn test_read_group_table() {
    tokio_test::block_on(impl_test_read_group_table());
}

async fn impl_test_read_group_table() {
    crate::dao::init_pg_pool().await;
    let id = "group_message_template";
    let sql = format!(
        r#"INSERT INTO "group"."{}" ("from", text, timestamp, message_id) VALUES($1, $2, $3, $4)"#,
        id
    );
    let _ = sqlx::query(&sql)
        //.bind()
        .bind(0)
        .bind("test")
        .bind(Utc::now().to_string())
        .bind(0)
        .execute(PG_POOL.get().unwrap())
        .await;
}
