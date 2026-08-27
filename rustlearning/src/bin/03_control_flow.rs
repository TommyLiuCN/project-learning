// ============================================================
// 第3章 · 控制流：if 表达式、三种循环、loop 返回值
// ============================================================
// 运行: cargo run --bin 03_control_flow
// 学习要点:
//   1. if 是表达式，可以出现在 let 右边
//   2. 条件必须是 bool（不会隐式把整数当条件）
//   3. loop 循环 + break 带返回值（其他语言少见）
//   4. while 与 for 区间遍历
//   5. 循环标签 'label 跳出多层循环
//   6. match 提前预览（第7章详解）
// ============================================================

fn main() {
    // ==== 1. if 表达式 ====
    let n = 7;
    if n > 5 {
        println!("{n} 大于 5");
    } else if n == 5 {
        println!("{n} 等于 5");
    } else {
        println!("{n} 小于 5");
    }
    // 对比 C: 条件可以写 if (n)（非零即真）；Rust 必须 if n != 0 —— bool 就是 bool，
    // 不允许隐式转换，杜绝 if (x = 5) 这种手滑赋值事故。

    // if 作为表达式（像 Python 的三元 x if cond else y，或 C 的 ?:）
    let parity = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{n} 是{parity}");
    // 注意两个分支类型必须一致——类型系统保证 parity 只有一种类型

    // ==== 2. loop：无条件循环 ====
    // 对比 C 的 for(;;) 或 while(1)；loop 让编译器明确知道这是有意死循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;       // break 可以带返回值！整个 loop 表达式的值
        }
    };
    println!("loop 结果 = {result}");

    // ==== 3. while ====
    let mut num = 3;
    while num != 0 {
        print!("{num}...");
        num -= 1;
    }
    println!("发射！");
    // 对比 C 几乎一样，只是条件不加括号、体必须用大括号

    // ==== 4. for：区间遍历（最常用） ====
    // 对比 Python 的 for i in range(...)；对比 C 的 for(i=0;i<5;i++) 手写下标
    for i in 0..5 {                  // 左闭右开 [0,5)
        print!("{i} ");
    }
    println!();
    for i in 1..=5 {                 // ..= 双闭区间 [1,5]
        print!("{i} ");
    }
    println!();
    // 反向遍历：迭代器适配器，比 C 的 for(i=n-1;i>=0;i--) 安全得多（不会下溢）
    for i in (0..5).rev() {
        print!("{i} ");
    }
    println!();

    // 遍历数组：for 直接拿引用迭代，不需要手动算长度（对比 C 的 sizeof 技巧）
    let arr = [10, 20, 30, 40];
    for v in arr {                   // 数组实现了 IntoIterator（Rust 2021 起）
        print!("{v} ");
    }
    println!();
    // 带下标遍历：enumerate() 像 Python 的 enumerate()
    for (idx, val) in arr.iter().enumerate() {
        print!("[{idx}]={val} ");
    }
    println!();

    // ==== 5. 循环标签：跳出多层循环 ====
    // 对比 C 用 goto 或 flag 变量；Rust 用标签干净利落（Python 只能靠封装函数）
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i * j == 6 {
                println!("找到 i*j==6（i={i}, j={j}），直接跳出两层");
                break 'outer;
            }
        }
    }

    // ==== 6. match 预览 ====
    let day = 6;
    let name = match day {
        1..=5 => "工作日",
        6 | 7 => "周末",             // | 匹配多个值
        _ => "非法",                 // _ 类似 switch 的 default，但 match 必须穷尽所有情况！
    };
    println!("day={day} → {name}");
    // 对比 C 的 switch：不用写 break（没有穿透）、能匹配范围、漏了情况编译报错。

    /*
     * ---- 一句话总结 ----
     * if/match/loop 都是表达式有值；条件必须严格 bool；
     * for 配区间和迭代器是首选，while 次之，loop 用于需要 break 带值退出的场景。
     */
}
