use std::{sync::mpsc, thread};

fn main() {
    // 1 使用channel传递消息
    // mpsc 多生产者单消费者 multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    // 轻量级复制一个新的生产者
    let tx1 = tx.clone();
    //
    thread::spawn(move || {
        let val = "tx hi".to_string();
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = "tx1 hi".to_string();
        tx1.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
