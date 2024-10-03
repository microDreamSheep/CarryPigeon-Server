use crate::utils::id::generate_id;
use std::thread;
use std::time::Duration;

#[test]
fn id_generate_test() {
    tokio_test::block_on(impl_id_generate_test());
}

async fn impl_id_generate_test() {
    // 创建一个数组用来装子线程
    let mut handles = vec![];
    for i in 0..6 {
        let _sleep_time = i;
        let handle = thread::spawn(move || {
            // 调用生成id
            for _ in 0..=10 {
                thread::sleep(Duration::from_millis(10));
                println!("{}", generate_id())
            }
        });
        // 存储子线程
        handles.push(handle);
    }
}
