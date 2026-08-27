// ============================================================
// 第11章 · 迭代器与闭包：函数式风格
// ============================================================
// 运行: cargo run --bin 11_iterators_closures
// 学习要点:
//   1. 闭包：捕获环境的三种方式 Fn/FnMut/FnOnce
//   2. 迭代器是惰性的：不消费不执行
//   3. map/filter/collect 管道（对比 Python 推导式）
//   4. iter / into_iter / iter_mut 三兄弟与所有权
//   5. 常用适配器：sum/max_by/zip/enumerate/take_while
//   6. 手写 Iterator trait
// ============================================================

fn main() {
    // ==== 1. 闭包：可以捕获环境变量的匿名函数 ====
    let factor = 3;
    let times = |x: i32| x * factor;             // 捕获外部的 factor！
    println!("times(10) = {}", times(10));
    // 对比 C: 没有闭包，得手写结构体+函数指针；Python 的 lambda 不能写语句且捕获靠引用。
    // Rust 按捕获方式分三档：
    //   Fn     只读借用捕获（可多次调用）——上面这个
    //   FnMut  可变借用捕获（会改环境）
    //   FnOnce 拿走所有权（只能调一次）

    let mut log = String::new();
    {
        let mut append = |msg: &str| log.push_str(msg);    // FnMut：修改了 log
        append("a,");
        append("b");
    }
    println!("FnMut 结果: {log}");

    let owned = vec![String::from("被拿走的")];
    let consume = move || {                       // move 强制拿走所有权（常用于线程，见14章）
        println!("消费掉: {:?}", owned);
        owned
    };
    let _taken = consume();
    // consume();                                  // ← 编译错误：owned 已被移动，只能调一次
    // 对比 Python 闭包默认也是「引用」，但循环变量捕获坑（late binding）出了名的多；
    // Rust 编译期就确定每个闭包怎么捕获，借用检查全程护航。

    // ==== 2. 迭代器是惰性的 ====
    let range = (1..=3).map(|x| x * x);           // map 只是记录了计划，没有执行任何计算！
    println!("还没消费，什么都没发生");
    let squares: Vec<i32> = range.collect();      // collect 时才真正跑起来
    println!("collect 后: {squares:?}");
    // 对比 Python: map() 返回迭代器同样惰性；但列表推导式立即求值。Rust 一律惰性，
    // 链到哪都只扫一遍数据（编译器还常常内联优化到和手写 for 循环一样快——零成本抽象）。

    // ==== 3. 经典管道：filter + map + collect ====
    let nums = vec![1, 2, 3, 4, 5, 6];
    let evens_doubled: Vec<i32> = nums.iter()
        .filter(|&&n| n % 2 == 0)                 // 留偶数（|&&n| 双重解构 &&i32）
        .map(|n| n * 2)                           // 翻倍
        .collect();
    println!("偶数翻倍: {evens_doubled:?}");
    // Python 等价: [n*2 for n in nums if n%2==0]
    // C 等价: 手写循环 + 中间数组，几十行起步

    // ==== 4. iter / into_iter / iter_mut ====
    let v = vec![String::from("a"), String::from("b")];
    let lens: Vec<usize> = v.iter().map(|s| s.len()).collect();       // &T 只读，v 还在
    println!("{lens:?}, v = {v:?}");

    let mut vm = vec![1, 2, 3];
    vm.iter_mut().for_each(|x| *x += 100);                             // &mut T 原地改
    println!("iter_mut 后: {vm:?}");

    let consumed: Vec<String> = v.into_iter().map(|s| s + "!").collect(); // T 移动，v 报废
    println!("into_iter 后: {consumed:?}");
    // println!("{v:?}");                        // ← 编译错误：v 已被 into_iter 消费
    // 记法：名字里带 mut 改原数据、带 into 转移所有权、裸 iter 只读。

    // ==== 5. 更多常用适配器 ====
    let data = [5, 2, 8, 1, 9, 3];
    println!("sum = {}", data.iter().sum::<i32>());
    println!("max = {:?}", data.iter().max());
    println!("any>8? {}", data.iter().any(|&x| x > 8));
    println!("前三个: {:?}", data.iter().take(3).collect::<Vec<_>>());

    let names = ["张三", "李四", "王五"];
    let scores = [88, 92, 75];
    let pairs: Vec<String> = names.iter().zip(scores.iter())          // zip 配对
        .enumerate()                                                  // 加下标
        .map(|(i, (n, s))| format!("{i}. {n}: {s} 分"))
        .collect();
    println!("{}", pairs.join(" | "));

    let first_long: Option<&&str> = ["hi", "hello", "hey"].iter().find(|s| s.len() > 3);
    println!("第一个长度>3的词: {first_long:?}");

    // 链式惰性求值：take_while 提前短路，不会处理后面的元素
    let sum: i32 = (1..).take_while(|&x| x <= 5).sum();               // 无限序列也能用！
    println!("1..=5 之和 = {sum}");

    // ==== 6. 手写 Iterator：实现 next 方法即可 ====
    struct Fib {
        a: u64,
        b: u64,
    }
    impl Fib {
        fn new() -> Self { Fib { a: 0, b: 1 } }
    }
    impl Iterator for Fib {
        type Item = u64;                          // 关联类型：每次产出的类型
        fn next(&mut self) -> Option<u64> {
            let out = self.a;
            (self.a, self.b) = (self.b, self.a + self.b);
            Some(out)
        }
    }
    let fibs: Vec<u64> = Fib::new().take(10).collect();
    println!("斐波那契前10项: {fibs:?}");
    // 对比 Python 的 __iter__/__next__ 或生成器 yield；C 得手动管理状态机。

    /*
     * ---- 一句话总结 ----
     * 闭包捕获环境分三档（Fn/FnMut/FnOnce），线程场景常配 move；
     * 迭代器惰性 + 零成本，链式管道是 Rust 处理集合的地道写法；
     * collect 目标类型靠标注推断，类型写错编译期就能发现。
     */
}
