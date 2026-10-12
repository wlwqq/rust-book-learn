mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

// 绝对路径：从crate根部开始，以字面量crate开头
// 相对路径：从当前模块开始，以 self、super 或当前模块的标识符开头。

// 使用use关键字简化函数调用
use crate::front_of_house::hosting;

// 当我们有个函数需要对外部人员来调用的时候，需要一层一层的调用到具体函数
// 如果不想让别人看到我们的mod层级，则使用pub use 将需要暴露出去的函数重导出
pub use crate::front_of_house::hosting::add_to_waitlist;
// 定义函数
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 使用use引入作用域后调用函数
    hosting::add_to_waitlist();

    // 直接使用pub use 重导出的函数
    add_to_waitlist();
}
/*
crate
 └── front_of_house
     ├── hosting
         ├── add_to_waitlist
         └── seat_at_table
*/

// super寻找父模块
fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // fix_incorrect_order在back_of_house模块，使用super找back_of_house的父模块
        // 这里的父模块就是crate根了
        super::serve_order();
    }

    fn cook_order() {}

    // 结构体设置为pub后，内部的字段依然是私有的，需要按需设置为pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
}
