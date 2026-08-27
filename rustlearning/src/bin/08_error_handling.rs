// ============================================================
// 第8章 · 错误处理：panic!、Result 与 ? 运算符
// ============================================================
// 运行: cargo run --bin 08_error_handling
// 学习要点:
//   1. 两类错误：不可恢复 panic! vs 可恢复 Result<T, E>
//   2. unwrap / expect 的适用场景
//   3. match 处理 Result
//   4. ? 运算符：错误的自动传播（vs 异常的 throw）
//   5. 自定义错误类型与 From 转换
//   6. main 直接返回 Result
// ============================================================

use std::fs;
use std::num::ParseIntError;

// ==== 1. Result 是什么？====
// 定义（简化）：enum Result<T, E> { Ok(T), Err(E) }
// 对比异常：C 没异常只能返回错误码 int（信息量可怜）；Java/Python 异常能携带堆栈但
// 控制流隐身——函数签名看不出会不会炸。Rust 把成败写进返回类型：
// 看到 Result 就知道可能失败，编译器还强制你处理（不处理会警告 unused_must_use）。

// 用 match 手动处理（最啰嗦但最直白的方式）
fn parse_age(text: &str) -> i32 {
    match text.parse::<i32>() {
        Ok(age) => age,
        Err(e) => {
            eprintln!("解析失败: {e}，使用默认 18");
            18
        }
    }
}

// ? 运算符版：出错就提前把 Err return 出去，成功则解包出 T
// 等价于 Python 的「不捕获让它抛」或 Go 的 if err != nil { return err }——一行搞定后者
#[allow(dead_code)]
fn parse_age_q(text: &str) -> Result<i32, ParseIntError> {
    let age: i32 = text.parse()?;      // Err 时直接 return Err(e)，Ok 时取出 i32
    Ok(age * 2)                        // 业务逻辑不再被错误处理打断
}

// ==== 2. 自定义错误类型 ====
// 小项目可以像 Java 自定义 Exception 一样定义自己的错误枚举：
use std::fmt;
use std::io;

#[derive(Debug)]
enum ConfigError {
    NotFound(String),                 // 业务错误：文件不存在
    Io(io::Error),                    // IO 错误：包装标准库错误
    InvalidPort(ParseIntError),       // 解析错误
}

// 实现 From 让 ? 自动做错误类型转换（类似异常体系里的子类向上转型/包装）
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::InvalidPort(e)
    }
}

impl From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        if e.kind() == io::ErrorKind::NotFound {
            return ConfigError::NotFound("路径不存在".to_string());
        }
        ConfigError::Io(e)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::NotFound(path) => write!(f, "配置文件不存在: {path}"),
            ConfigError::Io(e) => write!(f, "IO 错误: {e}"),
            ConfigError::InvalidPort(e) => write!(f, "端口号非法: {e}"),
        }
    }
}

// 读文件并解析端口：两种不同来源的错误都靠 From 被 ? 自动转换、一路传播到顶层统一处理
// 对比 Python: try: ... except FileNotFoundError ... except ValueError
//      Rust:   每个 fallible 调用一个 ?，错误类型衔接由编译器检查
fn load_port(path: &str) -> Result<u16, ConfigError> {
    let content = fs::read_to_string(path)?;     // io::Error --From--> ConfigError
    let port: u16 = content.trim().parse()?;     // ParseIntError --From--> ConfigError
    Ok(port)
}
// 真实项目常用 thiserror 库自动生成这些 impl 样板；大型应用则用 anyhow 动态错误。

fn main() -> Result<(), ConfigError> {
    // ==== 3. panic!：不可恢复，程序直接崩 ====
    // 对比 C 的 assert 失败 abort()、Java 的 Error、Python 未捕获异常。
    // 用于 bug（数组越界、断言失败），不要用它处理用户输入！
    // panic!("故意崩溃");   // ← 取消注释看崩溃输出（回溯信息需要 RUST_BACKTRACE=1）

    // 数组越界在 Rust 是 panic（debug 下必现），而 C 是未定义行为——可能静默读脏数据
    let _arr = [1, 2, 3];
    // _arr[10];            // ← 取消注释：index out of bounds panic

    // ==== 4. unwrap / expect：原型期的偷懒写法 ====
    let n: i32 = "42".parse().unwrap();               // Err 则 panic
    let m: i32 = "43".parse().expect("必须是合法数字");  // 同上，但附带自定义报错文案
    println!("unwrap={n}, expect={m}");
    // 原则：示例代码/测试里随便用；生产代码路径上应该用 ? 或 match。

    // ==== 5. match 处理 ====
    println!("parse_age('abc') = {}", parse_age("abc"));
    println!("parse_age('25')  = {}", parse_age("25"));

    // ==== 6. 组合子：不用 match 也能优雅处理 ====
    // 注意：map/and_then 等会消费（move）Result 本体；只想看一眼用 as_ref()
    let ok: Result<i32, _> = "100".parse::<i32>();
    println!("map 翻倍: {:?}, as_ref+map_or 默认值: {}",
             ok.as_ref().map(|v| v * 2), ok.as_ref().unwrap_or(&-1));
    let bad: Result<i32, _> = "x".parse::<i32>();
    println!("错误值 unwrap_or_else 兜底: {}",
             bad.unwrap_or_else(|e| { eprintln!("原始错误: {e}"); 0 }));

    // ==== 7. ? 链式传播 + main 收尾 ====
    // 先造一个配置文件供读取
    fs::write("demo_config.txt", "8080").expect("写临时配置失败");
    match load_port("demo_config.txt") {
        Ok(port) => println!("加载端口成功: {port}"),
        Err(e) => eprintln!("加载失败: {e}"),
    }
    // main 本身返回 Result：直接用 ? 传播，Err 会打印 Debug 并以非零码退出——
    // 相当于 Python 的 sys.exit(main()) 包装模式
    let port = load_port("demo_config.txt")?;
    println!("main 里用 ? 拿到的端口: {port}");

    // ==== 8. Box<dyn Error>：懒得定义错误类型的通用方案 ====
    fn quick_read(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let s = fs::read_to_string(path)?;    // 各种错误都塞进 trait 对象
        Ok(s)
    }
    match quick_read("demo_config.txt") {     // Box<dyn Error> 与 ConfigError 类型不同，
        Ok(s) => println!("quick_read 结果长度: {}", s.len()),   // 不能用 ? 混用——只能 match
        Err(e) => eprintln!("quick_read 失败: {e}"),
    }
    let _ = parse_age_q("7");                 // 顺带跑一下 ? 版本避免 dead_code 警告

    /*
     * ---- 一句话总结 ----
     * bug 用 panic，预期内的失败一律 Result<T,E>；
     * ? 是「出错就返回」的语法糖，配合 From 实现跨错误类型传播；
     * 和异常相比：失败可能性写在函数签名里，调用方不可能「忘记 catch」。
     */
    fs::remove_file("demo_config.txt").ok();   // 清理现场，忽略清理错误
    Ok(())
}
