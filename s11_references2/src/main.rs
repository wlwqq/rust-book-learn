fn main() {
    // 1.不可变引用 和 可变引用的数据竞争问题
    // 引用的作用域从声明引用开始，到最后一次使用引用为止(或者离开作用域)

    // demo1 在同一时间，只能有一个对某一特定数据的可变引用
    // 简单点说，有可变引用代表数据可能被修改，在这个期间，不能有别的可变引用，也不能有别的不可变引用，防止有数据竞争的问题
    test_demo1();

    // demo2 有可变引用时，不能存在不可变引用
    test_demo2();

    // demo3 都是不可变引用没关系，都是只读
    test_demo3();

    // 2.引用是对某块数据的使用，如果数据已经被gc了(所有者离开了作用域)，引用还存在的话，这种会访问到未知的内存也叫悬垂引用
    // Rust编译器会检查

    // demo4 引用是Copy类型
    test_demo4();
}

fn test_demo1() {
    let mut s = String::from("hello");
    let t1 = &mut s;
    let t2 = &mut s;
    // 报错，在t1的声明到使用之间，t2也是对同一份数据的可变引用，不满足同一时间某一个数据只能有一个可变引用
    // println!("t1 = {} t2 = {}", t1, t2);
    // 不报错
    println!("t2 = {}", t2);
}

fn test_demo2() {
    let mut s = String::from("hello");
    let t1 = &mut s;
    let t2 = &s;
    // 报错，在可变引用的使用周期内，存在不可变引用
    // println!("t1 = {}", t1);

    // 不报错
    println!("t2 = {}", t2);
}

fn test_demo3() {
    let s = String::from("hello");
    let t1 = &s;
    let t2 = &s;
    let t3 = &s;
    println!("t1={},t2={},t3={}", t1, t2, t3);
}

fn test_demo4() {
    let s = String::from("world");
    let t1 = &s;
    let t2 = t1;
    println!("t1 = {}, t2 = {}", t1, t2);
}
