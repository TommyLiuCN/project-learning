// ============================================================
// 第13章 · 文件 IO 与 JSON
// ============================================================
// 运行: go run ./13_file_io_json
// 学习要点:
//  1. 快捷读写：os.ReadFile / os.WriteFile（小文件一把梭）
//  2. 传统流程：os.Open + defer Close + 错误检查
//  3. bufio.Scanner 逐行读（大文件标配，对照 Python for line in f）
//  4. io.Writer/io.Reader 接口：一切皆流
//  5. encoding/json：结构体标签、Marshal/Unmarshal
//
// ============================================================
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// json 标签决定序列化后的字段名；不加标签时使用导出字段的原名
type Student struct {
	Name   string    `json:"name"`
	Age    int       `json:"age"`
	Scores []float64 `json:"scores,omitempty"` // omitempty：零值时省略该字段
}

func main() {
	dir, err := os.MkdirTemp("", "golearning") // 系统调用失败通常直接 fail fast
	if err != nil {
		panic(err)
	}
	defer os.RemoveAll(dir)                // 学完即清理临时目录
	path := filepath.Join(dir, "demo.txt") // filepath.Join 跨平台拼路径，别手动拼 "/"

	// ==== 1. 快捷方式：一次读完/写完（适合小文件）====
	content := "第一行\n第二行\n第三行\n"
	err = os.WriteFile(path, []byte(content), 0644) // 0644 是 Unix 风格权限位
	if err != nil {
		panic(err)
	}

	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	fmt.Print(string(data)) // []byte ↔ string 直接转换

	// ==== 2. 传统流程 + bufio 逐行读 ====
	// 对照 Python: with open(path) as f: for line in f:
	// Go 三件套：os.Open + defer Close + bufio.Scanner
	f, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer f.Close() // 第4章 defer 的经典应用：之后无论怎么 return 都保证关闭

	scanner := bufio.NewScanner(f)
	lineNo := 0
	for scanner.Scan() { // Scan 一行行推进，读完返回 false 结束循环
		lineNo++
		fmt.Printf("%d: %s\n", lineNo, scanner.Text())
	}
	if err := scanner.Err(); err != nil { // 别忘了检查扫描器自身的错误！
		panic(err)
	}
	// 注意：Scanner 默认单行上限 64KB，超大行需 scanner.Buffer 调大或改用 bufio.Reader

	// ==== 3. 追加写 + bufio.Writer ====
	f2, _ := os.OpenFile(path, os.O_APPEND|os.O_WRONLY|os.O_CREATE, 0644)
	// 标志组合拳 O_APPEND/O_WRONLY/O_CREATE 比 C fopen("a") 的模式串更直观
	w := bufio.NewWriter(f2) // 带缓冲写入：攒一批再落盘，减少系统调用次数
	w.WriteString("追加的一行\n")
	w.Flush() // 缓冲不满也要手动刷盘！忘记 Flush 数据就丢了（新手高频坑）
	f2.Close()

	final, _ := os.ReadFile(path)
	fmt.Println("总行数:", strings.Count(string(final), "\n"))

	// io.Writer/io.Reader 是 Go IO 世界的万能接口：
	// 文件、网络连接、HTTP body、strings.Builder……全都实现它们，
	// 所以同一套 bufio/fmt/json 代码可以无缝换数据源——接口哲学的最佳示范

	// ==== 4. JSON ====
	stu := Student{Name: "小明", Age: 18, Scores: []float64{92.5, 88}}

	jsonBytes, err := json.Marshal(stu) // 结构体 → JSON 字节串
	fmt.Println(string(jsonBytes), err == nil)

	pretty, _ := json.MarshalIndent(stu, "", "  ") // 带缩进的人类友好版本
	fmt.Println(string(pretty))
	// 序列化规则：只有【导出字段】(首字母大写)会参与 JSON；标签决定输出字段名

	// 反序列化：JSON → 结构体。必须传指针才能把数据写进去！
	var stu2 Student
	if err := json.Unmarshal(jsonBytes, &stu2); err != nil {
		panic(err)
	}
	fmt.Println(stu2.Name, stu2.Age, stu2.Scores)

	// 懒得定义结构体？map[string]any 也能接，代价是失去类型检查
	var obj map[string]any
	_ = json.Unmarshal(jsonBytes, &obj)
	fmt.Println(obj["name"], obj["age"])
	// 注意：JSON 数字统一解析成 float64——大整数精度场景要小心

	/*
	 * ---- 一句话总结 ----
	 * 小文件 ReadFile/WriteFile，大文件 Scanner 流式读；
	 * defer Close + Flush 是保命符；
	 * JSON 靠结构体标签映射，Unmarshal 必传指针。
	 */
}
