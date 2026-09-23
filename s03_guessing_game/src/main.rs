// 导入标准库std中的io库
use std::io;
// Rng是一个trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，Rng这个trait必须在作用域中
use rand::Rng;
// Ordering是一个枚举
use std::cmp::Ordering;
fn main() {
    // 打印提示
    println!("Guess the number!");
    // 生成随机数 rand::thread_rng() 返回 ThreadRng的实例
    // ThreadRng实现了Rng trait，但是gen_range()定义在Rng接口中，是接口的关联方法
    // ThreadRng本身并没有gen_range()方法，所以必须要使用use rand::Rng 导入接口中已经实现的方法
    // 编译器查找方法的顺序：1.查找类型本身的方法 2.找实例类型实现的trait的方法，要求作用域中这个trait已经导入
    // 所以gen_range()在上面的第二步中被找到了
    // 另外1..101是左闭右开的写法，1..=100是左闭右闭的写法
    let secret_number = rand::thread_rng().gen_range(1..101);

    // 使用循环来进行多次猜测
    loop {
        println!("Please inout your guess.");
        // 创建一个可变的变量guess
        // = 表示将值绑定到变量上，这里是将String::new()的结果绑定到变量guess上
        // :: 语法表示new是String的一个关联函数，创建了一个String实例
        let mut guess = String::new();

        // io::stdin() 返回了一个Stdin实例
        // read_line 是Stdin实例的函数，作用是将用户在标准输入的内容追加到入参中
        // &表示引用，&mut表示这是一个可变的引用，&mut guess 表示将guess的可变引用 绑定到 函数的形参buf上
        // read_line 函数返回的结果是io::Result<usize>，Result是枚举，有Ok和Err两种值，expect是Result类型的方法
        // 如果是Ok，expect函数会返回Ok中的值，如果是Err，会直接panic
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // {} 表示占位符
        println!("Your guessed: {}", guess);

        // 将用户输入的内容转成数字
        // trim()去除字符串左右空白字符，parse()用来将字符串转为u32类型，返回的依然是Result类型，需要处理失败的情况
        // 这里创建了一个同名的新变量guess(变量遮蔽), 将右边的结果绑定到guess上
        // 处理用户错误的输入
        let guess: u32 = match guess.trim().parse() {
            Ok(e) => e,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // 比较用户猜测的数字 和 随机生成的数字的大小关系
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                // 猜测正确后退出循环
                break;
            }
        }
    }
}
