// ============================================================
// 第5章 · 所有权与借用：Rust 最独特的概念（核心章节）
// ============================================================
// 运行: cargo run --bin 05_ownership
// 学习要点:
//   1. 所有权三规则：唯一拥有者 / 移动转移 / 离开作用域自动释放
//   2. move 语义 vs Copy 语义
//   3. clone：显式深拷贝
//   4. 借用 &T / &mut T 与借用规则
//   5. 悬垂引用在编译期被拒绝
//   6. 切片 &str 与 &[i32]
// ============================================================

// 接收 String 并拿走所有权（s 在函数结束后被释放）
fn take_ownership(s: String) {
    println!("take_ownership 拿到了: {s}");
}   // s 在这里离开作用域，内存自动释放 —— 不需要 free！

// 借用：只读不改，不获取所有权（& 相当于「借条」）
fn calc_len(s: &String) -> usize {
    s.len()
}

// 可变借用：可以修改原数据
fn push_world(s: &mut String) {
    s.push_str(", world");
}

// ❌ 经典悬垂引用示例（已注释掉，取消注释会编译失败）：
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s          // 编译错误：s 在函数结束被释放，不能返回它的引用
// }               // ← C 里这是未定义行为，编译器不管你；Rust 直接拒绝编译！

fn main() {
    // ==== 1. 为什么需要所有权？====
    // C/C++：手动 malloc/free、new/delete，忘了释放=泄漏，重复释放=崩溃，悬垂指针=未定义行为
    // Python/Java：GC 全托管，省心但运行时有开销、有停顿
    // Rust：编译期静态分析（谁拥有、谁能借），零运行时开销地做到「用完自动释放」
    {
        let _s = String::from("堆上的字符串");      // _s 拥有这块堆内存
    }   // ← 这里 _s 出作用域，drop 自动调用，堆内存立即释放（类似 C++ RAII，但是强制的）

    // ==== 2. 移动 move：赋值即转移所有权 ====
    let s1 = String::from("hello");
    let s2 = s1;                 // 所有权从 s1 移到 s2！
    // println!("{s1}");         // ← 编译错误：borrow of moved value `s1`
    println!("s2 = {s2}，s1 已失效");
    // 对比 C: 浅拷贝指针后两份 free → double free 崩溃；
    // 对比 Python: 变量只是引用标签，b = a 后两个名字都有效（引用计数兜底）。
    // Rust 的选择：栈上数据复制便宜的直接 Copy，堆上数据的赋值一律 move。

    // ==== 3. Copy 类型的例外 ====
    let x = 5;
    let y = x;                   // 整数是 Copy 类型，按位复制，x 依然可用
    println!("Copy: x={x}, y={y} 都能用");
    // Copy 家族：整数、浮点、bool、char、不可变引用、全由 Copy 组成的元组/数组。
    // 判断口诀：不需要「释放动作」的类型（无堆内存）基本都是 Copy。

    // ==== 4. clone：想要副本就明说 ====
    let s3 = String::from("原始");
    let s4 = s3.clone();         // 显式深拷贝堆数据，两个变量各自拥有
    println!("clone: s3={s3}, s4={s4}");
    // 对比 Python 的 copy.deepcopy；C++ 的拷贝构造——但 Rust 把深浅拷贝写进了语法层面，
    // 浅拷贝(move)和深拷贝(clone)一眼可辨，代码审查时一目了然。

    // ==== 5. 函数传参与所有权 ====
    let s = String::from("归属权测试");
    calc_len(&s);                // 借出去，不交出所有权
    println!("借出后还能用: {s}");

    take_ownership(s);           // 所有权移入函数
    // println!("{s}");          // ← 编译错误：s 已经没了

    // 还想继续用？要么传 &s（借用），要么 clone，要么把所有权还回来：
    let mut s = String::from("还回来");
    s = consume_and_give_back(s);
    println!("所有权绕了一圈回来了: {s}");

    // ==== 6. 借用规则：同一时刻「多个只读」或「一个可写」====
    let mut data = String::from("abc");
    let r1 = &data;
    let r2 = &data;              // ✓ 两个不可变借用共存
    println!("r1={r1}, r2={r2}");
    // NLL（Non-Lexical Lifetimes）：r1/r2 最后使用之后，借用就结束了
    let r3 = &mut data;          // ✓ 所以这里再可变借用没问题
    r3.push_str("def");
    // let r4 = &data;            // ✗ 取消注释报错：r3 还活着时不能再不可变借用
    println!("r3={r3}");
    // 这条规则从根上消灭了数据竞争：C 的并发 bug 十有八九是「边读边写」，
    // Python 用 GIL 锁全局硬扛性能；Rust 在编译期就禁止。

    // ==== 7. 可变借用的实际用法 ====
    push_world(&mut data);       // 借出可变引用，用完自动归还
    println!("push 后: {data}");

    // ==== 8. 切片：对集合的「窗口视图」====
    // &str 就是指向字符串某段的切片；对比 C 的 char* + len 手动配对，Rust 切片自带长度
    let full = String::from("hello world");
    let hello = &full[0..5];     // [0..5) 左闭右开，同 Python 的 full[0:5]
    let world = &full[6..];
    println!("切片: '{hello}' + '{world}'");

    let nums = [1, 2, 3, 4, 5];
    let mid = &nums[1..4];       // &[i32] 数组切片
    println!("数组切片: {mid:?}");

    /*
     * ---- 一句话总结 ----
     * 每个值有唯一所有者，赋值/传参默认移动所有权；
     * 想用不想拿 → 借用 &；想改 → &mut；想留副本 → clone。
     * 借用规则（多读或一写）+ 编译期检查 = 没有 GC 也没有悬垂指针。
     * 这是 Rust 一切安全性的地基，后面每一章都建立在它之上。
     */
}

// 收下所有权再归还的常见模式
fn consume_and_give_back(mut s: String) -> String {
    s.push_str("！");
    s                            // 所有权移回调用方
}
