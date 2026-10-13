fn main() {
    // 1 新建vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // vec! 宏

    // 2 更新vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // 3 读取vector的元素
    let v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    println!("the first of v is {}", first);

    match v.get(0) {
        Some(i) => println!("i = {}", i),
        None => println!("not found"),
    }

    // 4 vector会转移所有权么？
    let v = vec![1, 2, 3, 4, 5];
    let v0 = v[0]; // 这里是Copy
    println!("v[0] = {}", v[0]);
    let v = vec![String::from("a"), String::from("b")];
    // let v0 = v[0]; // vector和数组一样，不能实现所有权的移动，只能Copy

    // 5 借用规则对vec依然适用
    let mut v = vec![1, 2, 3, 4, 5];
    let v0 = &v[0]; // 这里发生了一次不可变的借用
    v.push(6); // 这里发生了一次所有者的修改vec
    // println!("v0 = {}", v0); // 这里使用了不可变的引用，在不可变的借用之间存在了可变的修改，这里是会报错的
    // 还有一个原因是当vec扩容时，可能会发生原来的内存空间不够，需要分配新内存然后存放数据，老内存就会被释放，为了防止
    // v0不会指向已经释放的内存造成内存安全问题，这里编译也不能放过
}
