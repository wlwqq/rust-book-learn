use std::fmt::{Debug, Display};

fn main() {
    // 1 提取函数减少重复代码
    let number_list = vec![1, 4, 5, 7, 10, 0];
    let large = largest(&number_list);
    println!("largest = {}", large);

    // 2 在函数中使用泛型
    let large = largest_v2(&number_list);
    println!("largest_v2 = {}", large);

    // 3 在结构体中定义泛型
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // 4 枚举中定义泛型
    // 5 方法中定义泛型

    // 6 trait定义共享的行为
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("read: {}", tweet.more());

    // 7 trait作为参数,trait bound

    // 8 因为trait bound作用在泛型上，所以可以约束特定类型，给特定类型实现特定方法
    let a = OnePoint::new(5);
    let b = OnePoint::new(4);
    println!("a > b = {}", a.great_than(&b));
    let c = OnePoint::new(Point { x: 1, y: 2 });
    let d = OnePoint::new(Point { x: 1, y: 2 });
    // 报错，因为Point类型没有实现PartialOrd
    // println!("c > d = {}", c.great_than(&d)) // doesn't satisfy `Point<{integer}, {integer}>: PartialOrd`

    // 9 Blanket Implementation 覆盖实现，他的意思是为某些特定的类型，实现特定的接口
    // 例如标准库中有impl<T: Display> ToString for T {有to_string()方法的实现}
    // 那么任何实现了Display接口的类型T都实现了ToString接口
}

fn largest(list: &[i32]) -> i32 {
    let mut large = list[0];

    for &i in list {
        if i > large {
            large = i;
        }
    }

    large
}

fn largest_v2<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut large = list[0];

    for &i in list {
        if i > large {
            large = i;
        }
    }

    large
}

struct Point<T, U> {
    x: T,
    y: U,
}

// impl后面的T,U是结构体自身的泛型
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    // 方法后面定义的是方法自己的泛型
    // 这个方法可以将两个类型毫不相干的Point组合成一个新的Point
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 定义trait
pub trait Summary {
    // 方法签名使用分号结尾
    fn summarize(&self) -> String;

    // 默认实现的方法
    fn more(&self) -> String {
        "Read More...".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},{}", self.headline, self.location)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

// 下面这种定义方式叫做trait bound，impl Summary其实是下面这种方式的语法糖
fn notify_v2<T: Summary>(item: T) {
    println!("{}", item.summarize());
}

// 使用 + 指定多个trait bound
fn notify_v3<T: Summary + Display>(a: T, b: T) {
    println!("notify_v3");
}

// 使用where简化trait bound
fn notify_v4<T, U>(a: T, b: U) -> i32
where
    T: Summary + Display,
    U: Clone + Debug,
{
    5
}

struct OnePoint<T> {
    x: T,
}

impl<T> OnePoint<T> {
    fn new(x: T) -> OnePoint<T> {
        OnePoint { x: x }
    }
}

impl<T: PartialOrd> OnePoint<T> {
    fn great_than(&self, other: &OnePoint<T>) -> bool {
        self.x > other.x
    }
}
