// ============================================================
// 第7章 · 模式匹配：match、if let 与解构
// ============================================================
// 运行: cargo run --bin 07_pattern_matching
// 学习要点:
//   1. match 表达式：穷尽性检查 + 不穿透
//   2. 匹配字面量、范围、多模式 |、守卫 if
//   3. 绑定 @ 模式
//   4. 解构：元组 / 结构体 / 枚举嵌套
//   5. if let / while let / let else：只关心一种模式的简写
// ============================================================

#[derive(Debug)]
enum Message {
    Quit,                          // 无数据
    Move { x: i32, y: i32 },       // 具名字段
    Write(String),                 // 携带 String
    ChangeColor(u8, u8, u8),       // 携带三个值
}

fn process(msg: &Message) -> String {
    match msg {                    // 每个 arm：模式 => 表达式；穷尽性由编译器保证
        Message::Quit => "退出".to_string(),
        Message::Move { x, y } => format!("移动到 ({x}, {y})"),   // 直接解构字段
        Message::Write(text) => format!("写入 '{text}'"),         // text 绑定内部数据
        Message::ChangeColor(r, g, b) => format!("颜色 #{r:02x}{g:02x}{b:02x}"),
    }
}

fn main() {
    // ==== 1. match 基础 ====
    // 对比 C switch：不用 break、不会穿透 fall-through、漏分支直接编译报错；
    // 对比 Python 3.10 的 match/case：语法神似，但 Rust 是编译期穷尽检查且能绑定数据。
    let n = 3;
    let desc = match n {
        0 => "零",
        1 | 2 => "一或二",                    // | 多模式或
        3..=9 => "三到九",                     // 范围匹配（..= 含端点）
        _ => "其他",                           // _ 通配，兜底必须有（否则不穷尽）
    };
    println!("{n} → {desc}");

    // ==== 2. 守卫 guard：匹配后加条件 ====
    let num = 60;
    let grade = match num {
        g if g >= 90 => "优秀",
        g if g >= 60 => "及格",
        _ => "不及格",
    };
    println!("{num} 分 → {grade}");

    // ==== 3. 绑定 @：匹配范围的同时拿到值 ====
    let age = 25;
    match age {
        a @ 0..=17 => println!("{a} 岁：未成年"),
        a @ 18..=59 => println!("{a} 岁：成年劳动力"),     // a 同时获得 age 的值
        a => println!("{a} 岁：退休年龄"),
    }

    // ==== 4. 解构元组与结构体 ====
    let point = (3, -2);
    match point {
        (0, y) => println!("在 y 轴上, y={y}"),
        (x, 0) => println!("在 x 轴上, x={x}"),
        (x, y) => println!("普通点 ({x}, {y})"),
    }
    // 像 Python 的模式解构 tuple，但多了「按形状分发」的能力

    #[allow(dead_code)]                       // mp 字段故意留着演示 .. 忽略语法
    struct Player { name: String, hp: i32, mp: i32 }
    let p = Player { name: "勇者".into(), hp: 100, mp: 30 };
    match p {
        Player { hp, .. } if hp <= 0 => println!("阵亡"),
        Player { name, hp, .. } => println!("{name} 存活，血量 {hp}"),  // .. 忽略其余字段
    }

    // ==== 5. 解构嵌套枚举（实战最常见的形态）====
    let msgs = [
        Message::Quit,
        Message::Move { x: 10, y: -5 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];
    for m in &msgs {
        println!("{}", process(m));
    }

    // ==== 6. if let：只关心一种模式时的简写 ====
    let config: Option<String> = Some("config.toml".to_string());
    // 用 match 写会啰嗦：
    // match config { Some(c) => ..., None => {} }
    if let Some(path) = config {                       // 解构并绑定 path
        println!("if let 拿到配置: {path}");
    } else {
        println!("没有配置");
    }
    // 类比：Python 的 `if v is not None:`，但这里连拆包带判断一步到位，且编译器保证安全

    // 带 else 的完整形态：
    let score: Result<i32, String> = Err("缺考".to_string());
    if let Ok(s) = &score {
        println!("得分 {s}");
    } else {
        println!("成绩异常: {:?}", score);            // else 分支拿原始值
    }

    // ==== 7. let else（Rust 1.65+）：守卫函数前置条件 ====
    // 「 happy path 在前、异常提前 return」的风格，替代一层层嵌套 if
    fn parse_port(arg: Option<&str>) -> u16 {
        let Some(text) = arg else {
            return 8080;               // else 分支必须「发散」：return/panic/continue
        };
        text.parse().unwrap_or(8080)   // 走到这里 text 已可用——主逻辑不再缩进
    }
    println!("parse_port(None) = {}, parse_port(Some(\"3000\")) = {}",
             parse_port(None), parse_port(Some("3000")));

    // ==== 8. while let：循环消费直到模式不成立 ====
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {                // pop 返回 Option，空了就 None 退出
        print!("弹出 {top} ");
    }
    println!();
    // 对比 Python 的 while stack: print(stack.pop())——Rust 版本不可能弹出 None 还往下走

    /*
     * ---- 一句话总结 ----
     * match 是 Rust 的流程控制核心：穷尽、不穿透、能解构嵌套数据；
     * 单一模式用 if let/let else 减少噪音，集合消费用 while let；
     * 掌握了模式匹配，Option/Result 的 API 才算真正顺手。
     */
}
