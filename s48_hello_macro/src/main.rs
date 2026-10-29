use s48_hello_macro_derive::HelloMacro;
fn main() {
    // 自定义derive宏
    // 过程宏必须定义在独立的crate中，这里新增一个 s48_hello_macro_derive
    // 过程宏本质上只是一个“文本/语法转换器”，它在运行（编译期）时只做一件事：根据你写的代码模板，拼装出一段特定的 Rust 代码字符串（TokenStream），然后吐给编译器。
    Pancakes::hello_macro();
}

// 定义一个trait，可以写在当前的crate, 也可以写在 s48_hello_macro_derive 中
trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Pancakes;
