// ============================================================
// 第11章 · Channel 与 select
// ============================================================
// 运行: go run ./11_channels
// 学习要点:
//  1. channel：goroutine 间的类型安全管道——"用通信共享内存"
//  2. 无缓冲(同步交接) vs 有缓冲(异步队列)
//  3. close 与 range / comma-ok：优雅通知"没有更多数据"
//  4. select：同时等待多个通道 + time.After 超时控制
//  5. 单向通道：编译期约束收发权限
//  6. 经典模式：worker pool
//
// ============================================================
package main

import (
	"fmt"
	"time"
)

// 单向通道参数：生产者只能发 chan<-，消费者只能收 <-chan。
// 双向通道可以隐式转成单向，编译器强制约束方向防止误用。
func producer(out chan<- int, n int) {
	for i := 1; i <= n; i++ {
		out <- i // 发送；无缓冲通道会阻塞到有人接收为止
	}
	close(out) // 由【发送方】负责关闭，告诉接收方不会再有新数据
}

func main() {
	// ==== 1. 无缓冲通道 ====
	ch := make(chan int) // 无缓冲：发送与接收必须"碰面"，是一个同步交接点
	// 类比队列：Python queue.Queue / 手写环形缓冲区；
	// channel 的阻塞语义内置了同步，通常无需再配锁
	go func() {
		ch <- 42 // 阻塞中……直到 main 来接收
	}()
	v := <-ch // <-ch 接收；箭头指向就是数据流向
	fmt.Println("收到:", v)

	// ==== 2. 有缓冲通道 ====
	buffered := make(chan string, 2) // 缓冲容量 2：没满就能发不阻塞，不空就能收
	buffered <- "a"
	buffered <- "b" // 已满，再来一次就会阻塞
	fmt.Println(len(buffered), cap(buffered))
	fmt.Println(<-buffered, <-buffered)

	// ==== 3. close + range ====
	nums := make(chan int)
	go producer(nums, 3)
	for n := range nums { // range 会在通道 close 且取空后自动结束循环
		fmt.Print(n, " ")
	}
	fmt.Println()

	x, ok := <-nums // comma-ok：ok=false 表示已关闭且取空（值是对应类型的零值）
	fmt.Println(x, ok)

	// 规矩：向已关闭的通道发送 → panic；重复 close → panic。谁生产谁关闭。

	// ==== 4. select 多路复用 ====
	c1 := make(chan string)
	c2 := make(chan string)
	go func() { time.Sleep(30 * time.Millisecond); c1 <- "来自 c1" }()
	go func() { time.Sleep(60 * time.Millisecond); c2 <- "来自 c2" }()

	for i := 0; i < 2; i++ {
		select { // 哪个通道先就绪走哪个分支；同时就绪则随机选（防饿死）
		case msg := <-c1:
			fmt.Println(msg)
		case msg := <-c2:
			fmt.Println(msg)
		}
	}

	// select + time.After = 超时控制（类比 Python asyncio.wait_for）
	timeout := make(chan int)
	select {
	case r := <-timeout:
		fmt.Println(r)
	case <-time.After(50 * time.Millisecond):
		fmt.Println("超时了！放弃等待")
	}
	// select 里配 default 分支则变成非阻塞尝试

	// ==== 5. worker pool 模式 ====
	jobs := make(chan int, 10) // 任务队列
	done := make(chan int, 10) // 结果队列
	for w := 1; w <= 3; w++ {  // 3 个 worker 抢同一个 jobs 队列
		go func(id int) {
			for j := range jobs { // jobs 关闭且取空后自动退出
				time.Sleep(20 * time.Millisecond) // 模拟干活
				done <- id*100 + j                // 把结果发回去
			}
		}(w)
	}
	const jobCount = 6
	for j := 1; j <= jobCount; j++ {
		jobs <- j
	}
	close(jobs) // 关闭任务队列 → worker 的 range 循环随之结束
	for i := 0; i < jobCount; i++ {
		<-done // 收完所有结果即可，具体哪个 worker 干的并不重要
	}
	fmt.Println("全部任务完成")

	// 心法："不要通过共享内存来通信，而要通过通信来共享内存。"
	// 能用 channel 传递数据所有权就不上锁；
	// 锁适合保护结构性状态（如缓存 map），channel 适合传递数据流。

	/*
	 * ---- 一句话总结 ----
	 * 无缓冲=握手同步，有缓冲=异步队列；
	 * 发送方 close、接收方 range；select 管多路+超时；
	 * worker pool 是最常用的并发骨架，背下来直接抄。
	 */
}
