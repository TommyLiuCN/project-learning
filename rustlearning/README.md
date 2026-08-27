# Rust 学习项目

按章节组织的 Rust 学习练习，从基础到进阶，每章一个独立可运行的二进制。注释中大量对比 C 和 Python，适合有 C/Python 基础的初学者。

## 项目结构

```
rustlearning/
├── Cargo.toml                          # 项目清单（依赖、edition 等）
├── README.md
└── src/
    └── bin/                            # 每章一个二进制目标
        ├── 01_hello.rs                 # Hello World、println! 宏、格式化输出
        ├── 02_variables.rs             # 不可变默认、遮蔽、const、基本类型、类型转换
        ├── 03_control_flow.rs          # if 表达式、loop 带返回值、for 区间、循环标签
        ├── 04_functions.rs             # 函数、语句 vs 表达式、文档注释、递归
        ├── 05_ownership.rs             # 所有权与借用（核心）：move/clone/&/&mut/切片
        ├── 06_structs_enums.rs         # 结构体三形态、impl 方法、枚举携带数据、Option
        ├── 07_pattern_matching.rs      # match 穷尽性、守卫、解构、if let / let else
        ├── 08_error_handling.rs        # panic! vs Result、? 运算符、自定义错误
        ├── 09_generics_traits_lifetimes.rs # 泛型单态化、trait 接口、生命周期标注
        ├── 10_collections.rs           # Vec/String/HashMap/HashSet/BTreeMap
        ├── 11_iterators_closures.rs    # 闭包三档、map/filter 管道、手写 Iterator
        ├── 12_smart_pointers.rs        # Box/Deref/Drop/Rc/RefCell
        ├── 13_modules_testing.rs       # mod/pub/use、单元测试、文档测试
        └── 14_concurrency.rs           # 线程、mpsc 通道、Arc<Mutex>、scoped threads
```

## 快速开始

### 前置条件

- **Rust 1.75+**（推荐最新稳定版；本项目在 rustc 1.97 下开发验证）
- 安装方式：<https://rustup.rs>（一条命令装好 rustc + cargo）

### 构建并运行某一章

```bash
# 在 rustlearning 目录下
cargo run --bin 01_hello      # 编译并运行第 1 章
cargo run --bin 05_ownership  # 所有权章节

# 只编译不运行（检查全部代码）
cargo build

# 查看所有可用二进制
cargo run --bin <按 Tab 补全>   # 或直接看 src/bin/ 目录
```

### 运行测试与文档

```bash
cargo test                    # 第13章演示了内置测试，含文档测试
cargo doc --open              # 生成并打开 API 文档（/// 注释的用武之地）
```

> Windows 终端中文乱码时先执行 `chcp 65001` 切换 UTF-8。

## 学习路线建议

| 阶段 | 章节 | 说明 |
|------|------|------|
| **入门** | 01-04 | 语法基础：变量、控制流、函数——手感接近 C |
| **核心** | 05-07 | 所有权/借用、结构体枚举、模式匹配——Rust 的灵魂，慢点学 |
| **进阶** | 08-12 | 错误处理、泛型/trait/生命周期、集合、迭代器、智能指针 |
| **高级** | 13-14 | 模块化组织代码、内置测试、无畏并发 |

**重点提示**：第 5 章「所有权」是 Rust 与 C/Python 差异最大的地方，
也是后续一切的基础。卡住是正常的，建议配合 [Rust Book 第4章](https://kaisery.github.io/trpl-zh-cn/ch04-00-understanding-ownership.html) 反复咀嚼。

## 与 C/C++、Python 快速对照

| 概念 | C/C++ | Python | Rust |
|------|-------|--------|------|
| 内存管理 | 手动 malloc/free | GC 引用计数 | **所有权系统**，编译期自动释放 |
| 空值 | NULL（悬垂=UB） | None | `Option<T>`，编译器强制处理 |
| 异常/错误码 | 错误码或异常 | try/except | `Result<T, E>` + `?` 传播 |
| 接口 | 虚函数/纯虚类 | ABC/Protocol | `trait`（可默认实现+运算符重载） |
| 泛型 | template | 无（鸭子类型） | 泛型 + trait 约束，编译期单态化 |
| 动态数组 | `std::vector` | `list` | `Vec<T>` |
| 哈希表 | `unordered_map` | `dict` | `HashMap<K, V>` |
| 字符串 | `char*` 手动管理 | `str` 不可变 | `String` 可变 / `&str` 切片 |
| 多线程 | pthread（易竞态） | threading + GIL | 线程无 GIL，数据竞争编译期拒绝 |

## 练习建议

每个文件顶部都有「学习要点」注释，正文按编号小节展开。建议：

1. 先通读代码和注释（注释里有大量与 C/Python 的对照）
2. 自己动手修改、实验——很多注释掉的行就是「故意留的编译错误」，
   取消注释跑一次 `cargo check`，读报错信息本身就是最好的练习
3. 试着在每个文件末尾加自己的小节
4. 遇到看不懂的编译错误，先读懂 E0xxx 错误码附带的建议再搜索

## 推荐资源

- [The Rust Programming Language（中文版）](https://kaisery.github.io/trpl-zh-cn/) — 官方教程"the book"，最系统的入门
- [Rust 语言圣经](https://course.rs/) — 中文社区最好的系统性教材，覆盖面广例子多
- [Rust by Example（中文版）](https://rustwiki.org/zh-CN/rust-by-example/) — 通过例子学习，适合当速查手册
- [Rustlings](https://github.com/rust-lang/rustlings) — 官方互动练习题，配合本项目食用效果更佳
- [std 文档](https://doc.rust-lang.org/std/) — 标准库参考
- [Compiler Explorer](https://godbolt.org/) — 看 Rust 生成的汇编，理解「零成本抽象」
