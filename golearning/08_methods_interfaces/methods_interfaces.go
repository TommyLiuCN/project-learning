// ============================================================
// 第8章 · 方法与接口
// ============================================================
// 运行: go run ./08_methods_interfaces
// 学习要点:
//  1. 方法 = 带接收者的函数；值接收者 vs 指针接收者
//  2. 接口：隐式实现（没有 implements 关键字）——编译期检查的鸭子类型
//  3. 面向接口编程与小接口组合（io.Reader 哲学）
//  4. 类型断言与 type switch：从接口还原具体类型
//  5. any = interface{}：空接口万物皆可装
//  6. fmt.Stringer：实现 String() 即可定制打印
//
// ============================================================
package main

import (
	"fmt"
	"math"
)

// ==== 1. 方法：带"接收者"的函数 ====
type Rect struct {
	W, H float64
}

// 值接收者：方法内操作副本，改不动原结构体。适合只读场景和小结构体。
func (r Rect) Area() float64 {
	return r.W * r.H
}

// 指针接收者：能修改原结构体；大结构体避免拷贝开销——实际项目中的主流选择。
// 规则记忆：【改得动改不动】和【要不要省拷贝】决定用哪种；
// 同一类型的所有方法最好统一用一种接收者，保持方法集一致。
func (r *Rect) Scale(k float64) {
	r.W *= k // Go 自动解引用：(*r).W 的简写
	r.H *= k
}

// 实现 fmt.Stringer 接口（见下文第7节）：类似 Python 的 __str__ 方法
func (r Rect) String() string {
	return fmt.Sprintf("Rect(%v×%v)", r.W, r.H)
}

// ==== 2/3. 接口定义与隐式实现 ====
// 接口 = 一组方法签名的集合。一个类型只要拥有这些方法就算实现了接口，
// 【完全不需要声明 implements】——像 Python 鸭子类型，但错误在编译期暴露！
type Shape interface {
	Area() float64
}

type Circle struct {
	R float64
}

// Circle 定义了 Area 方法 → 自动满足 Shape，一行声明都不用写
// （对比 Java: class Circle implements Shape / Python: 继承 ABC 才算数）
func (c Circle) Area() float64 { return math.Pi * c.R * c.R }

func describe(s Shape) string { // 面向接口编程：参数用小接口，能接的类型最广
	return fmt.Sprintf("%T 面积=%.2f", s, s.Area()) // %T 打印接口里的动态类型
}

// 小接口可以组合成大接口（效果类似继承，本质还是嵌入）：
type Shape2D interface {
	Shape           // 嵌入 Shape：自动获得 Area()
	String() string // 再追加一个方法要求
}

var _ Shape2D = Rect{} // 编译期断言惯用写法：确认 Rect 实现了 Shape2D

func main() {
	rect := Rect{W: 3, H: 4}
	rect.Scale(2) // rect 是可寻址变量，Go 自动取址：等价于 (&rect).Scale(2)
	fmt.Println(rect.String(), rect.Area())

	rp := &rect
	fmt.Println(rp.Area()) // 指针调用值接收者方法也合法（自动解引用）

	// ==== 3. 多态 ====
	shapes := []Shape{ // 接口切片里可以混装任何实现了该接口的类型
		Rect{W: 2, H: 5},
		Circle{R: 1},
	}
	for _, s := range shapes {
		fmt.Println(describe(s)) // 同一接口，不同行为——多态
	}

	// ==== 4. 类型断言与 type switch ====
	var anyShape Shape = Rect{W: 1, H: 1}
	// 断言成具体类型 x.(T)：失败会 panic；comma-ok 形式则安全返回布尔值
	rect2, ok := anyShape.(Rect)
	fmt.Println(rect2, ok)
	_, ok2 := anyShape.(Circle)
	fmt.Println(ok2) // false：装的是 Rect 不是 Circle

	// type switch：批量判断动态类型（比 C 的 void* 强转优雅太多）
	for _, s := range shapes {
		switch v := s.(type) { // 特殊语法：switch x.(type)，每个 case 里 v 是对应类型
		case Rect:
			fmt.Println("矩形:", v.W, "×", v.H) // 这里 v 已是 Rect
		case Circle:
			fmt.Println("圆形: 半径", v.R) // 这里 v 已是 Circle
		default:
			fmt.Println("未知图形")
		}
	}

	// ==== 5. any 与空接口 ====
	// interface{}（新别名 any）没有任何方法 → 所有类型都实现它。
	// 类似 Python 的动态变量 / Java 的 Object，但取出时必须断言回具体类型
	box := []any{1, "hello", 3.14, Rect{1, 1}}
	for _, item := range box {
		fmt.Printf("%v(%T) ", item, item)
	}
	fmt.Println()

	// ==== 6. fmt.Stringer ====
	// 只要实现 String() string，fmt 全家桶打印时会自动调用它
	fmt.Println(rect) // 不再输出 {4 8}，而是我们定制的描述文字

	/*
	 * ---- 一句话总结 ----
	 * 方法=函数+接收者，改动数据用指针接收者；
	 * 接口靠方法集隐式满足——"我有什么行为我就是什么"；
	 * 断言取回具体类型，Stringer 定制打印；小接口是 Go 抽象的灵魂。
	 */
}
