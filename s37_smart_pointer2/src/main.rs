use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。
    // 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。
    // 如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
    // Rc<T>共享所有权，所有的Rc都是值的所有者，直到最后一个Rc离开作用域才能清理值

    // a: 5 -> 6 -> Nil
    // let a = Cons(5, Box::new(Cons(6, Box::new(Nil))));
    // b: 3 -> a
    // let b = Cons(3, Box::new(a));
    // c: 4 -> a 下面这行代码无法编译，因为Box是拥有数据的所有权的，a已经被move到b中
    // let c = Cons(4, Box::new(a));

    // Rc::clone 可以写成 a.clone()，但是一般a.clone()这种写法一般用在深拷贝上，所以为了区分建议写Rc::clone
    // Rc::clone 只会增加引用计数，不会有很大的性能开销，所以在排查问题时可以忽略Rc::clone看似是clone其实不是深拷贝

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // 不可变引用：Rc<T>对外默认只提供“只读的访问权限”，Rc也要符合借用规则
    //同一时刻不能出现多个可变引用，Rc默认clone出来的就是不可变的，不过拥有数据的所有权，可以说延长了数据的生命
    // 让数据不要过早的因为所有者离开作用域而释放内存
    let shared_num = Rc::new(100);
    let a = Rc::clone(&shared_num);
    let b = Rc::clone(&shared_num);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}
