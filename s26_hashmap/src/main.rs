use std::collections::HashMap;

fn main() {
    // 1 新建hashmap
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    // 2 对二元组类型的vec执行collect()创建map
    let mut teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 3 hashmap与所有权
    let key = "a".to_string();
    let value = 97;
    let mut m: HashMap<String, i32> = HashMap::new();
    // key发生了移动 value发生了Copy
    m.insert(key, value);
    // println!("key = {}", key); // 报错
    println!("value={}", value); // 不报错

    // 4 访问map中的值
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Green".to_string(), 50);
    let score = scores.get("Blue");
    match score {
        Some(x) => println!("x = {}", x),
        None => println!("not found"),
    }

    // 5 遍历map
    for (k, v) in &scores {
        println!("key = {}, value={}", k, v);
    }

    // 6 更新map
    // 直接覆盖值
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Blue".to_string(), 20); // 直接覆盖值
    println!("Blue value = {}", scores.get("Blue").expect("not found"));

    // 当key不存在时新增值
    scores.entry("Blue".to_string()).or_insert(0);
    println!("Blue value = {}", scores.get("Blue").expect("not found"));
}
