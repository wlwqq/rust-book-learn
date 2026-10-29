use s50_thread_pool_web::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::{fs, io::Read, io::Write};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // 创建4个线程的线程池
    for connection in listener.incoming() {
        let connection = connection.unwrap();
        pool.execute(|| {
            handler_connection(connection);
        });
    }
}

fn handler_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];
    let read_size = connection.read(&mut buffer).unwrap();
    println!(
        "Request:\n{}",
        String::from_utf8_lossy(&buffer[..read_size])
    );
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    connection.write(response.as_bytes()).unwrap();
    connection.flush().unwrap();
} // connection 离开作用域，释放并且断开了连接
