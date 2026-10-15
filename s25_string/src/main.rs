fn main() {
    // 1 新建字符串
    let s1 = String::new();

    // 2 从字面量或者说字符串切片创建字符串，此时会申请内存存储字符串
    let s2 = "hello".to_string();
    let s3 = String::from("world");

    // 3 字符串是可变的，可以修改大小，可以修改内容
    let mut s = String::from("foo");
    s.push_str("bar"); // 添加字符串字面量
    s.push('!'); // 添加字符

    // 4 使用 + 连接字符串
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    // + 类似add函数 add(self, &str) -> String
    // self会将s1的所有权移动到add函数内，所以s1会失效，s2是引用，不会转移所有权
    // 另一个细节在于add函数的参数类型是&str，但是&s2的类型是&String
    // 这里调用可以成功的原因是&String可以被强制转换为&str, 这里会把参数 &s2强转成&s2[..]， &s2[..]就是字符串切片了
    let s3 = s1 + &s2;

    // 5 使用format! 连接字符串，不会获取所有权，只会获取引用
    let s1 = "one".to_string();
    let s2 = "two".to_string();
    let s3 = "three".to_string();
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("s1 = {}", s1);
    println!("s4 = {}", s4);

    // 6 遍历字符串的方式
    let s = "hello中国".to_string();
    for c in s.chars() {
        println!("c = {}", c);
    }
}
