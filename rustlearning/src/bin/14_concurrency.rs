// ============================================================
// 第14章 · 并发编程：线程、通道与 Arc<Mutex>
// ============================================================
// 运行: cargo run --bin 14_concurrency
// 学习要点:
//   1. thread::spawn 创建线程与 join 等待
//   2. move 闭包转移数据所有权进线程
//   3. 通道 mpsc：消息传递共享内存（Go 哲学）
//   4. Mutex<T> 互斥锁：锁即数据的守护者
//   5. Arc<T>：原子引用计数跨线程共享
//   6. scoped threads：借用栈上数据的轻量并发
// ============================================================

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // ==== 1. 创建线程 ====
    // 对比 C 的 pthread_create（裸函数指针+void* 传参，类型全靠自觉）；
    // 对比 Python threading.Thread(target=...)（GIL 让多线程变「假并发」）。
    // Rust 线程是真 OS 线程无 GIL；且编译器保证线程里的数据访问合法。
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  [工作线程] 第 {i} 圈");
            thread::sleep(Duration::from_millis(10));
        }
        "线程的返回值"                            // spawn 返回 JoinHandle，能取回结果！
    });
    println!("[主线程] 干点自己的活...");
    let result = handle.join().unwrap();          // join 阻塞等待并取回返回值
    println!("[主线程] 拿到: {result}");

    // ==== 2. move：把数据所有权交给线程 ====
    let data = vec![1, 2, 3];
    let h2 = thread::spawn(move || {              // move 把 data 所有权移入闭包
        println!("  [线程2] 数据 {:?}，长度 {}", data, data.len());
        data.iter().sum::<i32>()                  // 顺便把计算结果带回去
    });
    println!("  [线程2] 总和 = {}", h2.join().unwrap());
    // 没有 move 会怎样？闭包默认借用了 data，而线程可能活得比 main 栈帧久 → 编译错误。
    // 对比 C：pthread 里传悬垂指针是经典崩溃源——Rust 直接不让你编译。

    // ==== 3. 通道 Channel：用消息传递共享内存 ====
    // Go 的名言: "Don't communicate by sharing memory; share memory by communicating."
    // Rust 标准库自带 mpsc（multi-producer, single-consumer）。
    // 对比 Python queue.Queue（运行时加锁）；Rust 通道发送即移交所有权——不可能出现竞态读。
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();                         // 克隆发送端支持多生产者
    thread::spawn(move || {
        for msg in ["消息A1", "消息A2"] {
            tx.send(msg.to_string()).unwrap();    // 发送会拿走 String 的所有权
            thread::sleep(Duration::from_millis(5));
        }
    });
    thread::spawn(move || {
        for msg in ["消息B1", "消息B2"] {
            tx2.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(15));
        }
    });

    // 接收端：rx 是迭代器，所有发送端关闭后循环自然结束
    for received in rx {
        println!("[主线程收到] {received}");
    }

    // ==== 4. 共享内存路线：Arc + Mutex ====
    // 有时确实要共享状态（计数器、缓存）。规则：
    //   Arc = 原子引用计数的 Rc（跨线程安全版）
    //   Mutex = 给数据上锁（对比 C 的 pthread_mutex_t 手动 lock/unlock 配对，
    //           忘了 unlock 死锁伺候；Python with lock: 语法糖但 GIL 在场）
    // Rust 的妙处：Mutex「包裹」数据本身，想碰数据必须先过锁——物理上无法绕过！
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);       // 每个线程克隆一个句柄（只增计数）
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // lock() 返回智能指针，离开作用域自动解锁（RAII！）不可能忘记 unlock
                let mut num = counter.lock().unwrap();
                *num += 1;
            }                                     // ← 这里自动释放锁
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("4 个线程 × 1000 次，最终计数 = {}", *counter.lock().unwrap());
    // C 版本忘了加锁？数据竞争静默出错。Rust 版本不加锁根本编译不过。

    // ==== 5. scoped threads：不用 Arc 也能借用本地数据（Rust 1.63+）====
    // scope 保证所有子线程在块结束前 join，因此可以安全地借用栈上数据
    let mut results = vec![0i32; 3];
    let input = [10, 20, 30];
    thread::scope(|s| {
        // iter_mut 每次产出互不重叠的 &mut——「可变借用不冲突」编译器能看懂的形式
        for (slot, chunk) in results.iter_mut().zip(input.chunks(1)) {
            s.spawn(move || {
                *slot = chunk.iter().sum::<i32>() * 10;
            });
        }
    });                                           // ← scope 结束自动等待全部线程
    println!("scoped 结果: {results:?}");
    // 对比传统方案省掉了 Arc 的引用计数开销；C 并发里「等所有线程结束」要手写 barrier。

    /*
     * ---- 一句话总结 ----
     * Send/Sync 两个标记 trait + 所有权检查 = 无数据竞争的并发（编译期保证）；
     * 首选通道传递所有权，确需共享时 Arc<Mutex<T>>，短任务用 scope 借用；
     * 锁的释放由 RAII 自动完成，C 时代的手动 unlock/死锁排查少了一大类。
     */
}
