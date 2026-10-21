// #[cfg(test)] 表示这是一个单元测试模块，只在执行cargo test的时候才会执行代码
// 单元测试和要测试的代码应该放在一起，cargo build时不会将测试模块的代码打进二进制中
// 不仅节约构建时间，也减小了构建的产出物大小
// 和单元测试相对的是集成测试，集成测试测试的是整个lib库的功能，只能访问lib中pub的功能
// 集成测试表示的是整个库是否能正常工作，通常需要单独一个地方存放代码
// 集成测试需要在根目录下创建一个tests目录，该目录下可以写任意个文件，每一个文件作为独立的外部 crate 编译
#[cfg(test)]
mod unit_tests {

    // #[test]表示这是一个测试函数
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("make this test fail");
    }

    use crate::{Rectangle, greeting};
    #[test]
    fn test_can_hold() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_add_two() {
        // 这里区分下 use是为了简化调用的路径问题
        // 子mod是可以直接访问父mod的函数，不需要把add_two函数设置为pub
        assert_eq!(4, crate::add_two(2));
    }

    // 自定义失败信息
    #[test]
    fn test_greeting() {
        let result = greeting("wlh");
        // 通过带占位符的格式字符串和greeting函数的返回值，在执行cargo test之后
        // 输出的信息就会带上自定义的信息，更加友好
        assert!(result.contains("wlh"), "greeting return={}", result);
    }

    // 在函数头上新增#[should_panic]属性，在函数出现panic时测试通过，否则不通过

    // 使用Result<T, E>作为测试函数的返回值 而不是直接panic!
    #[test]
    fn it_works_v2() -> Result<(), String> {
        if 2 + 3 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /*
    cargo test  的参数区分 -- 之前和之后
    -- 之前的参数：传递给cargo test的参数
    -- 之后的参数：传递给代码的参数

    1.默认cargo test是多线程并发的，可以通过cargo test -- --test-threads=1 控制为1
    2.在测试函数中println!输出在控制台的内容在测试通过时不会打印，只有在测试失败时才会打印，通过 cargo test -- --show-output
    3.指定名字运行指定的测试 cargo test 测试函数名，或者是 cargo test xxx,任何包含xxx的函数都会执行
    4.在函数上添加#[ignore]后，使用cargo test就不会运行这个函数，如果只希望运行这些函数，那么可以使用cargo test -- --ignored
     */
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}", name)
    "hello".to_string()
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(i: i32) -> i32 {
    i + 2
}
