fn main() {
    // 1 定义枚举
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    // 2 定义枚举: 关联数据的枚举
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // 3 复杂枚举
    let e1 = Message::Quit;
    let e2 = Message::Move { x: 1, y: 2 };
    let e3 = Message::Write(String::from("hello"));
    let e4 = Message::ChangeColor(1, 2, 3);
    e1.show();

    // 4 Option: 被包含在prelude中，所以不需要显式引入, 直接使用Some(T) 和 None，不用通过Option::使用
    let some_num = Some(5);
    let some_string = Some("a string");
    let absent_num: Option<i32> = None; // 当使用None时，需要定义出Option<i32>，因为这种情况无法推断类型
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // 不关联任何数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含String
    ChangeColor(i32, i32, i32), // 包含3个i32
}

// 枚举可以定义方法
impl Message {
    fn show(&self) {
        println!("show");
    }
}
