fn main() {
    // Copy类型的元组
    test_copytuple();
    println!("==========");
    // 非Copy类型的元组
    test_noncopytuple();
    println!("==========");
    // Copy类型的数组
    test_copyarray();
    println!("==========");
    // 非Copy类型的数组
    test_noncopyarray();
    println!("==========");
    // Copy类型的结构体
    test_copystruct();
    println!("==========");
    // 非Copy类型的结构体
    test_noncopystruct();
}
#[derive(Debug)]
struct Point1 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

fn test_copytuple() {
    let t1 = (1, true, 3.14);
    let (x, y, z) = t1;
    let t2 = t1;
    println!("x ={}, y={}, z={}", x, y, z);
    println!("t2.0={},t2.1={},t2.2={}", t2.0, t2.1, t2.2);
    println!("t1.0={},t1.1={},t1.2={}", t1.0, t1.1, t1.2);
}

fn test_noncopytuple() {
    // 存在非Copy元素的元组整体都是非Copy的，赋值时整体发生移动
    let t1 = (1, true, String::from("hello"));
    let t2 = t1; // t1 移动给了t2, t1整体都不可用了
    println!("t2.0={},t2.1={},t2.2={}", t2.0, t2.1, t2.2);
    // println!("t1.0={}", t1.0); // 这里会报错

    // 部分移动
    let t1 = (1, true, String::from("hello"));
    let x = t1.0; // t1.0 是Copy类型，发生复制
    let z = t1.2; // t1.2 是非Copy类型，发生了移动
    println!("The value of z = {}", z);
    println!("The value of t1.0= {}", t1.0); // t1.0可以被访问
    // println!("The value of t1.2= {}", t1.2); // 因为移动了不能访问
}

fn test_copyarray() {
    let a1 = [1, 2, 3, 4, 5];
    // Copy类型发生复制
    for i in a1 {
        println!("a1= {}", i);
    }
    // 依然可以访问a1
    println!("a1[0] = {}", a1[0]);
}

fn test_noncopyarray() {
    let a1 = [String::from("a"), String::from("b")];
    // 非Copy类型，在这里会发生移动
    for i in a1 {
        println!("a1 = {}", i);
    }
    // a1 整体发生了移动
    // println!("a1[0]={}", a1[0]);

    let a1 = [String::from("a"), String::from("b")];
    let a2 = a1;
    // a1 整体发生了移动
    // println!("a1[0]={}", a1[0]);

    let a1 = [String::from("a"), String::from("b")];
    // 数组不允许部分移动，这一点和元组不一样，下面代码会报错
    // 元组的每个元素是独立的，允许移动单个元素，内存位置不会发生变化
    // 但是数组的每个元素是连续的，如果允许单个元素移动的话，该元素的内存会空一块，这样子不知道怎么遍历数组了
    // 换个简单的思路思考，数组的类型是[类型;长度] 允许单个元素移动的话长度就破坏了
    // let s = a1[0];
}

fn test_copystruct() {
    // Copy类型 不加Copy和Clone trait 依然是移动语义
    let t1 = Point1 { x: 1, y: 1 };
    let t2 = t1; // 发生移动
    // println!("t1 = {:?}", t1); // t1中的内容直接移动到了t2中，t1 不可用

    // 部分走的是复制语义
    let t1 = Point1 { x: 1, y: 1 };
    let x = t1.x; // Copy类型 复制语义
    println!("x = {}", x);
    println!("t1 = {:?}", t1); // t1依然可用

    // Copy类型 + Copy，Clone trait，复制语义
    let t1 = Point2 { x: 1, y: 1 };
    let t2 = t1; // 复制语义
    println!("t2 = {:?}", t2);
    println!("t1 = {:?}", t1); // t1依然可用
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
fn test_noncopystruct() {
    // 注意 Person 无法添加Copy和Clone，因为Person存在非Copy语义的元素
    // 编译器会帮助我们检查的
    let p1 = Person {
        name: String::from("wlh"),
        age: 11,
    };
    let p2 = p1; // 移动语义
    println!("p2 = {:?}", p2);
    // 无法使用p1
    // println!("p1 = {:?}", p1);

    // 部分移动
    let p1 = Person {
        name: String::from("wlh"),
        age: 11,
    };

    let age = p1.age; // 复制语义 p1 还能用
    println!("p1 = {:?}", p1);

    let name = p1.name; // 移动语义，p1.name 不可以用了 p1.age还可以，和元组一样
    println!("p1.age = {}", p1.age);
}
