fn main() {
    // if判断
    let number = 5;
    if number < 5 {
        println!("Condition is true");
    } else {
        println!()
    }

    // if表达式
    let x = if number == 5 { true } else { false };
    println!("The value of x is {}", x);

    // loop循环

    // while循环

    // for循环
    let arr1 = [1, 2, 3, 4, 5];
    for i in arr1 {
        println!("arr1 i = {}", i);
    }
    let arr2 = [String::from("a"), String::from("b")];
    // for循环消费了数组，转移了所有权
    for i in arr2 {
        println!("arr2 i = {}", i);
    }
    // println!("arr2[0]={}", arr2[0]);
}
