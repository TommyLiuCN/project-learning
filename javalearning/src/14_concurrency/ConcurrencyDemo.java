// ============================================================
// 第14章 · 多线程与并发（入门）
// ============================================================
// 运行: java src/14_concurrency/ConcurrencyDemo.java
// 学习要点:
//   1. 创建线程：Thread / Runnable / Lambda
//   2. sleep 与 join 等待
//   3. 竞态条件：为什么 count++ 不安全
//   4. synchronized 加锁
//   5. 原子类 AtomicInteger
//   6. 线程池 ExecutorService + Future
// ============================================================

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.atomic.AtomicInteger;

public class ConcurrencyDemo {

    private static int unsafeCount = 0;                 // 非线程安全计数器
    private static final AtomicInteger safeCount = new AtomicInteger(); // 原子计数器

    public static void main(String[] args) throws InterruptedException {
        // ==== 1. 创建线程的三种姿势 ====
        System.out.println("=== 创建线程 ===");
        // 方式一：继承 Thread（Java 单继承，占用了继承名额，不推荐）
        Thread t1 = new Thread() {
            @Override
            public void run() {
                System.out.println("[Thread 子类] 我是新线程: " + Thread.currentThread().getName());
            }
        };

        // 方式二：实现 Runnable + Lambda（推荐！任务和线程分离，类似 Python 的 target=func）
        Thread t2 = new Thread(() ->
                System.out.println("[Runnable] 主线程是: " + Thread.currentThread().getName()));

        t1.start();     // start() 才会开新线程！直接调 run() 只是普通方法调用
        t2.start();

        // ==== 2. sleep 与 join ====
        Thread.sleep(100);                              // 当前线程暂停 100ms，类比 Python time.sleep
        t1.join();                                      // 等 t1 结束才继续往下走，类比 thread.join()
        t2.join();
        System.out.println("两个线程都结束了\n");

        // ==== 3. 竞态条件演示 ====
        System.out.println("=== 竞态条件 ===");
        // count++ 看似一步，实际是"读→加→写回"三步。两个线程同时读旧值就会丢更新。
        // 对比 C 同样的问题；Python 因 GIL 情况不同但逻辑上同样要加锁。
        Thread a = new Thread(() -> addUnsafe(50_000));
        Thread b = new Thread(() -> addUnsafe(50_000));
        a.start(); b.start();
        a.join();  b.join();
        System.out.println("期望 100000, 实际 unsafe = " + unsafeCount + " (每次结果都可能不一样!)");

        // ==== 4. AtomicInteger 原子修复 ====
        Thread c = new Thread(() -> addSafe(50_000));
        Thread d = new Thread(() -> addSafe(50_000));
        c.start(); d.start();
        c.join();  d.join();
        System.out.println("原子类保证 safe = " + safeCount.get());
        // incrementAndGet 底层用 CPU 原子指令，比加锁轻量

        // ==== 5. synchronized 锁 ====
        System.out.println("\n=== synchronized ===");
        Object lock = new Object();                     // 任何对象都能当锁
        Runnable critical = () -> {
            synchronized (lock) {                       // 同一时刻只允许一个线程进入代码块
                System.out.println(Thread.currentThread().getName() + " 拿到锁，进入临界区");
                try { Thread.sleep(10); } catch (InterruptedException e) { }
            }                                           // 出了代码块自动释放锁（不会忘记解锁）
        };
        new Thread(critical, "线程甲").start();
        new Thread(critical, "线程乙").start();
        Thread.sleep(50);
        // 修饰方法时 synchronized(this)：整个方法体互斥——粒度太粗，性能差，推荐小块同步

        // ==== 6. 线程池 ====
        System.out.println("\n=== 线程池 ===");
        // 频繁创建销毁线程开销大；线程池复用线程。
        // 类比 Python 的 concurrent.futures.ThreadPoolExecutor——思想一模一样。
        try (ExecutorService pool = Executors.newFixedThreadPool(3)) { // 固定3个工人
            // submit 返回 Future：一张"取货单"，结果还没算完也能先拿到凭证
            Future<Integer> future = pool.submit(() -> {
                Thread.sleep(200);                      // 模拟耗时计算
                return 40 + 2;
            });
            System.out.println("提交任务后先干点别的...");
            Integer result;
            try {
                result = future.get();              // get() 会阻塞直到算完
            } catch (java.util.concurrent.ExecutionException e) {
                // 任务内部抛的异常会被包装成 ExecutionException 在这里收到
                throw new RuntimeException(e.getCause());
            }
            System.out.println("异步计算结果: " + result);

            pool.submit(() -> System.out.println("[池中线程] 打印任务"));
        }                                               // try-with-resources 自动优雅关闭(Java 19+)
                                                        // 老版本需手动 shutdown()

        /*
         * ---- 进阶路线 ----
         * volatile 关键字 → 可见性问题
         * ReentrantLock / ReadWriteLock → 更灵活的锁
         * ConcurrentHashMap → 线程安全的 Map
         * CompletableFuture → 异步编排
         * Java 21 虚拟线程 → 轻量百万级并发（Go 协程的味道）
         *
         * ---- 一句话总结 ----
         * start 开线程、join 等结束；
         * 共享可变数据要么加锁(synchronized)要么换原子类；
         * 实际开发几乎总是用线程池而不是裸 new Thread。
         */
    }

    static void addUnsafe(int times) {
        for (int i = 0; i < times; i++) {
            unsafeCount++;                  // 三步操作被打断就丢更新
        }
    }

    static void addSafe(int times) {
        for (int i = 0; i < times; i++) {
            safeCount.incrementAndGet();    // 读-改-写是原子的
        }
    }
}
