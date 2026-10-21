use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    // 假设是用户输入的数据
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    // 使用闭包的好处是将一段逻辑 赋值给某个变量，通过这个变量执行逻辑
    generate_work(simulated_user_specified_value, simulated_random_number);
    println!("========");
    // 如果要将闭包放到一个结构体中，并且缓存闭包的结果，需要定义出闭包的类型
    // 闭包的类型需要通过泛型和trait bound来定义，所有的闭包都实现了trait Fn/FnMut/FnOnce三个其中的一个
    generate_workout_v2(simulated_random_number, simulated_random_number);
    println!("========");
    // 更加完善的CacherV2
    let mut cacherV2 = CacherV2::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!("{}", cacherV2.value(2));
    println!("{}", cacherV2.value(3));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 判断用户的锻炼强度 < 25, 小于25就适当仰卧起坐等，大于25，随机数是3就休息，否则就跑步
fn generate_work(intensity: u32, random_number: u32) {
    // 下面是一个闭包的定义，闭包通常的使用场景是在小范围上下文中起作用，编译器推断出类型
    // 闭包存储在变量中被使用，|参数1,参数2| {执行的语句}
    // 编译器一旦推断出闭包中参数和返回值的类型后，该闭包的类型就固定了，不能作用在别的入参类型上了
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    //
    if intensity < 25 {
        println!("Today do {} pushups!", expensive_closure(intensity));
        println!("Next do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn generate_workout_v2(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Cach的值是私有的
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,     // 闭包计算逻辑
    value: Option<u32>, // 闭包的结果
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 关联函数，创建结构体
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    // 获取闭包的值，曾经计算过就缓存
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

struct CacherV2<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> CacherV2<T>
where
    T: Fn(u32) -> u32,
{
    fn new(cal: T) -> CacherV2<T> {
        CacherV2 {
            calculation: cal,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}
