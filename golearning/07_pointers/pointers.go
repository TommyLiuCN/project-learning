// ============================================================
// 第7章 · 指针
// ============================================================
// 运行: go run ./07_pointers
// 学习要点:
//  1. & 取地址、* 解引用——语法与 C 相同
//  2. 没有指针运算！(p+1 那套没了) —— 内存安全的关键取舍
//  3. Go 所有传参都是值传递；切片/map/指针"看起来像引用"的真相
//  4. new(T) vs &T{} vs make 三兄弟
//  5. nil 指针：判空习惯同 C；解引用 nil 是 panic 不是段错误
//  6. 逃逸分析与垃圾回收：不需要 free/delete
//
// ============================================================
package main

import "fmt"

type Counter struct {
	Name string
	N    int
}

// 值传递：拿到的是拷贝，改不动外面的（和 C 一样）
func tryModify(n int) { n = 100 }

// 传指针：拷贝的是地址，能改到原始数据（和 C 一样）
func reallyModify(n *int) { *n = 100 }

// 切片/map 参数也是值传递，但它们内部装着指向底层数据的指针，
// 所以函数内修改会反映到外面——这是第5章切片头知识的延续
func changeSlice(s []int)        { s[0] = 111 }
func changeMap(m map[string]int) { m["k"] = 222 }

// 返回局部变量的指针：【完全合法】！
// C 里这是悬垂指针未定义行为；Go 的逃逸分析会把变量搬到堆上，GC 保证它活着
func newCounter(name string) *Counter {
	c := Counter{Name: name} // 局部变量
	return &c                // 逃逸到堆，安全返回
}

func main() {
	// ==== 1. 基本用法 ====
	x := 42
	p := &x            // & 取地址；p 的类型是 *int
	fmt.Println(p, *p) // 打印地址(形如 0xc000...) 和值
	*p = 100           // 解引用赋值 → x 变成 100
	fmt.Println(x)

	// ==== 2. 没有指针运算 ====
	// *(p + 1)、p++ 这类操作直接编译错误！
	// C 里指针加减=按元素大小移动内存位置，越界全靠自觉；
	// Go 干脆禁止，要移动就老老实实用下标或切片表达式
	// 指针只能比较相等/不等（同一底层数组内），不能做算术也不能比大小
	arr := [3]int{1, 2, 3}
	i := &arr[0]
	k := &arr[0]
	j := &arr[1]
	fmt.Println(i == k, i == j) // true false：指向同一元素才相等
	fmt.Println(*i == *j)       // 值层面的比较照常进行：false (1 != 2)

	// ==== 3. 值传递的真相 ====
	a := 1
	tryModify(a)
	fmt.Println("值传递没改动:", a)
	reallyModify(&a)
	fmt.Println("传指针改动了:", a)

	sl := []int{1, 2, 3}
	m := map[string]int{"k": 1}
	changeSlice(sl)
	changeMap(m)
	fmt.Println(sl[0], m["k"]) // 都变了——因为拷贝的头部里带着指向数据的指针

	// 结论：Go 只有值传递。想被修改 → 传指针；
	// 切片/map/通道天生自带指针所以"免传"，但 append 扩容/重新赋值仍需注意

	// ==== 4. new / &T{} / make ====
	q := new(int)     // new(T)：分配 T 的零值并返回 *T，等价于 var v T; q = &v
	cnt := &Counter{} // 结构体惯用 &T{...}：创建并取址一步到位
	*q = 7
	cnt.N = 3
	fmt.Println(*q, cnt.N)

	s := make([]int, 2) // make 只用于三种引用类型：切片/map/通道，会初始化内部结构
	fmt.Println(s)
	// new([]int) 得到的是"指向 nil 切片的指针"，几乎没用——别混用！

	// ==== 5. 返回局部变量的指针（逃逸分析）====
	c := newCounter("订单计数")
	fmt.Println(c.Name, c.N) // 完全安全：c 已逃逸到堆上

	// ==== 6. nil 指针 ====
	var np *int // 未初始化的指针零值是 nil（不是 C 的野指针随机值！）
	fmt.Println(np == nil)
	if np != nil { // 使用前判空，习惯和 C 相同
		fmt.Println(*np)
	}
	// 解引用 nil 指针 → runtime panic（可被 recover 捕获，第9章），
	// 不是 C 的段错误直接崩进程；下面这行取消注释可以感受一下：
	// fmt.Println(*np)

	// 内存管理总结：不需要 malloc/free 配对，也没有 C++ 的 delete，
	// 垃圾回收器全程托管。性能敏感场景再研究 sync.Pool / 内存对齐即可。

	/*
	 * ---- 一句话总结 ----
	 * 指针语法照搬 C 但砍掉了算术；一切传参皆拷贝；
	 * new 少用 &T{} 多用 make 专属三兄弟；nil 解引用是 panic 而非段错误；
	 * 逃逸分析+GC 让"返回局部变量地址"从 UB 变成日常。
	 */
}
