// ============================================================
// 第12章 · 智能指针：Box、Rc 与 RefCell
// ============================================================
// 运行: cargo run --bin 12_smart_pointers
// 学习要点:
//   1. 智能指针 = 结构体 + Deref/Drop（对比裸指针）
//   2. Box<T>：堆分配与递归类型
//   3. Deref 解引用与自动 coercion
//   4. Drop trait：确定性析构（RAII）
//   5. Rc<T>：引用计数共享所有权
//   6. RefCell<T>：运行时借用检查（内部可变性）
// ============================================================

use std::cell::RefCell;
use std::rc::Rc;

// ==== 1. Box<T>：最简单的智能指针 ====
// 把数据放到堆上，栈上只留一个指针。编译器保证：指针活着数据就活着，指针死数据立刻释放。
// 对比 C 的 malloc/free：手动配对，漏 free 泄漏、double free 崩溃；
// 对比 C++ unique_ptr：几乎一样！但 Rust 的独占所有权让它不可能被意外复制。
#[derive(Debug)]
enum List {                                    // 递归类型：编译器要算大小，必须用 Box 打断
    Cons(i32, Box<List>),                      // （字段仅通过 Debug 打印，故 allow dead_code）
    Nil,
}

use List::{Cons, Nil};

fn walk(list: &List) -> Vec<i32> {             // 遍历链表，让字段真正被「读」到
    let mut out = Vec::new();
    let mut cur = list;
    while let Cons(val, next) = cur {
        out.push(*val);
        cur = next;
    }
    out
}

// 自定义 Drop：离开作用域时自动调用（对比 C++ 析构函数；Python 的 __del__ 时序不可靠）
struct Resource {
    name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  [drop] 释放资源: {}", self.name);     // 手动 free 的活儿全自动了
    }
}

fn main() {
    // ==== 2. Box 基础用法 ====
    let b = Box::new(5);
    println!("b = {b}，*b = {}", *b);            // * 解引用取堆上的值（像 C 的 *p）
    // 大数组放堆上避免栈溢出：
    let big = Box::new([0u8; 100_000]);
    println!("大数组首元素 = {}", big[0]);         // Box 用起来完全透明

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表: {list:?}, 遍历 = {:?}", walk(&list));
    // 对比 C: struct Node* next + 手动遍历释放；这里 drop 自动递归释放整条链。

    // ==== 3. Deref：智能指针的「透明性」====
    // String 和 Vec 也是智能指针！实现了 Deref 所以能当 &str / &[T] 用。
    let s = String::from("hello");
    fn takes_str(x: &str) -> usize { x.len() }
    println!("String 自动转 &str 长度: {}", takes_str(&s));   // &String --Deref--> &str
    // 这就是为什么函数参数写 &str 比 &String 更通用（同理 &[T] 优于 &Vec<T>）。

    // ==== 4. Drop 与 RAII ====
    {
        let _r1 = Resource { name: "文件句柄".into() };
        let _r2 = Resource { name: "数据库连接".into() };
        println!("  正在使用资源...");
    }                                            // ← 离开作用域，按声明逆序 drop
    println!("作用域结束，资源已全部释放（无需 try/finally）");
    // 对比 Python with open(...) as f: 或 Java try-with-resources——Rust 所有类型天生如此。

    // 手动提前释放：std::mem::drop（注意不是 C 的 free，只是触发 Drop）
    let r3 = Resource { name: "临时锁".into() };
    drop(r3);
    println!("  r3 已提前释放，后面代码不再持有它");
    // drop(r3);                                  // ← 编译错误：value used after being moved

    // ==== 5. Rc<T>：引用计数，多处共享同一份数据 ====
    // 单一所有权的规则太严格？比如图/树中多个节点指向同一个子节点。
    // Rc 让「只读共享」合法：clone 不复制数据，只增加计数。
    // 对比 C++ shared_ptr（同为引用计数）；对比 Python 万物皆引用计数+GC 兜底。
    let shared = Rc::new(String::from("共享数据"));
    println!("初始计数: {}", Rc::strong_count(&shared));
    let child_a = Rc::clone(&shared);            // 计数 → 2
    let child_b = Rc::clone(&shared);            // 计数 → 3
    println!("两个 clone 后计数: {}（没有复制字符串本体）", Rc::strong_count(&shared));
    println!("a={child_a}, b={child_b}, 原始={shared}");

    {
        let _child_c = Rc::clone(&shared);
        println!("作用域内计数: {}", Rc::strong_count(&shared));
    }                                            // c 出作用域计数回落
    println!("离开内层作用域后计数: {}", Rc::strong_count(&shared));

    // ==== 6. RefCell<T>：把借用检查推迟到运行时 ====
    // 编译期借用规则有时无法表达（如「逻辑上互斥但编译器看不见」），RefCell 提供内部可变性：
    // 表面不可变，实际可在运行时借用修改。违反规则不会 UB，而是当场 panic——依然安全。
    let cell = RefCell::new(42);
    {
        let mut m = cell.borrow_mut();           // 可变借用
        *m += 8;
        // let r = cell.borrow();                // ← 取消注释：已有可变借用再读 → panic!
    }
    println!("RefCell 内部值: {}", cell.borrow());

    // ==== 7. 组合拳 Rc<RefCell<T>>：多所有权 + 可变 ====
    // 经典场景：多个所有者共享一份数据且偶尔要改（如 GUI 控件树、依赖图）
    let counter = Rc::new(RefCell::new(0));
    let handles: Vec<_> = (0..3)
        .map(|_| {
            let c = Rc::clone(&counter);
            move || *c.borrow_mut() += 10        // 每个克隆都改同一份数据
        })
        .collect();
    for h in handles {
        h();
    }
    println!("Rc<RefCell> 计数结果: {:?}", counter.borrow());   // 30
    // 注意 Rc 不是线程安全的！跨线程要用 Arc<Mutex<T>>（第14章）。

    /*
     * ---- 一句话总结 ----
     * Box 解决「放堆上」和递归类型；Drop 给所有类型 RAII；
     * Rc 共享只读所有权（单线程）；需要边共享边改就套 RefCell；
     * 智能指针是普通结构体 + Deref/Drop 两个 trait——没有魔法，全是机制。
     */
}
