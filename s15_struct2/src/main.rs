fn main() {
    // 1 元组结构体：有结构体名称，字段没有名称的结构体
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // 2 类单元结构体：没有任何字段的结构体
    let subject = AlwaysEqual;
}

// 下面是两个元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体：定义某个类型用来实现trait，但是不需要存储任何数据
struct AlwaysEqual;
