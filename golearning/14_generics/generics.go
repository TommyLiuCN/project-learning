// ============================================================
// 第14章 · 泛型（Go 1.18+）
// ============================================================
// 运行: go run ./14_generics
// 学习要点:
//  1. 泛型函数：类型参数 [T any]，一套代码服务多种类型
//  2. 约束(constraint)：限定类型参数的能力范围
//  3. ~ 底层类型 与 内置约束 comparable / cmp.Ordered
//  4. 泛型类型：泛型结构体容器
//  5. 泛型 vs 接口 vs C 模板 vs Python 鸭子类型
//
// ============================================================
package main

import (
	"cmp"
	"fmt"
)

// ==== 1. 泛型函数 ====
// [T any] 声明类型参数 T，约束是 any（任意类型）。
// 对照：C++ template<typename T> 编译期展开；Python 不检查类型自然用不着；
// Go 1.18 之前只能 interface{} + 类型断言，又丑又不安全。
func Print[T any](label string, vals ...T) { // 泛型和可变参数可以组合
	fmt.Printf("%s: %v\n", label, vals)
}

// ==== 2. 约束：限定能力 ====
// cmp.Ordered 是标准库约束：所有支持 < > 比较的类型（各种整型/浮点/字符串）。
// 有了 Ordered 约束，编译器才允许在函数体内写 a > b。
func Max[T cmp.Ordered](a, b T) T {
	if a > b {
		return a
	}
	return b
}

// 自定义约束：接口里直接列出允许的【类型集合】，竖线表示"或"。
// ~int 表示"底层类型是 int"的所有类型——包括 type MyInt int 这类自定义类型
type Number interface {
	~int | ~int64 | ~float64
}

func Sum[T Number](nums []T) T { // 一个 Sum 通吃多种数值切片
	var total T // 零值起步，var total T 在泛型里完全合法
	for _, n := range nums {
		total += n
	}
	return total
}

// ==== 3. comparable：内置约束，表示支持 == 和 != 的类型 ====
func IndexOf[T comparable](haystack []T, needle T) int {
	for i, v := range haystack {
		if v == needle {
			return i
		}
	}
	return -1
}

// ==== 4. 泛型类型 ====
// 泛型栈。对照：C++ 的 template<class T> class stack；
// Java 泛型是编译期擦除的假泛型；Go 是真·类型安全的运行期实现
type Stack[T any] struct {
	items []T
}

func (s *Stack[T]) Push(v T) { s.items = append(s.items, v) }

func (s *Stack[T]) Pop() (T, bool) { // comma-ok 风格：空栈返回零值 + false
	var zero T
	if len(s.items) == 0 {
		return zero, false
	}
	v := s.items[len(s.items)-1]
	s.items = s.items[:len(s.items)-1]
	return v, true
}

func (s *Stack[T]) Len() int { return len(s.items) }

func main() {
	Print("整数", 1, 2, 3)
	Print("字符串", "a", "b")

	// 类型推断：多数情况不用手写 [int]，编译器看实参就知道
	fmt.Println(Max(3, 9), Max("苹果", "香蕉")) // 字符串按字节字典序比较
	fmt.Println(Max[float64](2.5, -1))      // 推断不出时显式指定

	fmt.Println(Sum([]int{1, 2, 3}), Sum([]float64{1.5, 2.5}))

	fmt.Println(IndexOf([]string{"x", "y"}, "y"), IndexOf([]int{1, 2}, 9))

	// 泛型类型使用时需要实例化：Stack[int]、Stack[string] 是不同的类型
	st := Stack[string]{}
	st.Push("go")
	st.Push("rust")
	v, ok := st.Pop()
	fmt.Println(v, ok, st.Len())

	_, empty := st.Pop()
	_, empty2 := st.Pop()
	fmt.Println(empty, empty2) // true false：第二次弹出时栈已空

	// ==== 5. 选型心法 ====
	// 泛型：类型安全地复用【算法和容器】（数据结构层面）
	// 接口：抽象【行为】、实现多态（能力层面）
	// 官方建议：先用接口和具体类型把逻辑写对，发现确实重复了再上泛型，
	// 不要为了泛型而泛型

	/*
	 * ---- 一句话总结 ----
	 * [T 约束] 三段式是全部语法；
	 * 能力看约束(Ordered/comparable)，复用容器算法选泛型、抽象行为选接口。
	 */
}
