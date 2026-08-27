// ============================================================
// 第2章 · 变量与基本类型：不可变默认、遮蔽与类型系统
// ============================================================
// 运行: cargo run --bin 02_variables
// 学习要点:
//   1. let 默认不可变，mut 才可变（与 C/Python 相反的哲学）
//   2. 遮蔽 shadowing：同名重新声明
//   3. const 常量与类型标注
//   4. 标量类型：整数、浮点、bool、char（Unicode）
//   5. 类型转换 as 与字符串解析 parse
//   6. 整数溢出的行为
// ============================================================

// const：编译期常量，必须标注类型；对比 C 的 #define / const，Python 没有真常量（约定全大写）
const MAX_POINTS: u32 = 100_000;      // _ 是数字分隔符，纯可读性
static GREETING: &str = "我是 static 静态变量";  // static 有固定内存地址，类似 C 的全局变量

fn main() {
    // ==== 1. 不可变是默认 ====
    let x = 5;
    // x = 6;            // ← 取消注释会编译失败！cannot assign twice to immutable variable
    println!("x = {x}（默认不可变）");
    let mut y = 5;
    y += 1;              // 声明为 mut 后才能改
    println!("y = {y}（mut 可变）");
    // 对比 C: int x = 5 默认可改，要 const int 才只读——Rust 反过来，逼你显式声明可变性，
    // 读代码时一眼知道哪些状态会变。Python 更松：连常量都没有，全靠自觉。

    // ==== 2. 遮蔽 shadowing ====
    let spaces = "   ";          // &str 字符串字面量
    let spaces = spaces.len();   // 同名重新声明，类型都可以变！&str → usize
    println!("遮蔽后 spaces = {spaces}");
    // 和 mut 的区别：遮蔽是「新建一个变量」，还能改变类型；
    // mut 是「原地修改」，类型必须始终一致。函数处理中间值时非常好用。

    // ==== 3. 整数类型 ====
    // i8/i16/i32/i64/i128/isize 与对应无符号 u 系列。 isize/usize 跟平台指针等宽。
    // 对比 C 的 int/long 平台相关宽度让人头秃；Rust 宽度写死在类型名里。
    let a: i32 = -42;
    let b: u8 = 255;
    let c = 9_0000_i64;          // 后缀直接标注类型
    println!("i32={a}, u8={b}, i64={c}, usize(数组下标常用)={}", spaces);
    // 默认推断为 i32（性能与安全的折中）

    // ==== 4. 浮点、布尔、字符 ====
    let f1 = 2.5_f64;            // f64 默认且精度更高（现代 CPU 上不比 f32 慢）
    let flag: bool = true;
    let heart = '❤';             // char 是 4 字节 Unicode 标量！
    let c_char = 'A';
    println!("f64={f1}, bool={flag}, char='{heart}' '{c_char}'");
    // 对比 C 的 char 只有 1 字节存 ASCII；Python3 的 str 天生 Unicode。
    // 注意：Rust 的 '❤' 是 char，但 "中文字符串".len() 返回的是「字节数」不是字符数！

    // ==== 5. 类型转换：没有隐式转换！====
    let big: i64 = 300;
    let small = big as i8;       // as 强制转换，高位截断（300 超出 i8 范围会绕回）
    println!("i64 300 as i8 = {small}（截断绕回，和 C 的强转一样危险）");
    // let z: i32 = big;         // 编译错误！Rust 不做隐式窄化转换，C 会悄悄帮你转然后埋雷
    let parsed: i32 = "123".parse().expect("解析失败");  // 字符串→数字，可能失败所以返回 Result
    println!("parse 得到 {parsed}");

    // ==== 6. 整数溢出行为 ====
    let ov: u8 = 250u8.wrapping_add(10);      // 显式环绕，回到 4
    println!("wrapping_add: 250+10(u8) = {ov}");
    // debug 构建：普通 + 溢出会 panic；release 构建：静默环绕（同 C）。
    // 想明确处理可用 checked_add(返回 Option)/saturating_add(饱和) 等：
    println!("checked_add = {:?}, saturating_add = {}", 250u8.checked_add(10), 250u8.saturating_add(10));

    // ==== 7. 元组与单元类型 ====
    let tup: (i32, f64, char) = (500, 6.4, 'z');     // 复合类型，长度固定
    let (t1, t2, t3) = tup;                          // 解构，像 Python 的 a, b, c = ...
    println!("解构: {t1} {t2} {t3}, 点访问: {}", tup.0);

    /*
     * ---- 一句话总结 ----
     * Rust 变量默认不可变（let），可变要声明（let mut），
     * 重名是遮蔽不是覆盖；一切类型转换必须显式（as/parse）；
     * 类型宽度写在名字里，溢出行为可控可选。
     */
    println!("{GREETING}, MAX_POINTS={MAX_POINTS}");
}
