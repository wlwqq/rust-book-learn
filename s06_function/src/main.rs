fn main() {
    println!("Hello, world!");
    another_function();
    another_function1(32);

    // 语句和表达式
    // 1 rust是基于表达式的语言
    // 6 是一个表达式，但是 let x = 6;是一个语句
    let x = 6;
    // 下面的res1 和 res2 中，表达式的结尾不加分号，加了分号之后表达式会变成语句
    let res1 = {
        let x = 1;
        x + 1;
    };
    let res2 = {
        let x = 1;
        x + 1
    };

    // 2 有返回值的函数
    let x = five();
    println!("The value of x is {}", x);
    let x = plus_one(5);
    println!("The value of x is {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
