fn main() {
    let v = vec![1, 2, 3];
    // 1 创建一个迭代器
    let v1_iter = v.iter();
    for val in v1_iter {
        println!("get {}", val);
    }

    // iter() 获取不可变引用的迭代器
    // iter_mut() 获取可变引用的迭代器
    // into_iter() 获取所有权的迭代器

    // 2 消费迭代器的方法
    // 消费迭代器的方法会获取 迭代器的所有权，迭代器是新的数据结构，所以他有自己的所有权
    // sum()方法的签名会获取所有权，所以迭代器被消费之后无法二次使用
    let v = vec![1, 2, 3];
    let v1_iter = v.iter();
    // 调用sum之后不能再使用v1_iter了
    let total: i32 = v1_iter.sum();

    // 3 产生其他迭代器的方法
    // 持续产生新的迭代器，还是惰性的
    let v = vec![1, 2, 3, 4, 5];
    let v1: Vec<i32> = v.iter().map(|x| x * 2).filter(|x| x % 2 == 0).collect();
}
