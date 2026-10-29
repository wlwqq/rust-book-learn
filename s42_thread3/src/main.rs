use std::{sync::Arc, sync::Mutex, thread};
fn main() {
    // 1 共享状态并发：使用互斥器保护数据，在任意时刻，只能有一个线程访问互斥器中的数据，线程首先通过
    // 获取互斥器的锁来表明希望访问数据，锁是互斥器的一部分，记录了谁有数据的排他访问权

    // demo1:单线程中使用MMutex
    let m = Mutex::new(5);
    {
        // lock方法会阻塞线程直到获取锁，Mutex是一个智能指针，lock()返回的也是智能指针MutexGuard
        // 这个指针实现了Deref，所以可以当作普通引用来操作值，也实现了Drop，所以会在作用域结束时自动释放锁
        // 我们不需要手动释放锁，锁的释放是自动发生的
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // demo2:多线程中使用Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
