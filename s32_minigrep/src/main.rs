use s32_minigrep::Config;
use std::env;
use std::process;

fn main() {
    // std::env::args() 返回了一个包含命令行参数的迭代器
    // 获取命令行的参数,第一个参数是程序本身
    let args: Vec<String> = env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        // 打印到错误输出流
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    println!("Search for:{}", config.query);
    println!("File:{}", config.file_name);

    // 运行程序
    if let Err(e) = s32_minigrep::run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
