// ============================================================
// 第6章 · 结构体（struct）
// ============================================================
// 运行: go run ./06_structs
// 学习要点:
//  1. 定义与字面量：字段名在前类型在后；推荐带字段名的初始化
//  2. 零值可用：结构体没有"未初始化"问题，字段全是各自零值
//  3. 构造函数惯例：NewXxx() 返回指针
//  4. 匿名嵌入(embedding)：用组合代替继承——Go 没有继承！
//  5. 结构体比较：== 逐字段比较
//  6. 匿名结构体 与 结构体标签(tag)
//
// ============================================================
package main

import "fmt"

// type + struct 定义结构体。对比 C 的 struct：写法几乎一样，
// 但 Go 里 type 定义的每个类型都自动拥有方法集（第8章），不再是裸数据袋。
type Point struct {
	X int // 字段名在前、类型在后（与 C 声明顺序相反，和变量声明一致）
	Y int
}

// Circle 嵌入 Point：匿名字段（只写类型不写字段名）。
// 这是 Go 实现"复用"的方式——组合优于继承。Circle 会"提升"(promote)Point 的字段和方法，
// 效果看着像继承，但【没有 class/extends 关键字，也不是 is-a 关系】。
type Circle struct {
	Point  // 嵌入后 c.X 是 c.Point.X 的语法糖
	Radius float64
}

// 构造函数惯例：Go 没有 C++/Java 式构造器，约定提供 NewXxx 返回指针。
// 返回指针的理由：避免大结构体拷贝；让调用方能修改它；保持一致的引用语义。
func NewCircle(x, y int, r float64) *Circle {
	return &Circle{Point: Point{X: x, Y: y}, Radius: r}
}

// 结构体标签(tag)：反引号包裹的 key:"value" 字符串，
// 编译器不管它，由反射/序列化库在运行期读取——encoding/json 就靠它（第13章实战）
type User struct {
	Name string `json:"name"`          // 序列化成 JSON 时字段名叫 "name"
	Age  int    `json:"age,omitempty"` // omitempty：零值时省略该字段
}

func main() {
	// ==== 1. 字面量创建 ====
	p1 := Point{X: 1, Y: 2}  // 推荐：带字段名（可读、不怕日后字段增删导致错位）
	p2 := Point{1, 2}        // 按位置初始化：字段一多就是灾难，别这么写
	var p3 Point             // 零值 {0 0}——所有字段都是各自类型的零值，直接可用！
	p4 := &Point{X: 3, Y: 4} // 对复合字面量取地址 → *Point，一行完成"创建+取指针"
	fmt.Println(p1, p2, p3, p4)

	// 字段全可比时，结构体支持 == 比较（逐字段比较）
	// 对比 Python 默认按对象身份(内存地址)比较；C 的 struct 根本不能用 ==
	fmt.Println(p1 == p2, p1 == p3)

	// ==== 2. 访问字段 ====
	p4.X = 30 // 通过指针访问字段不用 -> ！p4.X 自动解引用
	fmt.Println(*p4)
	// （C 要写 p->X 或 (*p).X，Go 统一用点号，语言层面抹平了值/指针差异）

	// ==== 3. 构造函数 ====
	c := NewCircle(0, 0, 5)
	fmt.Println(c.Radius)
	fmt.Println(c.X, c.Y) // 嵌入的字段被"提升"，像自己的字段一样直接访问

	// ==== 4. 匿名结构体：一次性数据结构 ====
	cfg := struct { // 不预定义类型，适合临时组装数据/测试桩
		Host string
		Port int
	}{Host: "localhost", Port: 8080}
	fmt.Println(cfg.Host, cfg.Port)

	// ==== 5. 嵌入 vs 继承 ====
	// Circle 不是 Point 的子类，只是"内部持有一个 Point 并转发访问"。
	outer := Circle{Point: Point{X: 1, Y: 1}, Radius: 2.5}
	fmt.Println(outer.Point.Y, outer.Y) // 两种访问方式等价
	// 对比：Java class Circle extends Point / Python class Circle(Point)。
	// Go 用嵌入+接口组合出继承的所有好处，还避开了菱形继承等深坑。

	// ==== 6. 标签只是元数据 ====
	fmt.Printf("%+v\n", User{Name: "小明", Age: 18}) // %+v 打印时带字段名
	// 标签内容可用反射(reflect 包)读取；记住结论即可：
	// tag 决定了 JSON 序列化的字段名和选项，第13章见真章

	/*
	 * ---- 一句话总结 ----
	 * 结构体 = C struct + 方法 + 嵌入 + 标签；
	 * 初始化带字段名、构造走 NewXxx、复用靠嵌入不靠继承；
	 * 零值结构体开箱即用是 Go API 设计的重要习惯。
	 */
}
