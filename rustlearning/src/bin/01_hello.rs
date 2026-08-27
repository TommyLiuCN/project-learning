// ============================================================
// 第1章 · Hello Rust：第一个程序与 println! 宏
// ============================================================
// 运行: cargo run --bin 01_hello
// 学习要点:
//   1. Rust 程序入口 fn main()
//   2. println! 是宏不是函数（! 的含义）
//   3. 格式化输出：{} 占位、{:?} 调试、{变量名} 内联
//   4. 编译期检查：类型错误直接编译失败
//   5. cargo 基本工作流
// ============================================================

fn main() {
    // ==== 1. 最经典的开始 ====
    // 对比 C:  printf("Hello, world!\n");  需要手动加 \n
    // 对比 Python: print("Hello, world!")  自带换行
    // Rust: println! 自带换行，! 表示这是「宏」而非函数——宏在编译期展开，
    //       所以能接受任意数量的参数（printf 那种可变参数 C 要用 ... 黑魔法）
    println!("Hello, world!");

    // ==== 2. 宏 vs 函数 ====
    // 带 ! 的是宏：println!/vec!/assert_eq!/write! ...
    // 函数调用写 ! 会编译报错，编译器会贴心提示「你是不是想用宏？」
    let name = "Rust";
    let version = 1.97;
    println!("Hello, {name}!");          // 内联写法（Rust 2021+），类似 Python f-string
    println!("版本 {}", version);         // 占位符写法，类似 C 的 printf("%s", ...)

    // ==== 3. 格式化输出全家桶 ====
    let x = 42;
    let pi = 3.14159;
    println!("十进制 {x}, 十六进制 {:x}, 八进制 {:o}, 二进制 {:b}", x, x, x);
    println!("保留两位小数: {:.2}", pi);           // 类似 printf("%.2f")
    println!("宽度对齐: [{:>8}] [{:<8}] [{:^8}]", "右", "左", "中");
    println!("补零: {:08.2}", pi);
    println!("{:#?}", (1, 2, 3));                  // Debug 美化输出，调试利器

    // ==== 4. eprintln! 输出到 stderr ====
    // 对比 C 的 fprintf(stderr, ...) / Python 的 print(..., file=sys.stderr)
    eprintln!("这条走标准错误流");

    // ==== 5. 表达式体验：一切皆表达式 ====
    // 块 {} 的值是最后一个表达式的值（注意没有分号！），后面第4章细讲
    let sum = {
        let a = 10;
        let b = 32;
        a + b                       // 没有分号 → 这是表达式，作为块的值
    };
    println!("块表达式的值 = {sum}");

    /*
     * ---- cargo 工作流速记 ----
     * cargo new rustlearning        新建项目
     * cargo build                   编译到 target/debug/
     * cargo run                     编译并运行
     * cargo run --bin 01_hello      运行指定的二进制（本项目每章一个 bin）
     * cargo check                   只查类型不生成代码，最快
     * cargo build --release         开优化发布构建（跑基准用它）
     *
     * ---- 一句话总结 ----
     * Rust 没有 printf 那样的运行时格式解析，println! 在编译期就检查好类型；
     * 写错了占位符数量/类型会直接编译失败——安全从 Hello World 开始。
     */
}
