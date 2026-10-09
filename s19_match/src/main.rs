fn main() {
    // 1 模式匹配
    let c = Coin::Nickel;
    println!("value_in_cents = {}", value_in_cents(c));

    // 2 匹配Option
    let five = Some(5);
    let six = plus_one(five);

    // 3 _ 通配符匹配
    let c = Coin::Penny;
    coin_part_match(c);

    // 4 if let 简写只有一种匹配的情况
    // = 左边是模式，右边是match要匹配的表达式
    let some_one = Some(3);
    if let Some(i) = some_one {
        println!("i = {}", i);
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 注意这里有所有权的转移了
fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 15,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn coin_part_match(c: Coin) {
    match c {
        Coin::Dime => println!("Dime"),
        _ => println!("other"),
    }
}
