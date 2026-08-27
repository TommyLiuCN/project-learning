// ============================================================
// 第12章 · 常用标准库速览
// ============================================================
// 运行: go run ./12_stdlib
// 学习要点:
//  1. strings：字符串全家桶 + Builder 高效拼接
//  2. strconv：字符串 ↔ 数字（第2章的延伸）
//  3. time：时间与格式化——参考时间的"布局魔法"
//  4. math / math/rand：数学函数与随机数
//  5. sort：排序与自定义比较
//  6. unicode/utf8：中文处理的正确姿势（字节 vs 字符）
//
// ============================================================
package main

import (
	"fmt"
	"math"
	"math/rand"
	"sort"
	"strconv"
	"strings"
	"time"
	"unicode/utf8"
)

type Person struct { // 给 sort.Slice 演示用的结构体
	Name string
	Age  int
}

func main() {
	// ==== 1. strings 包 ====
	s := "Hello, Go 语言!"
	fmt.Println(strings.Contains(s, "Go"), strings.Count(s, "l"))
	fmt.Println(strings.ToUpper(s), strings.Repeat("ab", 3))

	parts := strings.Split("a,b,c", ",")
	fmt.Println(parts, strings.Join(parts, "-")) // split/join 一对，同 Python
	fmt.Println(strings.Fields(" go  is  fun ")) // 按任意空白切分，比 Split(" ") 智能

	fmt.Println(strings.ReplaceAll("2024-01-01", "-", "/"))
	fmt.Println(strings.TrimSpace("  hi  "), strings.HasPrefix("gopher", "go"))

	// 大量拼接用 strings.Builder：内部攒 []byte，最后一次成型。
	// 对照 Python 的 "".join(list)、C++ 的 ostringstream；
	// 循环里 s += x 在两种语言里都是性能反模式
	var b strings.Builder
	for i := 0; i < 3; i++ {
		fmt.Fprintf(&b, "item%d;", i) // Builder 实现了 io.Writer 接口，fmt 可直写
	}
	fmt.Println(b.String())

	// ==== 2. strconv 补充 ====
	n, _ := strconv.ParseInt("ff", 16, 64) // 指定进制解析
	fmt.Println(n, strconv.FormatFloat(3.14159, 'f', 2, 64))

	// ==== 3. time 包 ====
	now := time.Now()
	fmt.Println(now.Format("2006-01-02 15:04:05"))
	// 布局魔法：Go 不用 %Y%m%d 占位符，而是拿【参考时间】当模板：
	//   Mon Jan 2 15:04:05 MST 2006 → 写成 1月2日下午3点4分5秒2006年
	// 记忆口诀：1234567 = 月日时分秒年。对照 Python/C 的 strftime("%Y-%m-%d %H:%M:%S")
	fmt.Println(now.Format("2006/01/02 03PM"))

	later := now.Add(90 * time.Minute)  // 时间运算全靠 Duration 类型
	fmt.Println(later.After(now))       // 比较：After/Before/Equal
	d := later.Sub(now)                 // 差值也是 Duration
	fmt.Println(d.Hours(), d.Minutes()) // 1.5 小时、90 分钟

	t, _ := time.Parse("2006-01-02", "2026-08-24") // 字符串 → 时间
	fmt.Println(t.Weekday())

	// 注意：time.Now() 带本地时区；解析无时区的时间默认 UTC，
	// 跨时区业务建议全程 time.UTC 显式处理

	// ==== 4. math 与 math/rand ====
	fmt.Println(math.Sqrt(2), math.Abs(-3.5), math.Floor(2.7), math.Max(1, 2))
	fmt.Println(math.Pow(2, 10), math.Pi)

	fmt.Println(rand.Intn(100), rand.Float64()) // Go 1.20+ 自动随机播种，无需 seed
	r := rand.New(rand.NewSource(42))           // 固定种子 → 可复现序列，测试友好
	fmt.Println(r.Intn(100), r.Intn(100))

	// ==== 5. sort 包 ====
	nums := []int{3, 1, 4, 1, 5}
	sort.Ints(nums) // 就地升序；sort.Strings 同理
	strs := []string{"香蕉", "apple", "Apple"}
	sort.Strings(strs) // 按字节序排：大写<小写<中文（Unicode 码点顺序）
	fmt.Println(nums, strs)

	people := []Person{{"Bob", 30}, {"Alice", 25}, {"Carol", 35}}
	sort.Slice(people, func(i, j int) bool { // 自定义比较器：闭包直接写规则
		return people[i].Age < people[j].Age
	})
	fmt.Println(people)
	// Go 1.21+ 更推荐泛型的 slices 包：slices.SortFunc/slices.Contains 等（第14章）

	// ==== 6. 中文与 UTF-8 ====
	zh := "Go语言"
	fmt.Println(len(zh), utf8.RuneCountInString(zh)) // 8 个字节 vs 4 个字符！
	// Go 的 string 就是 UTF-8 字节序列：
	// len 按字节算（和 C 的 strlen 行为一致）；要数字符必须走 utf8/rune
	runes := []rune(zh) // 转 rune 切片才能按下标取"字符"
	fmt.Println(string(runes[2]), runes[2] == '语')
	// 反向遍历字节再解码可用 []byte(s)；日常处理文本优先 range（自动按 rune）

	/*
	 * ---- 一句话总结 ----
	 * 拼接找 Builder、时间背 2006-01-02、随机数免播种、
	 * 排序 sort.Slice 塞闭包、中文字符数认准 RuneCount——
	 * 标准库就是 Go 的"电池"，先查标准库再造轮子。
	 */
}
