// ============================================================
// 第4章 · 函数
// ============================================================
// 运行: go run ./04_functions
// 学习要点:
//  1. 多返回值：Go 招牌特性，错误处理的基石（第9章展开）
//  2. 命名返回值与裸 return
//  3. 可变参数 ...T（类似 Python 的 *args）
//  4. 函数是一等公民：赋值给变量、当参数传递（像 Python，不像 C）
//  5. 匿名函数与闭包：捕获外部变量的引用
//  6. defer 延迟执行：对标 Python with / C++ RAII
//
// ============================================================
package main

import (
	"errors"
	"fmt"
)

// func 名字(参数) 返回类型。参数名在前、类型在后（与 C 相反）。
// 相邻同类型参数可以合并声明：a, b int
func add(a, b int) int {
	return a + b
}

// ==== 1. 多返回值 ====
// C 返回多个值得靠出参指针或结构体；Python 靠元组打包。
// Go 原生多返回值，最常见模式：(结果, error)。出错时结果返回零值。
func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, errors.New("除数不能为零")
	}
	return a / b, nil // nil 是 Go 的"空"，类似 C 的 NULL、Python 的 None
}

// ==== 2. 命名返回值 ====
// 给返回值起名后它就是预声明的局部变量（从零值起步），
// 好处是文档性强，还支持裸 return；大函数里慎用裸 return（可读性反噬）
func swap(a, b int) (first, second int) {
	first = b
	second = a
	return // 裸 return：等价于 return first, second
}

// ==== 3. 可变参数 ====
// ...int 表示任意个 int 参数，函数体内 nums 是 []int 切片。
// 类似 Python 的 def f(*args)。调用方还能把切片展开传入：f(slice...)
func sumAll(nums ...int) int {
	total := 0
	for _, n := range nums {
		total += n
	}
	return total
}

// 函数是一等公民：函数类型写作 func(参数列表) 返回值
// 对比 C 只能传函数指针（还没有闭包）；Python 天生如此，Go 同样支持
func apply(f func(int, int) int, a, b int) int {
	return f(a, b)
}

// 闭包工厂：内部匿名函数捕获了外部的 count 变量【本身】（引用语义）
// 外层函数返回后 count 依然存活——编译器的逃逸分析会把它搬到堆上由 GC 管理
func makeCounter() func() int {
	count := 0
	return func() int {
		count++
		return count
	}
}

func main() {
	fmt.Println(add(1, 2))

	// 多返回值的接收：不需要的值用 _ 丢弃
	result, err := divide(10, 4)
	fmt.Println(result, err == nil)
	_, err2 := divide(1, 0)
	fmt.Println("错误信息:", err2) // error 本质是接口，直接打印就是消息文本（第9章）

	f, s := swap(3, 9)
	fmt.Println(f, s)

	fmt.Println(sumAll(1, 2, 3), sumAll()) // 一个参数都不传也合法：nums 是空切片

	list := []int{4, 5, 6}
	fmt.Println(sumAll(list...)) // 切片展开为可变参数

	// ==== 4. 函数是一等公民 ====
	op := add               // 函数赋给变量（没有括号！加了括号是调用）
	fmt.Println(op(10, 20)) // 像 Python 把函数当对象传来传去；C 只能传函数指针
	fmt.Println(apply(op, 1, 2))

	double := func(x int) int { return x * 2 } // 匿名函数（函数字面量）
	fmt.Println(double(21))

	// ==== 5. 闭包 ====
	counter := makeCounter()
	fmt.Println(counter(), counter(), counter()) // 1 2 3：count 被"记住"了
	c2 := makeCounter()                          // 每次 makeCounter 都产生独立的新闭包
	fmt.Println(c2())

	// 经典陷阱：循环变量捕获。（Go 1.22 起已修复：每次迭代都是新变量）
	funcs := []func() int{}
	for i := 0; i < 3; i++ {
		funcs = append(funcs, func() int { return i })
	}
	fmt.Println(funcs[0](), funcs[1](), funcs[2]()) // 输出 0 1 2（老版本 Go 会输出 3 3 3）

	// ==== 6. defer 延迟执行 ====
	// defer 把调用压入栈，在当前函数 return 之后按【后进先出】顺序执行。
	// 对照：Python 的 with open(...) / try-finally；C++ 的 RAII 析构；
	//       C 只能在每个 return 前手动重复清理代码，极易泄漏资源。
	defer fmt.Println("defer A: 最先注册，最后执行")
	defer fmt.Println("defer B: 中间注册")
	defer fmt.Println("defer C: 最后注册，最先执行")

	fmt.Println("正常代码先跑完...")
	// 典型用法：打开文件后立刻 defer f.Close()，之后无论多少个 return/panic 都不会忘关

	/*
	 * ---- 一句话总结 ----
	 * (结果, error) 双返回值是全语言最核心惯例；闭包捕获引用；
	 * 循环里起 goroutine 记得传参快照；defer 负责"谁申请谁清理"。
	 */
}
