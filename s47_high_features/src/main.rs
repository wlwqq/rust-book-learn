use std::fmt::Display;
fn main() -> std::io::Result<()> {
    let x = 5;
    let y = &x;

    // 高级trait
    /*
        1 trait的关联类型
        关联类型（Associated Types）就像是在 Trait 内部留了一个“类型占位符”。
        它的核心作用是：把具体类型的确定时间推迟到 Trait 被实现（impl）的那一刻。
        pub trait Iterator {
            type Item;

            fn next(&mut self) -> Option<Self::Item>
        }

        在具体实现Iterator时，需要指定Item的具体类型

        2 trait中泛型的默认泛型类型参数
        trait Add<RHS=Self> {
        type Output;

        fn add(self, rhs: RHS) -> Self::Output;
        }

        RHS是一个泛型，在此基础上RHS=Self，表示在没有手动指定RHS的情况下，就是只写impl Add for xxxx
        的情况下，RHS的具体类型就是默认类型，如果需要指定别的类型，那就impl Add<Meters> for Millimeters显示的写出来


        3 当同一个实例实现了多个trait，且trait中的方法名称一致
        使用trait名::方法名(实例)
        如 Wizard::fly(&people)

        4 当trait中定义关联函数，并且实例实现了接口
        // 下面的例子中，Dog本身有关联函数，Dog实现了Animal也有关联函数
        // 使用Dog::函数 只能调用Dog自己的关联函数，如果想要调用Dog实现的Animal的函数，需要<Dog as Animal>::函数使用
        println!("{}", Dog::baby_name());
        println!("{}", <Dog as Animal>::baby_name());
        trait Animal {
            fn baby_name() -> String;
        }
        struct Dog {}

        impl Dog {
            fn baby_name() -> String {
                String::from("Dog")
            }
        }

        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("Animal")
            }
        }

        5 父trait
        trait B:A {}
        这种定义方式表示 trait A是trait B的父trait，表示任何想要实现B的类型必须要先实现A
        那么在实现B时，可以直接调用A的方法
        实现 OutlinePrint的必须实现 fmt::Display
        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {} *", output);
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }

        6 孤儿规则：如果你想为某个类型实现某个 Trait，那么“类型”和“Trait”这两个东西中，至少有一个必须是由你当前所在的这个 crate（Crate，即你的项目或库）所定义的。
        如果类型和 Trait 都是别人写的（你全占不到），你就不能为它俩实现 Trait。这就像孤儿一样“无依无靠”，所以被称为孤儿规则
        比如想要为vec类型实现Display trait这是实现不了的，因为vec是标准库的，Display也是标准库的，都不是本地的，所以不能这样做
        可以使用newtype打破这种规则，原理上是构建一个类型包装一下vec，然后用这个新类型实现trait
        fn main() {
            let w = Wrapper(vec![String::from("hello"), String::from("world")]);
            println!("w = {}", w);
        }
        struct Wrapper(Vec<String>);

        impl Display for Wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "[{}]", self.0.join(", "))
            }
        }

    动态类型DST
    胖指针的体积是普通指针的两倍（在 64 位系统上是 16 字节），它由两部分组成：
    数据指针（Data Pointer）：指向实际存储数据的内存地址。
    元数据（Metadata）：记录这个 DST 的“额外信息”：
    对于 str 和 [T]，元数据是元素的数量（长度，Length）。例如 &str 包含（地址 + 字符长度）。
    对于 dyn Trait，元数据是虚表指针（Vtable），里面记录了该类型实现了哪些方法，以便进行动态派发。

    // 7  函数指针
    函数指针存的是8字节的地址，地址指的是函数的地址，和闭包的区别是闭包其实是一个匿名结构体，可以捕获外部变量，但是函数不能
    函数指针的写法是fn。闭包的写法是Fn FnMut FnOnce

    // 8 宏: 代码生成的
    macro_rules! 的声明宏：类似代码替换
    三种过程宏：类似读取部分代码生成部分代码
        自定义#[derive]宏在结构体和枚举上指定通过derive属性添加代码
        类属性宏定义可用于任意项的自定义属性
        类函数宏看起来像函数不过作用于作为参数传递的token

    声明宏
    #[macro_export] 说明只要将定义了宏的 crate 引入作用域，宏就应当是可用的
    macro_rules! vec {} 表示宏的名称是vec ,{} 中表示宏的定义体
    宏的定义体由 模式 => {} 组成，表示如果模式匹配，就执行{}中的代码块
    $($x:expr),*     ,* 表示$($x:expr)可以匹配多个，$x:expr expr表示匹配任何表达式，表达式计作$x
    #[macro_export]
    macro_rules! vec {
        ( $($x:expr),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }


    过程宏：过程宏接收rust代码，在这些代码上进行操作，产出另一些代码，声明宏更像是匹配到模式后替换代码

    总结
    声明宏：macro_rules!，使用时类似vec![] 用来做代码替换的，匹配上内容就替换成对应的代码，类似于文本替换
    过程宏：
        derive宏生成接口的默认实现代码，#[derive(MyTrait)]，作用在结构体或者枚举上
        属性宏可以作用在结构体，枚举，函数上，类似#[route(GET, "/")]
        函数宏和声明宏很像，但是比声明宏强大，类似sql!(SELECT * FROM posts WHERE id=1);

         */
    println!("do_twice(1) = {}", do_twice(add_one, 1));

    Ok(())
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
