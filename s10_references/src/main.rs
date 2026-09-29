fn main() {
    // 变量和数据的交互要么是移动，要么是复制
    // 移动会伴随所有权的交接
    // 复制会伴随的数据的双份，不管是简单的i32的复制，还是Copy类型的结构体的复制

    // 引用：在不移动交接所有权 和 不复制的情况下，使用引用来实现变量和数据的交互
    // 引用允许不转移所有权(不发生移动)，也不会发生数据的复制的情况下，使用数据，是数据和变量的第三种交互方式
    // 很重要的一点，引用等于临时借用数据，最后是要还的！！！
    // 创建引用的过程叫做借用borrowing

    // 1 不可变引用
    let mut s1 = String::from("hello");
    // 等价let s = &s1 ； 这里s存储是s1的地址, 也叫做s是s1的引用，引用不拥有所有权，所以在离开作用域时，什么也不会发生
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    // 2 可变引用
    change(&mut s1);
    println!("s1 = {}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s借用结束，离开作用域

fn change(s: &mut String) {
    s.push_str("world");
} // s借用结束，离开作用域
