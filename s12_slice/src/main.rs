fn main() {
    // slice是另外一种不拥有所有权的数据结构，可以引用集合中一段连续的元素

    // 1 字符串slice
    let s = String::from("hello world");
    let hello = &s[0..5]; // 左闭右开
    let world = &s[6..11];

    // 2 借用检查器
    let mut s = String::from("hello world");
    let s1 = first_word(&s);
    // 下面这行等于是s的可变引用的修改，但是s1的作用域持续到后面的println
    // 在不可变引用之间不能存在可变引用，借用检查器会报编译错误
    // s.clear();
    println!("s1 = {}", s1);
}

fn first_word(s: &String) -> &str {
    // 字符串转字节slice
    let bytes = s.as_bytes();

    // iter()获取迭代器，enumerate()将迭代器封装为(索引，数据)
    // 这里有个引用结构的语法糖，使用&item 匹配引用类型，那么item就是具体的值
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
