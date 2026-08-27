// ============================================================
// 第2章 · 变量、常量与基本类型
// ============================================================
// 运行: go run ./02_vars_types
// 学习要点:
//  1. 变量声明的三种方式：var、var 块、:= 短声明
//  2. 未使用的局部变量/导入直接编译报错（强制整洁，和 C/Python 都不同）
//  3. 基本类型与零值：Go 没有"未初始化的垃圾值"
//  4. 显式类型转换：连 int→float64 都要手写（比 C 更严格）
//  5. const 常量与 iota 枚举
//  6. 字符串 ↔ 数字互转：strconv 包
//
// ============================================================
package main

import (
	"fmt"
	"strconv"
)

// 包级变量必须用 var 声明（:= 只能用在函数内，新手高频踩坑点）
var globalCounter = 100 // 类型可省略，由右侧推断

// const 类似 C 的 const/#define：编译期确定、不可修改；
// Python 没有真常量（只有全大写命名的君子协定）
const Pi = 3.14159

func main() {
	// ==== 1. 变量声明 ====
	var a int = 10        // 完整写法
	var b = 20            // 类型推断（像 C++ 的 auto）
	c := 30               // 短声明：最常用！但只能出现在函数内部
	var x, y = 1, "hello" // 一次声明多个，类型可以不同
	fmt.Println(a, b, c, x, y)

	// i := 99 若声明后不用，编译器直接报错 "declared and not used"：
	// C/Python 都不管这事，Go 强制你保持整洁

	// ==== 2. 基本类型与零值 ====
	// 整数位数固定不随平台变化（这点像 Java 不像 C）：
	//   int8/int16/int32/int64 明确位数；int 在 64 位平台就是 64 位；uint 无符号版
	// byte 是 uint8 的别名；rune 是 int32 的别名，表示一个 Unicode 码点
	var (
		i  int     = -5
		u  uint    = 42
		f  float64 = 3.14 // 默认浮点是 float64；没有单独叫 float 的类型
		s  string         // 零值是 ""（不是 null！）
		ok bool           // 零值是 false
		z  int            // 零值是 0
	)
	// 重要概念「零值」：变量声明即自动初始化为对应零值，
	// 不存在 C 里"读到随机垃圾值"的问题，也没有 Python 的未定义变量
	fmt.Println(i, u, f, s == "", ok, z)

	// Go 的 if 条件必须是布尔值：if(1) 在 C 能编译，在 Go 直接报错（同 Java）

	// ==== 3. 显式类型转换 ====
	// T(v) 语法。Go 强类型且【没有任何隐式转换】：
	// int 和 float64 直接相加编译不过——C 会悄悄帮你转，Python 会自动提升
	d := float64(i) / 2 // -2.5；若直接写 i / 2 是整数除法得 -2（向零截断，同 C）
	fmt.Println(d)

	// 大→小的转换可能溢出但不报错（和 C 相同的静默截断），运行期才发生
	var big int = 300
	small := uint8(big) // 300 % 256 = 44
	fmt.Println(small)

	// rune 就是 int32：一个汉字是一个 rune（按码点数一个），但 UTF-8 编码占 3 字节
	r := '中'
	fmt.Printf("%c 的码点是 %d, 类型是 %T\n", r, r, r)

	// ==== 4. const 与 iota ====
	const StatusOK = 200 // 未指定类型的常量是"无类型常量"，精度无限，用的时候才定型
	fmt.Println(Pi, StatusOK)

	// iota：const 块里的自增行号，从 0 开始——Go 惯用的枚举写法
	type Weekday int
	const (
		Sunday    Weekday = iota // 0
		Monday                   // 1（后续行自动重复上一行的表达式 = iota）
		Tuesday                  // 2
		Wednesday                // 3
	)
	fmt.Println(Sunday, Monday, Wednesday)

	// 常见技巧：配合位运算做标志位（Linux 文件权限既视感）
	const (
		Read  = 1 << iota // 1<<0 = 1
		Write             // 1<<1 = 2
		Exec              // 1<<2 = 4
	)
	fmt.Println(Read, Write, Exec)

	// ==== 5. 字符串 ↔ 数字（strconv 包）====
	// Python 用 int("123")/str(456)；Go 一律走 strconv，且返回 (值, 错误) 双返回值
	n, err := strconv.Atoi("123") // Atoi = ASCII to integer；错误处理第9章细讲
	fmt.Println(n+1, err == nil)

	fv, _ := strconv.ParseFloat("3.14", 64) // _ 显式丢弃错误（确定不会错时才这么干）
	fmt.Println(fv)

	s2 := strconv.Itoa(456) // int → string
	fmt.Println(s2 + "!")

	// 字符串拼接：+ 号即可；大量循环拼接请用 strings.Builder（第12章）
	msg := "Go" + "学习"
	fmt.Println(msg)

	// fmt.Sprintf：格式化成字符串而不打印（类比 C 的 sprintf / Python 的 f-string）
	info := fmt.Sprintf("%s 共 %d 章", msg, 14)
	fmt.Println(info)

	_ = globalCounter // 包级变量不使用也不报错，只有【局部】变量受"必须使用"限制

	/*
	 * ---- 一句话总结 ----
	 * 函数内优先 :=；一切变量有零值；类型转换必须显式写 T(v)；
	 * 枚举靠 const + iota；字符串数字互转找 strconv。
	 */
}
