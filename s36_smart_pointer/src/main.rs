use std::ops::Deref;
fn main() {
    // 截止目前，遇到了引用，切片，String，vector
    // 引用：8字节，存储变量的地址
    // 切片：16字节，存储第一个元素的地址和切片长度
    // String/vector：结构体，智能指针
    // 普通引用和智能指针的一个额外的区别是引用是一类只借用数据的指针；相反，在大部分情况下，智能指针 拥有 它们指向的数据。
    // 智能指针通常使用结构体实现，所以String类型其实由一个结构体<指针，长度，容量> 和 堆上具体的字符串内容构成
    // 智能指针区别于常规结构体的显著特性在于其实现了Deref和Drop trait，Deref trait允许智能指针结构体实例表现的像引用一样，
    // Drop trait允许我们自定义当智能指针离开作用域时运行的代码
    // 比较常见的智能指针
    // 1 Box<T> 用于在堆上分配值
    // 2 Rc<T> 一个引用计数类型，其数据可以有多个所有者
    // 3 Ref<T>和RefMut<T>，通过RefCell<T>访问

    // 1 Box<T> 允许将一个值放在堆上而不是栈上，留在栈上的是指向堆数据的指针
    let b = Box::new(5); // 5存储在堆上，b是存储在栈上的一个box，离开作用域时会释放栈上和堆上的数据
    println!("b={}", b);

    // 2 Deref trait将智能指针当作常规引用处理
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // 通过*解引用
    let x = Box::new(5);
    assert_eq!(5, *x); // 通过*解引用，这里解的是Box

    // 3 自定义智能指针
    let x = MyBox::new(5);
    // MyBox不能解引用，为了使用 * 解引用，需要实现Deref trait
    // assert_eq!(5, *x); // 报错 can't be dereferenced
    // 编译器默认只会对&类型解引用
    assert_eq!(5, *x); // 底层代码 *(x.deref())，先调用deref()获取引用，再通过*解引用

    // 4 函数和方法的隐式解引用强制转换
    // 解引用强制转换是Rust在函数或方法传参上的一种便利，解引用强制转换只能作用在实现了Deref trait的类型上
    // 解引用强制转换 将类型A隐式转换为另一种类型B的引用，只要A类型实现了Deref trait，并且关联类型是B类型
    let x = MyBox::new("wlh".to_string()); // 类型是MyBox<String>
    hello(&x); // &MyBox<String> 通过deref 转为 &String类型，&String在标准库中的deref转为 &str,最终满足hello函数的签名
    // 上面多次deref转换都是编译器在编译期间自动做的

    // 5 实现Drop trait进行用户自定义的清理代码
    let x = CustomSmartPointer {
        data: "wlh".to_string(),
    };
    let y = CustomSmartPointer {
        data: "huawei".to_string(),
    };
    println!("CustomSmartPointer...");
}

fn hello(name: &str) {
    println!("hello,{}", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 定义此trait的关联类型

    // 返回了一个引用，这样通过*就可以解引用了
    // 这里还有一个考量是大部分使用*的情况时，并不希望获取所有权
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("drop function execute, data={}", self.data);
    }
}
