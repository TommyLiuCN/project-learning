# Go 学习项目

按章节组织的 Go 语言学习练习，从基础到进阶，每章独立可运行（`go run ./NN_xxx`）。注释中大量对比 C 和 Python，适合有 C/Python 基础的初学者。

## 项目结构

```
golearning/
├── go.mod                      # 模块定义（module golearning）
├── README.md
├── 01_hello/                   # 入口、导入包、fmt 输出、命令行参数
├── 02_vars_types/              # var/:=/const/iota、基本类型、显式转换、strconv
├── 03_control_flow/            # if 初始化语句、for 三形态、range、switch、label
├── 04_functions/               # 多返回值、可变参数、闭包、defer
├── 05_composite_types/         # 数组 vs 切片 vs map（重点切片共享底层陷阱）
├── 06_structs/                 # 结构体、零值可用、NewXxx 构造惯例、嵌入组合、标签
├── 07_pointers/                # 指针、值传递真相、new/&T{}/make、nil、逃逸与 GC
├── 08_methods_interfaces/      # 方法接收者、隐式接口、多态、类型断言、any、Stringer
├── 09_errors/                  # error 即值、自定义错误、errors.Is/As/%w、panic/recover
├── 10_goroutines/              # goroutine、WaitGroup、Mutex、-race 竞态检测
├── 11_channels/                # channel 收发/close/range、select、单向通道、worker pool
├── 12_stdlib/                  # strings/strconv/time/math/rand/sort/utf8
├── 13_file_io_json/            # os 快捷读写、bufio 流式处理、encoding/json
└── 14_generics/                # 类型参数、约束、comparable、泛型容器
```

每章一个独立的 `package main` 目录，互不依赖，可单独运行。

## 快速开始

### 前置条件

- **Go 1.22+**（本项目在 Go 1.26 下开发验证），安装：https://go.dev/dl/
- 任一终端（PowerShell / CMD / bash），或 VS Code（装 Go 扩展）/ GoLand

### 运行某一章

```powershell
# 在 golearning 目录下
go run ./01_hello          # 直接跑第1章
go run ./11_channels       # 并发章节同理

# 中文乱码时先执行 chcp 65001 把控制台切成 UTF-8
```

### 全量验证 / 工具链

```powershell
go build ./...             # 全量编译检查（不产出二进制）
go vet ./...               # 静态检查
gofmt -w .                 # 格式化（VS Code 保存即格式化）
go run -race ./10_goroutines  # 竞态检测器跑并发章节
```

## 学习路线建议

| 阶段 | 章节 | 说明 |
|------|------|------|
| **入门** | 01-04 | 语法基础：入口、变量类型、流程控制、函数与 defer |
| **核心** | 05-08 | 切片/map、结构体、指针、方法与接口——Go 类型系统的灵魂 |
| **进阶** | 09-11 | 错误处理惯例、goroutine、channel——写出地道 Go 代码的关键 |
| **高级** | 12-14 | 常用标准库、文件 IO 与 JSON、泛型 |

建议顺序学习：并发两章（10、11）依赖前面的 defer 和错误处理概念。

## 与 C/C++、Python 快速对照

| 概念 | C/C++ | Python | Go |
|------|-------|--------|----|
| 程序入口 | `int main()` | 脚本顺序执行 | `package main` + `func main()` |
| 动态数组 | `vector` | `list` | 切片 `[]T`（共享底层数组！） |
| 哈希表 | `unordered_map` | `dict` | `map[K]V`（遍历顺序随机） |
| 错误处理 | 返回码/errno | `try/except` | `(结果, error)` 双返回值 |
| 字符串 | `char*` 手动管理 | `str` 不可变 | `string` 不可变 UTF-8 字节序列 |
| 资源释放 | 手动 free / RAII | `with` 语句 | `defer` |
| 继承 | class 继承 | class 继承 | 无继承：嵌入组合 + 接口 |
| 并发 | pthread | threading/asyncio | goroutine + channel |
| 枚举 | enum | 无原生支持 | `const ... iota` |

## 练习建议

每个文件顶部都有「学习要点」注释。建议：

1. 先通读代码和注释（注释里有大量与 C/Python 的对照）
2. 自己动手修改、实验：故意越界访问切片、忘写 break 的 case、去掉 Mutex 后用 `-race` 观察数据竞争
3. 小练习：给 13 章写一个命令行待办工具（os.Args 解析命令 + JSON 文件持久化）；给 11 章加一个带超时的扇入(fan-in)合并
4. 遇到不理解的 API，查 [pkg.go.dev](https://pkg.go.dev/std)

## 推荐资源

- [A Tour of Go](https://tour.go-zh.org/) — 官方交互式教程（中文）
- [Go by Example](https://gobyexample.com/) — 按示例学语法，和本项目风格互补
- [Effective Go](https://go.dev/doc/effective_go) — 官方地道路线指南
- [pkg.go.dev](https://pkg.go.dev/std) — 标准库文档
- 《Go 程序设计语言》（The Go Programming Language）— 经典教材
- [Uber Go Style Guide（中文版）](https://github.com/uber-go/guide/blob/master/style.md) — 工程实践进阶
- [Go FAQ](https://go.dev/doc/faq) — 很多"为什么这样设计"的官方答案
