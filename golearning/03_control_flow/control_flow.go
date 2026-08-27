// ============================================================
// 第3章 · 流程控制
// ============================================================
// 运行: go run ./03_control_flow
// 学习要点:
//  1. if：条件不加括号、{ 不能另起一行；还能带初始化语句
//  2. for 是唯一的循环关键字（没有 while / do-while）
//  3. range 遍历：切片/映射/字符串（按 rune 不拆中文）/通道
//  4. switch：默认自动 break、case 可多值可表达式、type switch
//  5. 标签 label：break/continue 直接作用于外层循环
//
// ============================================================
package main

import "fmt"

func main() {
	// ==== 1. if / else if / else ====
	score := 85
	// 与 C/Java 区别：条件不用括号；左花括号必须和 if 同行（gofmt 强制）
	if score >= 90 {
		fmt.Println("优秀")
	} else if score >= 80 {
		fmt.Println("良好")
	} else {
		fmt.Println("继续加油")
	}

	// 特色：if 可带一条初始化语句，分号后接条件
	// 初始化的变量作用域仅限整个 if/else 链——避免污染外部作用域
	if n := score / 10; n >= 9 {
		fmt.Println("档位:", n)
	} else {
		fmt.Println("十位档:", n) // else 分支里 n 仍可见
	}
	// fmt.Println(n) // 编译错误：n 已超出作用域

	// ==== 2. for 的三种形态 ====
	// 形态1：经典三段式（等同 C 的 for）
	for i := 0; i < 3; i++ {
		fmt.Print(i, " ")
	}
	fmt.Println()

	// 形态2：只留条件 = 其他语言的 while（Go 没有 while 关键字！）
	count := 3
	for count > 0 {
		fmt.Print(count, " ")
		count--
	}
	fmt.Println()

	// 形态3：无限循环 = while(true)，靠 break 跳出
	sum := 0
	for {
		sum++
		if sum >= 5 {
			break
		}
	}
	fmt.Println("sum =", sum)

	// 没有 do-while；要模拟就 for { ...; if !cond { break } }

	// ==== 3. range 遍历 ====
	nums := []int{10, 20, 30} // 切片，第5章详讲
	// 返回 (下标, 值)：像 Python 的 enumerate(nums)
	for idx, val := range nums {
		fmt.Printf("nums[%d]=%d ", idx, val)
	}
	fmt.Println()

	for _, val := range nums { // _ 丢弃不需要的下标（Go 不允许存在未使用变量）
		fmt.Print(val, " ")
	}
	fmt.Println()

	// range 字符串按【rune(码点)】迭代而不是字节——中文不会被拆碎！
	// C 遍历 char* 是按字节；Python3 的 str 也是按码点，这里和 Python 一致
	for i, r := range "你好" {
		fmt.Printf("(%d:%c) ", i, r) // 注意 i 是字节偏移：0 和 3
	}
	fmt.Println()

	// range map：顺序故意随机化，防止你依赖遍历顺序
	ages := map[string]int{"张三": 20, "李四": 25}
	for name, age := range ages {
		fmt.Printf("%s=%d ", name, age)
	}
	fmt.Println()

	// ==== 4. switch ====
	// 最大区别：每个 case 自带 break！不像 C 会贯穿到下一个 case
	day := 6
	switch day {
	case 1, 2, 3, 4, 5: // 多个值逗号分隔
		fmt.Println("工作日")
	case 6, 7:
		fmt.Println("周末")
	default:
		fmt.Println("非法日期")
	}
	// 真想要贯穿行为：显式写 fallthrough（很少用到）

	// 无表达式的 switch = 更清爽的 if-else 链（神似 Python 3.10 的 match-case）
	switch h := 21; {
	case h < 12:
		fmt.Println("上午")
	case h < 18:
		fmt.Println("下午")
	default:
		fmt.Println("晚上")
	}

	// case 可以是任意非常量表达式（C 要求编译期常量，Python 的 case 要常量模式）
	switch {
	case score > 90 && day < 6:
		fmt.Println("学霸的工作日")
	default:
		fmt.Println("普通的一天")
	}

	// type switch：判断接口里的动态类型，第8章接口部分实战

	// ==== 5. break/continue 与标签 ====
outer: // 定义标签
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if j == 1 {
				continue outer // 直接跳过外层本轮——C/Python 的 continue 做不到
			}
			if i == 2 {
				break outer // 直接跳出外层——比 C 的标志位写法优雅
			}
			fmt.Printf("(%d,%d) ", i, j)
		}
	}
	fmt.Println()

	/*
	 * ---- 一句话总结 ----
	 * 一个 for 打天下；range 是遍历瑞士军刀；
	 * switch 默认不贯穿、无表达式更好用；label 解决嵌套循环跳出难题。
	 */
}
