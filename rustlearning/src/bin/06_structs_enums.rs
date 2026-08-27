// ============================================================
// 第6章 · 结构体与枚举：自定义数据类型
// ============================================================
// 运行: cargo run --bin 06_structs_enums
// 学习要点:
//   1. 结构体：命名字段 / 元组 / 单元三种形态
//   2. impl 方法与关联函数（Self、new）
//   3. 结构体更新语法 .. 与所有权联动
//   4. 枚举携带数据：代数数据类型（vs C 枚举 / Python 类）
//   5. Option<T>：空值的安全替代品
//   6. Debug/PartialEq 等 derive 派生
// ============================================================

// ==== 1. 命名字段结构体 ====
// 对比 C 的 struct：长得一样，但 Rust 的可以带方法（通过 impl）；
// 对比 Python 的 class：字段必须声明类型，编译期检查，不用写 __init__。
#[derive(Debug, Clone, PartialEq)]      // derive 自动实现调试打印/克隆/相等比较
struct User {
    name: String,
    email: String,
    age: u8,
    active: bool,
}

impl User {
    // 关联函数：没有 self，用 :: 调用——惯用的「构造函数」。
    // 对比 C++ 的构造函数；Python 的 __init__/classmethod。
    fn new(name: &str, email: &str) -> Self {          // Self 是 impl 块类型的别名
        Self {
            name: name.to_string(),     // 字段名和变量名相同时可简写成 name,
            email: email.to_string(),
            age: 18,
            active: true,
        }
    }

    // 方法：第一个参数是 self 的变体
    // &self 只读借用（最常用）| &mut self 可变借用 | self 拿走所有权（少见）
    fn birthday(&mut self) {
        self.age += 1;
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // 拿走所有权的方法：调用后原变量不能再用
    fn into_summary(self) -> String {
        format!("{}({})", self.name, self.age)
    }
}

// ==== 2. 元组结构体：给元组起名字 ====
struct Point(i32, i32);
struct Millimeter(i32);                 // 新类型模式：用类型区分单位，防止混用

impl Point {
    fn distance(&self) -> f64 {
        ((self.0 * self.0 + self.1 * self.1) as f64).sqrt()
    }
}

// ==== 3. 枚举：每个变体可以携带不同数据 ====
// 对比 C 的 enum：只是整数常量集合，想带数据得配 union（还不安全）；
// 对比 Python 的 Enum + dataclass 组合。Rust 一个枚举全搞定——这就是「代数数据类型」，
// 和第7章的模式匹配是天作之合。
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },             // 变体带具名字段
    Rectangle(f64, f64),                // 变体带匿名元组
    Triangle,                           // 变体不带数据
}

impl Shape {
    fn area(&self) -> f64 {
        match self {                    // match 必须穷尽所有变体，加新变体时编译器提醒你补逻辑
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle => 0.0,     // 示例占位
        }
    }
}

fn main() {
    // ==== 4. 创建与访问 ====
    let mut u1 = User::new("小明", "ming@example.com");
    println!("创建: {:?}", u1);                  // 需要 Debug 才能 {:?} 打印
    u1.birthday();
    println!("过生日后 age={}，成年={}", u1.age, u1.is_adult());

    let summary = u1.into_summary();             // 所有权被拿走
    println!("摘要: {summary}");
    // println!("{:?}", u1);                     // ← 编译错误：u1 已被消费（move 进了方法）

    // ==== 5. 结构体更新语法 .. ====
    let base = User::new("模板", "tpl@example.com");
    let u2 = User {
        name: String::from("小红"),
        ..base                                   // 其余字段从 base 复制/移动过来
    };
    println!("u2 = {:?}", u2);
    // 注意所有权细节：String 字段是 move 出来的！base.email 已经失效：
    println!("base 还能用 name: {}", base.name);
    // println!("{}", base.email);               // ← 编译错误：email 被 move 进了 u2
    // 对比 C 的指定初始化器 (C99) .email=...；Python 的 dataclass(replace=True)。

    // ==== 6. 元组结构体与新类型模式 ====
    let p = Point(3, 4);
    println!("点 (3,4) 到原点距离 = {:.2}", p.distance());
    let width = Millimeter(500);
    println!("宽度 = {} 毫米", width.0);
    // fn f(w: Millimeter) {}; f(Point(500, 0));  // ← 编译错误！数值相同但类型不同
    // 对比 C 的 typedef int mm_t——typedef 只是别名不产生新类型，Rust 新类型是真隔离。

    // ==== 7. 枚举与匹配 ====
    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle,
    ];
    for s in &shapes {
        println!("{s:?} 面积 = {:.2}", s.area());
    }

    // ==== 8. Option<T>：消灭空指针 ====
    // 定义：enum Option<T> { Some(T), None } —— 「有值」和「没值」都是普通值。
    // 对比 C：NULL 解引用=段错误/UB；Python：AttributeError: 'NoneType' object...
    // Java：NullPointerException 十亿刀教训。Rust 没有 null，可能缺席的值类型上就写着 Option。
    fn find_user(name: &str) -> Option<User> {
        if name == "小明" { Some(User::new("小明", "ming@example.com")) } else { None }
    }
    let found = find_user("小明");
    match found {
        Some(u) => println!("找到用户: {} <{}>", u.name, u.email),
        None => println!("查无此人"),
    }
    // 常用组合子：unwrap_or 默认值、map 转换、and_then 链式（像 Python 的 or/map）
    let none_case = find_user("路人");
    println!("默认邮箱: {}", none_case.unwrap_or(User::new("游客", "guest@x.com")).email);

    /*
     * ---- 一句话总结 ----
     * struct 存数据 + impl 加行为；关联函数当构造器；
     * enum 变体可携带异构数据，配 match 实现安全的分支分发；
     * Option 取代 null，「可能为空」从文档约定升级为编译期契约。
     */
}
