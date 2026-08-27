// ============================================================
// 第1章 · Hello World 与程序入口
// ============================================================
// 运行: go run ./01_hello
// 学习要点:
//  1. package main 与 func main：Go 程序的固定入口
//  2. import 导入包：没用到的包会直接编译错误（和 C/Python 都不同！）
//  3. fmt.Println / Printf：格式化输出与常用动词
//  4. os.Args 命令行参数（对应 C 的 argv、Python 的 sys.argv）
//  5. 注释写法
//
// ============================================================
package main

// 对比 C 的 #include <stdio.h>：Go 导入的是"包"而非头文件，
// 编译器直接读源码提取 API 信息，不需要 .h 那样的声明文件。
import (
	"fmt"
	"os"
)

// 每个可执行程序必须有且只有一个 main 包 + main 函数。
// 对比 C: int main(void)；对比 Python: 脚本从第一行顺序执行、没有入口函数。
// 注意：main 没有返回值也不收参数——命令行参数要用 os.Args 获取。
func main() {
	fmt.Println("Hello, Go!") // 自动换行，多个参数之间自动加空格

	// ==== 1. Printf 格式化输出（和 C 的 printf 几乎一样）====
	name := "Gopher"
	n := 42
	fmt.Printf("名字=%s 数字=%d 十六进制=%#x 浮点=%.2f\n", name, n, 255, 3.14159)

	// 常用动词：%d 整数 %s 字符串 %f 浮点 %t 布尔
	// %v 万能动词(默认格式) %+v 结构体带字段名 %T 打印类型——后两个是 Go 特色
	fmt.Printf("万能动词 %v, 类型 %T\n", n, n)

	// ==== 2. 命令行参数 ====
	// C: int main(int argc, char *argv[])；Python: sys.argv；Go: os.Args 字符串切片
	// os.Args[0] 是程序自身路径，真实参数从 [1] 开始（和 C/Python 一致）
	fmt.Println("命令行参数个数:", len(os.Args))
	if len(os.Args) > 1 {
		fmt.Println("第一个参数:", os.Args[1])
	}

	// ==== 3. 注释 ====
	// 单行注释，和 C++/Java 一样。
	/*
	   块注释也和 C 相同，可以跨行。
	   但 Go 社区惯例：文档注释统一以 // 开头（godoc 只认这种）。
	*/

	// Println 自动换行；Print 不换行
	fmt.Print("不换行的输出 ")
	fmt.Print("还是同一行\n")

	/*
	 * ---- 一句话总结 ----
	 * package main + func main 是铁打入口；import 了就必须用；
	 * fmt 的 %v/%T 动词是排查类型的利器。
	 */
}
