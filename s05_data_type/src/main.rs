fn main() {
    // 1 标量类型表示单个值，Rust有4个标量类型：整型，浮点型，布尔型，字符

    // 2 复合类型表示多个值合成一个类型，Rust有2种基本的复合类型：元组和数组
    // 2.1 不同类型固定长度的是元组
    // 将元组绑定到tup1变量上
    let tup1 = (1, true, 3.14);
    // 模式匹配解构元组
    let (x, y, z) = tup1;
    println!("x = {}, y = {},z = {}", x, y, z);

    // 课外知识
    // 元组的解构是值绑定变量的过程，如果值是Copy类型，那么发生Copy，如果值不是Copy类型，那么发生move
    // 元组没有独立的所有权，他的所有权是内部每个元素的所有权
    // 下面将 1 绑定到t1上，是Copy行为，将"wlh"绑定到t2上是move行为(所有权在这里被move了)
    // 所以tup2.0可以访问，但是tup2.1不能被访问，因为"wlh"现在属于t2了
    let tup2 = (1, String::from("wlh"));
    let (t1, t2) = tup2;
    println!("t1={}, t2={}", t1, t2);
    println!("tup2.0={}", tup2.0);
    // borrow of moved value: `tup2.1`
    // move occurs because `tup2.1` has type `String`, which does not implement the `Copy` trait
    // println!("tup2.1={}", tup2.1);

    // 2.1 相同类型固定长度的是数组
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("months[0] = {}", months[0]);
    // 数组类型为 [元素类型;元素个数]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 元素为3，长度为5
    let b = [3; 5];
    // 访问元素
    println!("b[3]={}", b[3]);

    // 课外知识
    // 和元组的所有权转移不一样，如果不是Copy类型，那么会直接报错，不能发生move行为
    // 下面第二行代码无法编译通过，rust中数组是一个整体连续的内存，和元组的内存结构不一样
    // 无法对数据的某个元素做move操作，元组的某个元素发生move之后，
    // 编译器可以标记该位置无法被访问，但是编译器无法对数组的某个位置操作
    let hello = [String::from("hello")];
    // let  h1 = hello[0];
}
