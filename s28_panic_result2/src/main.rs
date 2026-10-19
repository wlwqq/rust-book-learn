use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    // 1 传播错误，当发生错误时将错误传递给调用者解决
    let name = get_username_from_file();

    // 2 由于传播错误是比较常见的，所以在Rust中提供了?运算符来简化代码
    // ?的作用是如果Result的结果是Ok，则返回Ok中的值，如果是Err，?会检查函数签名的Err有没有实现From trait
    // 如果有的话，?会帮我们将代码的Err转换为函数签名的Err返回
    let name = read_username_from_file();

    // 3 只有在返回值是Result的函数中才可以使用?，因为?的处理机制就是判断是Ok还是Err
    // 所以直接在main函数中使用?是不行的，main函数的返回类型只有2种
    // 一种是 fn main() {}
    // 一种是 fn main() -> Result<(), Box<dyn Error>> {}
    // Box<dyn Error> 被称为 trait对象，这种方式可以在main函数中接收入任何错误
}

fn get_username_from_file() -> Result<String, io::Error> {
    let f = File::open("name.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut name = String::new();
    match f.read_to_string(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
