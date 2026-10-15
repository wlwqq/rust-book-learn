fn main() {
    // 1 遍历vec
    let mut v = vec![1, 2, 3, 4, 5];
    // 不可变借用
    for i in &v {
        println!("i = {}", i);
    }
    // 可变借用
    for i in &mut v {
        (*i) += 50;
    }

    // 2 使用枚举在vec中存储不同类型
    let row = vec![
        SheetCell::Int(5),
        SheetCell::Float(3.14),
        SheetCell::Text(String::from("hello")),
    ];
}

enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
