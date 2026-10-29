use crate::my_mod::AveragedCollection;
fn main() {
    // 1 封装
    /*
    1.rust中默认都是私有的
    2.私有的东西只能在当前定义的mod内使用，或者定义的mod的子mod中使用
     */
    let mut ac = AveragedCollection::new();
    ac.add(5);
    ac.add(6);
    println!("avg = {}", ac.average());
}

mod my_mod {
    // 结构体是pub的，但是字段是private的
    // 计算vec的平均值
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        //
        pub fn new() -> AveragedCollection {
            AveragedCollection {
                list: vec![],
                average: 0.0,
            }
        }

        // 添加元素
        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        // 获取平均值
        pub fn average(&self) -> f64 {
            self.average
        }

        // 计算平均值
        fn update_average(&mut self) {
            if !self.list.is_empty() {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }
}
