use std::{
    fs,
    io::Read,
    io::Write,
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // TcpListener income()方法返回的是一个迭代器，其中的每个流代表客户端和服务端之间的连接
    // incoming()是阻塞的，一次只会从操作系统的底层连接队列中获取一个
    let connection_iter = listener.incoming();
    for stream in connection_iter {
        let stream = stream.unwrap();
        // handle_connection函数执行完后，连接会因为作用域结束于是自动关闭掉
        // 所以for循环其实是调用的迭代器的下一个next()方法，底层调用的是accept()方法
        // 查看有没有用户的新建连接
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let read_size = stream.read(&mut buffer).unwrap();
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

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
