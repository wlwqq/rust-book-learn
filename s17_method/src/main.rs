fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // rect1.area() 这一行存在rust中的 自动引用和解引用
    // area()声明的定义是&self，但是这里使用的是rect1调用
    // rust帮我们自动引用了，等同于(&rect1).area()
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 使用关联函数创建结构体实例
    let rect1 = Rectangle::square(5);
    println!("rect1 = {:#?}", rect1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle表示{}中的内容都和Rectangle相关
impl Rectangle {
    // &self 是 self:&Self的缩写，Self是impl块类型的别名
    // 方法的第一个参数必须是self，所以可以简写为&self
    // &self 不可变借用
    // &mut self 可变借用
    // self 获取所有权
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数：第一个参数不用是self，因为不作用在结构体实例上，而是作用在impl块的类型上
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
