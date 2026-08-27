// ============================================================
// 第13章 · 模块系统与测试：组织代码的正确姿势
// ============================================================
// 运行: cargo run --bin 13_modules_testing
// 学习要点:
//   1. mod 定义模块、pub 控制可见性、use 引入路径
//   2. 模块树与路径（crate:: / super::）
//   3. 单元测试 #[test] 与断言宏
//   4. should_panic 测试错误路径
//   5. 文档测试 doctest
//   6. cargo test 工作流
// ============================================================

// ==== 1. 在单文件里定义模块（真实项目会拆到 src/*.rs，机制相同）====
mod kitchen {
    // 默认一切私有！只有 pub 的才能被外部访问。
    // 对比 C 的头文件+static：声明实现分离、链接期才报错；
    // Rust 可见性是编译期语言特性，不靠头文件。
    // 对比 Python：_前缀纯约定；Rust 不写 pub 外面就是看不见。

    pub fn cook(dish: &str) -> String {
        format!("烹饪 {dish}（用了{}份{}）", wash_vegetables(), secret_sauce())
        // 同模块内部可以随便调用私有项
    }

    fn wash_vegetables() -> &'static str {       // 私有辅助函数
        "洗净的蔬菜"
    }

    fn secret_sauce() -> &'static str {
        "秘制酱料"                               // 外部无法直接调用
    }

    pub mod appliance {                          // 嵌套模块
        pub fn microwave(food: &str) -> String {
            format!("微波炉加热 {food}")
        }
    }
}

use kitchen::appliance::microwave;               // use 绝对路径引入
// use crate::kitchen;                           // crate:: 从二进制根开始（这里即本文件）
// 对比 Python: from package.module import func —— 几乎一模一样的手感

// ==== 2. super:: 访问父模块 ====
mod pantry {
    pub const RICE: &str = "大米";

    pub fn today_menu() -> String {
        // super 指父模块（这里就是文件根），类似文件系统的 ..
        format!("今日主食: {}", super::kitchen::cook("蛋炒饭"))
    }
}

fn main() {
    // ==== 3. 路径调用 ====
    println!("{}", kitchen::cook("红烧肉"));
    println!("{}", microwave("剩饭"));
    println!("{}", pantry::today_menu());
    println!("库存常量: {}", pantry::RICE);      // pub 常量跨模块访问

    /*
     * ---- 真实项目的模块布局 ----
     * src/
     * ├── main.rs          二进制入口（crate root）
     * ├── lib.rs           库入口（可选）
     * ├── config.rs        mod config {...}
     * └── utils/
     *     ├── mod.rs       （或 utils.rs + utils/ 子目录，2024 版推荐后者）
     *     └── net.rs
     * main.rs 里写 mod config; mod utils; 即把文件挂进模块树——
     * 「声明」和「定义」分离度远低于 C 头文件体系。
     */

    demo_assert_macros();
}

// ==== 4. 单元测试：#[cfg(test)] 只在测试时编译 ====
// 对比 C：没有内置测试框架，得拉 Check/CUnit 第三方库；
// Python 有 unittest/pytest；Rust 内置且零依赖，cargo test 一条命令。

#[allow(dead_code)]                              // 只被测试用到，普通构建下标记豁免
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 故意留一个「有 bug」的除法用于演示测试失败信息
#[allow(dead_code)]
fn div(a: i32, b: i32) -> i32 {
    a / b                                        // b=0 时 panic——正好被测试抓住
}

#[cfg(test)]
mod tests {
    use super::*;                                // 引入父模块（被测代码）的所有项

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);                // assert_eq!/assert_ne! 自动打印两侧值
        assert!(add(-1, 1) == 0);                // assert!(布尔表达式)
    }

    #[test]
    fn test_div_by_zero_panics() {               // 测「应该崩溃」的场景
        let result = std::panic::catch_unwind(|| div(10, 0));
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]                              // 更直接的写法：期望 panic 才算通过
    fn test_div_zero() {
        let _ = div(1, 0);
    }

    #[test]
    #[ignore = "演示如何临时跳过"]                 // cargo test -- --ignored 才会跑
    fn expensive_test() {
        assert!(true);
    }
}

// ==== 5. 文档测试：注释里的示例代码也会被执行 ====
/// 把温度从摄氏转华氏。
///
/// # 示例（Examples）
/// ```
/// # use 不会出现在文档里，井号行只参与编译;
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```
/// cargo test 会把它当用例跑——文档永远不会过期！这是 Rust 生态的独特武器，
/// 对应 Python doctest 但集成度和运行速度好得多。
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn demo_assert_macros() {
    println!("celsius_to_fahrenheit(37) = {}", celsius_to_fahrenheit(37.0));

    // 运行时断言也常用（debug 构建默认开启，release 可关）：
    let version = (1, 97);
    debug_assert!(version >= (1, 70), "Rust 版本过低");
    println!("版本检查通过: {:?}", version);

    /*
     * ---- cargo test 输出解读 ----
     * running 4 tests
     * test tests::test_add ... ok
     * ...
     * test result: ok. 4 passed; 0 failed
     *
     * 常用命令：
     *   cargo test                     跑全部（含文档测试）
     *   cargo test test_add            按名字过滤
     *   cargo test -- --nocapture      显示测试中的 println! 输出
     */
}
