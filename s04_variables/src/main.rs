fn main() {
    // 1 不可变变量
    // 两层含义：a.变量不能重新绑定值 b.即使值是struct，vector之类的，也不能通过这个变量修改
    // 这里提示一下，和scala中的val不同，val只限制不能重新绑定值，
    // 但是如果值是可变list之类的话，是可以通过val变量修改list的
    let x = 5;
    println!("The value of x is:{}", x);
    // error[E0384]: cannot assign twice to immutable variable `x`
    // x = 6;

    // 2 可变变量
    let mut y = 5;
    println!("The value of y is:{}", y);
    y = 10;
    println!("The value of y is:{}", y);

    // 3 常量: 绑定到常量名且不允许更改的值
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // 4 遮蔽：声明和前面变量同名的变量，就会实现第一个变量被第二个变量遮蔽了
    let val1 = 5; // 将5绑定给变量val1，这里我们称之为val1 1号
    let val1 = val1 + 1; // 创建新的同名变量, 将6绑定给val1，这里我们称它为val1 2号
    {
        let val1 = val1 + 1; // 创建新的同名变量, 将7绑定给val1，这里我们称之为val1 3号
        println!("The inner value of val1 is:{}", val1);
    } // val1 3号在这里作用域结束了，遮蔽也就结束了
    // 这里回到val1 2号
    println!("The value of val1 is:{}", val1);

    // 变量遮蔽还有一个特点是 因为变量遮蔽是隐藏，不是覆盖，所以可以通过变量遮蔽做到看似修改变量类型的操作
    // 这里看似修改变量类型，实际上是创建了多个独立的变量
    let str = "hello world";
    let str = str.len(); // 这两个str在内存中是相互独立的
    println!("The value of str is:{}", str);
} // val1 1号 和 val1 2号在这里作用域结束，GC
