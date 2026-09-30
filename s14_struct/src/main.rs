fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    let uname = user1.username; // 移动
    // 移动后无法使用user1.username
    // println!("user1.username= {}", user1.username);
    println!("user1.active={}", user1.active);

    // 1 结构体的字段不允许设置为可变，必须将整个结构体变量设置为可变
    let mut user2 = User {
        active: true,
        username: String::from("someone111"),
        email: String::from("someone111@example.com"),
        sign_in_count: 2,
    };

    user2.username = String::from("someone222");

    // 3 从旧结构体实例创建新结构体实例
    // 这里会涉及 移动 或者 复制 语义，因为这里等于简写 email: u1.email,
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = User {
        username: String::from("u2"),
        ..u1
    };
    // println!("u1.email = {}", u1.email);
    println!("u1.active = {}", u1.active);
    println!("u2.email = {}", u2.email);

    // 4 结构体引用
    let t1 = &User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    // 下面代码会报错，因为t1是借用来的引用，他不能把非Copy的东西移动出去
    // let uame = t1.username;
    // 下面代码不会报错，因为走的是copy
    let sign = t1.sign_in_count;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 2 当参数名 和 字段名一致，则可以简写
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 2,
    }
}
