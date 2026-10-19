use std::fs::File;

fn main() {
    // 1 panic!与不可恢复的错误
    let v = vec![1, 2, 3];
    // v[99];
    // 使用 RUST_BACKTRACE=1 cargo run 运行可以查看到完整的报错堆栈

    // 2 Result与可恢复的错误
    let f = File::open("hello.txt");
    match f {
        Ok(file) => println!("hello.txt is exist"),
        Err(err) => println!("err = {:?}", err),
    };

    // 3 使用 unwarp和expect简写Result的match处理
    let f = File::open("hello.txt").unwrap();
    // thread 'main' (2775057) panicked at src/main.rs:17:37:
    // called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // 4 使用expect的区别在于可以传递自定义的错误信息
    let f = File::open("hello.txt").expect("hello.txt open fail");
    // thread 'main' (2777322) panicked at src/main.rs:22:37:
    // hello.txt open fail: Os { code: 2, kind: NotFound, message: "No such file or directory" }
}
