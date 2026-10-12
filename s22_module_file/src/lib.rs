// 理论上所有的代码都只能写在src/main.rs 和 src/lib.rs中
// 因为这是和Cargo约定好的，Cargo会将这两个文件交给rustc编译
// 但是都写在一个文件中会导致文件过大，需要将一些代码拆分出来，之前在s21_module
// 中已经按照module对代码做了拆分，这里拆分多文件是在物理意义上将module拆分到
// 不同的文件中
// 拆分到文件的方法是
// 1.在src/main.rs或者src/lib.rs文件中声明mod front_of_house; 告诉 Rust 在另一个与模块同名的文件中加载模块的内容
// 2.新建src/front_of_house.rs 文件, 在front_of_house.rs文件中定义代码或者函数

// 拆分到文件和文件夹的方法是
// 1.在src/main.rs或者src/lib.rs文件中声明mod backend_of_house;
// 2.在src/backend_of_house.rs文件中声明 pub mod room; 表示去src/backend_of_house/目录下寻找room.rs
// 3.在src/backend_of_house/room.rs中定义函数

// 告诉 Rust 在另一个与模块同名的文件中加载模块的内容
mod front_of_house;
use crate::front_of_house::hosting;

mod backend_of_house;
use crate::backend_of_house::room::room_func;

fn my_function() {
    // 绝对路径使用
    crate::front_of_house::hosting::add_to_waitlist();

    // use导入查询
    hosting::add_to_waitlist();

    room_func();
}
