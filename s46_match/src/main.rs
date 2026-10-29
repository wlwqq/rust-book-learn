use crate::Message::ChangeColor;

fn main() {
    // 模式匹配的各种写法
    // 1 match 分支
    // 必须穷尽所有分支
    let x = Some(5);
    match x {
        Some(i) => println!("i = {}", i),
        None => (),
    }

    // 2 if let
    // 可以不用穷尽所有分支，不会检查
    if let Some(i) = x {
        println!("i={}", i);
    } else if true {
        println!("true");
    }

    // 3 while let
    let mut stack = vec![1, 2, 3];
    while let Some(x) = stack.pop() {
        println!("x={}", x);
    }

    // 4 for循环
    let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    for (i, e) in v.iter().enumerate() {
        println!("i={},e={}", i, e);
    }

    // 5 let语句
    let (x, y) = (1, 2);

    // 6 函数参数
    func1((100, 101));

    // 匹配语法
    // 1 匹配字面量
    let x = 1;
    match x {
        1 => println!("1"),
        _ => println!("other"),
    }

    // 2 匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 这里是 let y = x; 给y绑定了x的值
        _ => println!("Default case, x = {:?}", x),  // x是外部的x
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // 3 ｜ 表示或匹配多个模式
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 4 ..= 或者 .. 匹配范围
    let x = 4;

    match x {
        1..5 => println!("one through four"),
        _ => println!("something else"),
    }

    // 5 解构结构体
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
    // 6 解构枚举
    let msg = ChangeColor(1, 2, 3);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        _ => println!("other"),
    }
    // 7 解构嵌套的结构体或枚举
    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    // 8 解构结构体和元组
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 9 使用_忽略整个值 或者 嵌套的_忽略部分值
    // _ 不会造成值和变量的绑定
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // 10 使用 .. 忽略值
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // 11 循环守卫
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 12 使用 @ 绑定值 和 匹配条件
    // 使用 id_variable @ 绑定值，并且判断是否在 3-7之间
    let h = Nessage::Hello { id: 5 };
    match h {
        Nessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Nessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
        _ => (),
    }
}

enum Nessage {
    Hello { id: i32 },
}

fn func1((a, b): (i32, i32)) {
    println!("a={},b={}", a, b);
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor2(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
