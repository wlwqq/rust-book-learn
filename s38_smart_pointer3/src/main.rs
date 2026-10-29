use std::{cell::RefCell, rc::Rc};
fn main() {
    // 1 Rc 引用计数指针记录了值上关联所有者，默认情况下通过Rc只能读值，不能改值
    // 借用规则要求在存在不可变引用的时候，不允许修改，这是语言层面上的承诺：有人读的时候，数据不能改变
    // 有些时候需要修改值后再去读，就需要使用RefCell<T>指针，配合Rc<T>指针可以实现有人读，有人写
    // 但是需要注意，这依然不能打破借用规则，有读的时候不能有写，但是可以打破Rc只能读不能写的规则
    // 并且还有一点需要注意，借用规则的编译期间检查不对RefCell起作用，只在运行期检查，一旦违反规则则panic
    /*
    代码1
    let mut s = "wlh".to_string();
    let s1 = &s;
    let s2 = &mut s;
    println!("s1 = {}", s1);

    代码2
    let mut s = "wlh".to_string();
    let s1 = &s;
    s.push_str("sss");
    println!("s1 = {}", s1);

    不可变引用s1的使用范围内存在可变引用s2，或者存在所有者对数据修改，编译都会直接报错
    这里要注意的点是：引用的规则是Rust在语言层面的承诺，只要有任何不可变引用在，所有者/可变引用都会被限制
    引用的生命周期是使用周期, 不是粗暴的到作用域结束
     */

    // s 和 user1 共享了所有权，这个和上面的常规变量的独占所有权是不一样的
    // 常规变量作为所有者可以当作可变引用来看待，但是共享所有者只是当作所有者计数+1延长数据的释放时间
    // user1.borrow_mut().push_str("world");这一行执行完后可变引用的就还了
    // println!("s = {}", s.borrow()); 这一行是不可变借用，不会有问题
    let s = Rc::new(RefCell::new("hello".to_string()));
    let user1 = Rc::clone(&s);
    s.borrow_mut().push_str("world");
    println!("s = {}", s.borrow());

    // 下面代码会报错
    // let ss = s.borrow(); 这一行ss借用了值，ss的生命周期直到作用域代码}结束，这个和引用不一样，不是到最后一次使用的地方
    // user1.borrow_mut().push_str("world"); 在不可变借用的期间中想要可变借用，这里在运行时会panic
    // let s = Rc::new(RefCell::new("hello".to_string()));
    // let user1 = Rc::clone(&s);
    // let ss = s.borrow();
    // user1.borrow_mut().push_str("world");

    // 2 Box<T> Rc<T> RefCell<T> 的选用理由
    // Rc允许数据有多个所有者，Box和RefCell为单一所有者
    // Box允许在编译时执行不可变/可变借用检查，Rc仅允许在编译时执行不可变检查，RefCell在运行时执行不可变/可变检查
    // RefCell允许在运行时执行可变借用检查，所以可以在RefCell自身是不可变的情况下修改内部的值
}

/*
    let mut s = "wlh".to_string();
    let s1 = &s;
    println!("s1 = {}", s1);
    s.push_str(" hello");
    let s2 = &s;
    println!("s2={}", s2);
上面代码不会报错

既然普通变量错开时间就能实现“先读后改，改完再读”，那为什么还要费劲搞 Rc<RefCell> 呢？

答案在于：谁拥有数据的控制权？

普通变量（你的这段代码）：

数据（s）只有一个绝对的主人。

局限性：如果这个 s 被传到了函数 A 里面，或者被放进了某个结构体里，你（在 main 函数里）就失去了对它的直接控制权，没办法随时随地想改就改、想读就读了。

Rc<RefCell>（智能指针）：

数据没有单一的主人，大家都有“房产证”（Rc）。

强大之处：哪怕 user1 和 user2 分别在完全不同的函数、不同的模块、不同的业务逻辑里，只要它们手里拿着 Rc，它们就可以随时随地通过 borrow_mut() 去排队修改，而不需要把数据的所有权传来传去。

一句话总结
如果你的数据只在一个大括号（或同一个作用域）里折腾，老老实实用普通的 mut 和错开的借用，这是最高效、最原生的 Rust 方式。

只有当数据需要跨越不同的地方、被多方共同持有并随时动态修改（比如图结构、复杂的观察者模式、组件化架构）时，普通引用根本做不到，这时候才必须请出 Rc<RefCell>
 */
