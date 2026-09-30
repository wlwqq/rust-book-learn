fn main() {
    // 字符串字面量
    // s是&str，是不可变引用
    let s = "Hello world!";

    // 字符串slice作为函数参数更通用
    let s = String::from("hello world");
    let s1 = first_word(&s);
    println!("s1 = {}", s1);

    // 切片的切片
    let s = String::from("helloworld");
    let s1 = &s[0..5];
    println!("s1 = {}", s1); // hello

    let s2 = &s1[0..2];
    println!("s2 = {}", s2); // he

    // 引用是瘦指针，存储的是所有者变量的地址，所以说可变引用对数据的修改其实是委托所有者进行修改的
    // 切片是胖指针，存储的是序列的元素地址和切片持有的长度，这个元素地址是切片的第一个元素的地址
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
