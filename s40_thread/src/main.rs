use std::{thread, time::Duration};

fn main() {
    // 1 创建线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi num={} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // 2 move闭包
    // rust会根据闭包中如何使用环境来决定是获取环境的不可变/可变/所有权
    // 但是闭包结合线程后，这种推断会来带来问题
    // 比如下面，如果不写move，由于在闭包中只是println，所以获取vec的不可变引用即可，但是这种就会编译失败，因为无法保证vec能一直活着
    let v = vec![1, 2, 3];
    thread::spawn(move || println!("vec = {:?}", v));
}
