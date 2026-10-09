use std::fmt;
use std::io;
use std::io::Result as IOResult;
// 简写
// use std::{fmt, io};

// 简写，等于use std::io; use std::io::Write;
// use std::io::{self, Write};

mod modA {
    pub fn moda_func1() {}
}

fn main() {
    crate::modA::moda_func1();

    // 1 cargo new s20_pkg_and_crate 执行后s20_pkg_and_crate是包名
    // src/main.rs是一个与包同名的二进制crate的crate根
    // src/lib.rs是一个与包同名的库crate的crate根
    // crate根文件Cargo会传递给rustc来编译构建

    // 2 一个package可以包含多个二进制crate和1个库crate；crate中可以包含多个module

    // 3 由于crate名和包名一样，所以在看代码时，不要以为是在调用包内的内容，而是在调用crate中的内容

    // 4 module的作用是对crate中的代码做分组，还可以控制代码是public还是private的
    // 参考 s21_restaurant 包中的代码学习module

    /*
    关于module，内容比较多，这里总结一下
    1.在src/lib.rs或者src/main.rs文件中定义mod，
    mod可以嵌套，mod可以设置可见性，默认是私有，需要用pub公开
    2.使用mod中的函数或者结构体枚举等数据数据结构，需要使用绝对路径或者相对路径使用，
    绝对路径是用 create::开头，注意是字面量crate
    相对路径注意是相对当前使用位置所在的mod来作为相对参考，使用super::也是相对当前mod的上一层mod
    3.使用绝对路径和相对路径都是导致每次使用函数都要一段比较长的前缀，可以使用use + 绝对路径导入进来后，
    直接使用导入的名称来使用函数，不用写很长的前缀，注意use的一些简写，包括use xxx as yyy 之类的
    4.如果外部的人想要调用我们代码中的一些数据结构，可以一层一层的use下去，比如use a::b::c::d::func(),
    如果不想让别人看到我们自己代码的mod层级，则可以使用pub use 将a::b::c::d::func() 重导出，
    这样别人在调用时只需要写包名::func()
     */
}

// func1 和 func2 在使用use引入的Result时需要区分开
fn func1() -> fmt::Result {
    Ok(())
}

fn func2() -> io::Result<()> {
    Ok(())
}

// use配合as指定新的名称
fn func3() -> IOResult<()> {
    Ok(())
}
