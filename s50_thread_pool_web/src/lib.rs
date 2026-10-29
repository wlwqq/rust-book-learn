use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

// ThreadPool 中维护多个worker
// 每个worker都有一个id和一个线程
// 通过channel，ThreadPool向通道发送任务，Worker获取任务并执行
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(thread_num: u32) -> ThreadPool {
        assert!(thread_num > 0);
        // 创建channel
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        // 创建workers
        let mut workers = Vec::with_capacity(thread_num as usize);
        for id in 0..thread_num {
            // 创建线程
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 将这个闭包封装到Box中, 因为闭包的大小是不一样的
        let job = Box::new(f);
        // 发送到通道中
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

// Mutex 保证只有某个时刻只有一个线程可以通过reveiver获取任务
// Arc保证receiver可以被多个线程持有，通过引用计数
// 这里有一个需要注意的点：let job = receiver.lock().unwrap().recv().unwrap();
// 这一行到分号结束，锁就被释放了，lock()返回的是一个智能指针，智能指针的生命周期结束是}位置，即作用域结束才释放
// 那为什么这里到了分号结尾就释放呢，因为这里是链式编写，是一个没有具名变量的临时值
// Rust 编译器规定：如果一个临时值没有被绑定到一个具名变量上，它会在包含它的“当前语句（Statement，即遇到第一个 ;）”结束时立刻被销毁（Drop）。
// 如果用这种写法let guard = receiver.lock().unwrap(); 那么锁要到}再释放
// 这里又要注意一点，就是引用，引用不一样，引用并不拥有值，引用在老版本也是到}结束，但是在新版本引用是在最后一次使用的地方结束
struct Worker {
    id: u32,
    thread: Option<JoinHandle<()>>,
}
impl Worker {
    fn new(id: u32, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                // 获取到任务后锁释放
                match message {
                    Message::NewJob(job) => {
                        // 是任务就处理
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        // 是终止消息就停止循环，这个线程就执行结束了
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
