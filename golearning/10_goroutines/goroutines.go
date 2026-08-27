// ============================================================
// 第10章 · Goroutine 与同步
// ============================================================
// 运行: go run ./10_goroutines
// 竞态检测: go run -race ./10_goroutines
// 学习要点:
//  1. go 关键字启动并发：goroutine 比线程便宜得多
//  2. goroutine vs OS 线程 vs Python 协程
//  3. sync.WaitGroup：等待一批并发任务完成
//  4. 数据竞争与 sync.Mutex 互斥锁
//  5. -race 竞态检测器的使用
//
// ============================================================
package main

import (
	"fmt"
	"sync"
	"time"
)

func say(msg string, wg *sync.WaitGroup) {
	defer wg.Done() // 任务结束时计数-1；用 defer 保证 panic 时也减
	fmt.Println("goroutine 说:", msg)
	time.Sleep(50 * time.Millisecond) // 模拟耗时操作
}

func main() {
	// ==== 1. 启动 goroutine ====
	// 在函数调用前加 go，就把它扔到后台跑，go 语句本身立即返回不阻塞。
	// 三方对比：
	//   C:      pthread_create 手动管句柄/join，线程 MB 级栈
	//   Python: threading.Thread 受 GIL 限制无法并行跑 CPU 任务；
	//           asyncio 协程虽轻但有 async 函数"染色"传染问题
	//   Go:     goroutine 由运行时调度到少量 OS 线程上(M:N 调度)，
	//           初始栈仅 ~2KB 且可伸缩，轻松开几十万个，且无 GIL、无染色
	var wg sync.WaitGroup // WaitGroup：计数器式的"集合点"，类似 join 所有线程
	wg.Add(2)             // 预告要等 2 个任务
	go say("你好", &wg)     // 注意传 &wg：WaitGroup 不允许拷贝！
	go say("世界", &wg)
	wg.Wait() // 阻塞直到计数归零
	fmt.Println("两个都完成了")

	// 重要：main 一退出所有 goroutine 直接死亡，不会自动等待！
	// 所以必须有 WaitGroup/channel 这样的同步手段

	// ==== 2. 匿名 goroutine 批量任务 ====
	results := make([]int, 5) // 各 goroutine 写不同下标 → 无数据竞争
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(idx int) { // 把循环变量作为参数传入（Go1.22 前的经典坑：共享同一个 i）
			defer wg.Done()
			time.Sleep(time.Duration(5-idx) * 10 * time.Millisecond) // 故意乱序完成
			results[idx] = idx * idx
		}(i)
	}
	wg.Wait()
	fmt.Println(results) // [0 1 4 9 16]——每个槽位由对应 goroutine 填好

	// ==== 3. 数据竞争与 Mutex ====
	// 多个 goroutine 同时读写同一变量 = 数据竞争，结果错误且不可复现
	counter := 0
	var mu sync.Mutex // 互斥锁：同一时刻只允许一个 goroutine 进入临界区
	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			mu.Lock() // 加锁
			counter++ // counter++ 是 读→加→写 三步，不加锁必丢更新
			mu.Unlock()
		}()
	}
	wg.Wait()
	fmt.Println("加锁后的计数:", counter) // 稳定输出 1000

	// 实验建议：删掉 Lock/Unlock 两行后执行 go run -race ./10_goroutines，
	// 会看到 DATA RACE 报告——竞态检测器是排查并发的第一利器

	// 补充武器库：
	//   纯计数/标志位        → sync/atomic 原子操作更轻量
	//   只初始化一次的逻辑    → sync.Once
	//   保护读多写少的 map    → sync.RWMutex 或官方 sync.Map
	//   goroutine 间传数据   → channel（下一章），优先于锁

	/*
	 * ---- 一句话总结 ----
	 * go 一个词开并发；main 不等人，WaitGroup 来收尾；
	 * 共享可变状态是万恶之源——要么加锁，要么干脆别共享（用 channel）。
	 */
}
