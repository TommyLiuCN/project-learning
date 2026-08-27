// ============================================================
// 第9章 · 泛型、Trait 与生命周期：抽象三板斧
// ============================================================
// 运行: cargo run --bin 09_generics_traits_lifetimes
// 学习要点:
//   1. 泛型函数与泛型结构体（单态化：编译期生成专用代码）
//   2. trait 定义与实现（vs 接口）
//   3. 默认方法与 derive 宏
//   4. trait 约束：where 子句、impl Trait
//   5. trait 对象 dyn Trait（动态派发 vs 泛型静态派发）
//   6. 生命周期标注初探
// ============================================================

use std::fmt::Display;

// ==== 1. 泛型函数 ====
// T: PartialOrd 是 trait 约束：要求 T 支持 < > 比较。
// 对比 C 的宏黑魔法或 void* 强转（无类型安全）；C++ template 无约束直到 concepts；
// Python 靠鸭子类型运行时炸。Rust 把能力写进签名，调用前编译期就验证。
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// ==== 2. 泛型结构体 + 多约束 ====
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Display> Pair<T> {         // where 也可写成多约束
    fn max(&self) -> &T {
        if self.first >= self.second { &self.first } else { &self.second }
    }
}

// ==== 3. trait：Rust 的「接口」（但更强）====
// 对比 Java interface：语法神似；区别① 可以给已有类型实现已有 trait（如给 i32 加方法）；
// 区别② 可以带默认实现；区别③ 支持运算符重载等（Java 接口做不到）。
// 对比 Python ABC：编译期强制实现，而不是运行时才报错。
trait Animal {
    fn name(&self) -> String;

    // 默认实现：不重写也能用（Java 8+ default 方法的对应物）
    fn intro(&self) {
        println!("我是 {}", self.name());
    }
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> String { "旺财".to_string() }
    // 不重写 intro()，享受默认实现
}

impl Animal for Cat {
    fn name(&self) -> String { "咪咪".to_string() }
    fn intro(&self) {                            // 重写默认实现
        println!("本喵叫 {}，勿扰", self.name());
    }
}

// trait 约束的函数：两种等价写法
fn make_speak<A: Animal>(a: &A) {
    a.intro();
}
fn make_speak_impl(a: &impl Animal) {            // impl Trait 简写，语义相同
    a.intro();
}

// ==== 4. 运算符重载：也是 trait！====
// std::ops::Add 定义了 + 号行为。对比 C++ operator+ 直接重载；Python __add__。
// Rust 统一走 trait：想要 + 就实现 Add，想要 == 就实现 PartialEq。
#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;                          // 关联类型：指定加法结果的类型
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// ==== 5. 生命周期：引用有效范围的标注 ====
// 大多数时候编译器能自动推断（省略规则）；返回引用且参数有多个引用时需要标注。
// 'a 不是延长任何生命，只是告诉编译器「返回值的存活不超过输入中最短的那个」。
// 对比 C：返回悬垂指针编译器一声不吭；Rust 标注错了直接编译失败。
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    // ==== 泛型使用 ====
    let nums = [3, 7, 1, 9, 4];
    let words = ["apple", "pear", "banana"];
    println!("最大数字: {}", largest(&nums));
    println!("最长单词: {}", largest(&words));
    // largest(&["a", 1]);      // ← 编译错误：数组元素类型必须一致，混装不行

    let p = Pair { first: 10, second: 25 };
    println!("Pair 较大值: {}", p.max());

    // ==== trait 使用 ====
    make_speak(&Dog);
    make_speak_impl(&Cat);
    Dog.intro();                                 // 默认实现直接可用

    // 动态派发 dyn Trait：运行时查虚表（像 Java/C++ 的接口指针/virtual）
    // 泛型则是静态派发：编译期为每个具体类型生成代码（单态化），零开销但二进制变大
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    println!("--- 动物园开饭 ---");
    for a in &zoo {
        a.intro();                               // 运行时决定调谁的 intro
    }
    // 类比 C 的函数指针表/手写 vtable；Rust 一层 dyn 就搞定且类型安全

    // ==== 运算符重载 ====
    let v = Vec2 { x: 1.0, y: 2.0 } + Vec2 { x: 10.0, y: 20.0 };
    println!("向量加法: {v:?}");

    // ==== 生命周期 ====
    let s1 = String::from("长长的字符串一");
    let result;
    {
        let s2 = String::from("短");
        result = longest(s1.as_str(), s2.as_str());
        println!("较长者: {result}");             // ✓ 在 s2 存活期内使用
    }
    // println!("{result}");     // ← 取消注释编译错误：result 可能指向已释放的 s2
    //                             //   C 里这就是悬垂指针 UB，Rust 编译期拦截

    let static_msg: &'static str = "我活在整个程序期间";   // 'static：最长的生命周期
    println!("{static_msg}");

    /*
     * ---- 一句话总结 ----
     * 泛型 = 编译期多态（快），dyn Trait = 运行期多态（灵活），按需选择；
     * trait 是接口+默认实现+运算符重载的统一抽象层；
     * 生命周期是给引用画的「有效期凭证」，编译器凭它消灭悬垂引用。
     */
}
