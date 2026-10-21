use std::fmt::Display;

fn main() {
    // 生命周期与引用的有效性
    // 1 每一个引用都有生命周期：被引用的数据需要活的比引用更久
    let x = String::from("hello");
    let y = "xyz";

    // 2 生命周期：当函数的入参是多个引用，且返回值是引用类型时，需要标注生命周期
    // 生命周期的作用是告诉编译器引用的存活关系, 防止出现引用使用了已经死亡的数据
    // 生命周期标注不能改变引用的生命周期，只是用来描述多个引用生命周期之间的关系
    let result = longest(&x, y);

    // 3 什么情况下需要标注生命周期？
    // 输入是多个引用 + 输出是引用 = 必须标注
    // 4 怎么标注生命周期, 将生命周期的标注当作“对账游戏”，实事求是
    // 如果函数的返回值不确定和哪个输入有关，则返回值的生命周期和输入一样
    // 如果函数的返回值确定和哪个输入一致，则返回值和生命周期和某一个一致

    // 5 生命周期标注在函数签名上，表示的是对于数据存活时间的协议
    // 借用检查器会按照这个协议去检查 函数内实现 和 外部调用函数 的使用姿势是否符合协议
    // 不符合协议的则编译无法通过, 例如下面的例子，longest函数要求res <= s1 s2 的生命周期需要一致
    // 但是使用的人 res 的生命周期是 > s2的生命周期的，所以下面的代码编译无法通过
    /*
    let s1 = "hello".to_string();
    let res;
    {
        let s2 = "h".to_string();
        res = longest(&s1, &s2);
    }
    println!("res = {}", res);
    */

    // 6 生命周期省略规则
    /*
    函数或者方法的参数的生命周期称为输入生命周期，返回值称为输出生命周期
    编译器会根据3条规则计算引用的生命周期，如果3条规则后仍然存在没有计算出生命周期的引用，那么停止编译并报错
    规则1适用于输入生命周期，规则23适用于输出生命周期

    规则1 每一个是引用的参数都有它自己的生命周期参数。换句话说就是，
    有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
    有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推

    规则2 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32

    规则3 如果方法有多个输入生命周期参数并且其中一个参数是&self或者&mut self, 说明这是个实例的方法，那么所有的输出
    生命周期参数都等于self的生命周期
     */

    // 7 结构体中包含引用需要生命周期标注，意味着结构体实例的存活周期不能比引用存活的时间久

    // 8 'static 静态生命周期

    // 9 结合泛型类型参数、trait bounds 和 生命周期标注
    longest_with_an_announcement("hello", "word", "xxx");
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("ann = {}", ann);
    if x.len() > y.len() { x } else { y }
}

// 定义一个函数返回两个字符串中长的那个
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn func1<'a>(a: &'a str, b: &str) -> &'a str {
    &a[0..1]
}

// 包含引用的结构体
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 生命周期标注是结构体的一部分，impl后和类型名称之后的生命周期标注必须写
impl<'a> ImportantExcerpt<'a> {
    // 规则1 所有的输入参数都有自己的生命周期，这个规则实现后，输入输出都有标注了
    // fn level<'a>(&'a self) -> i32
    fn level(&self) -> i32 {
        3
    }

    // 规则1 announce_and_return_part<'a,'b>(&'a self, announcement: &'b str) -> &str
    // 规则2 不满足
    // 规则3 announce_and_return_part<'a,'b>(&'a self, announcement: &'b str) -> &'a str 满足
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
