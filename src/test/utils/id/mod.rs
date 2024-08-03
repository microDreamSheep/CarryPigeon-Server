use std::{thread};
use std::time::Duration;
use crate::utils::id::generate_id;

#[test]
fn id_generate_test(){
    // 创建一个数组用来装子线程
    let mut handles = vec![];
    for i in 0..6 {
        let sleep_time = i;
        let handle = thread::spawn(move || {
            // 调用生成id
            for _ in 0..=10 {
                thread::sleep(Duration::from_millis(10));
                println!("{}",generate_id())
            }
        });
        // 存储子线程
        handles.push(handle);
    }
    // 主线程等待所有子线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }
}