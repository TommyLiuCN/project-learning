# Java 学习项目

按章节组织的 Java 学习练习，从基础到进阶，每章独立可运行。注释中大量对比 C 和 Python，适合有 C/Python 基础的初学者。

## 项目结构

```
src/
├── 01_hello/                      # Hello World、输出、Scanner 输入
├── 02_variables_types/            # 八大基本类型、final、var、类型转换
├── 03_control_flow/               # if/switch(含箭头表达式)/循环
├── 04_methods/                    # 方法、重载、可变参数、递归、static
├── 05_arrays_strings/             # 数组、Arrays 工具类、String/StringBuilder
├── 06_oop_basics/                 # 类与对象、构造方法、封装、record
├── 07_inheritance_polymorphism/   # 继承、super、重写、多态、instanceof
├── 08_abstract_interfaces/        # 抽象类、接口、default 方法
├── 09_collections/                # List/Set/Map、遍历方式、Collections 工具类
├── 10_generics/                   # 泛型类、泛型方法、通配符、类型擦除
├── 11_exceptions/                 # 异常体系、checked/unchecked、自定义异常
├── 12_lambda_stream/              # Lambda、函数式接口、Stream API、Optional
├── 13_file_io/                    # Files/NIO 读写、Path 操作
└── 14_concurrency/                # 线程、synchronized、线程池、原子类
```

## 快速开始

### 前置条件

- **JDK 17+**（推荐 JDK 21+，本项目在 JDK 25 下开发验证）
- 任一终端（PowerShell / CMD），或 IntelliJ IDEA

### 运行某一章（单文件模式，最简单）

Java 11+ 支持直接运行 `.java` 源文件，无需手动编译：

```powershell
# 在 javalearning 目录下
java src/01_hello/HelloWorld.java

# 或使用附带脚本（自动处理中文乱码）
.\run.ps1 src\01_hello\HelloWorld.java
```

> 中文乱码时先执行 `chcp 65001` 把控制台切成 UTF-8。

### 传统两步（编译 + 运行）

```bash
javac -encoding UTF-8 -d out src/01_hello/HelloWorld.java
java -cp out HelloWorld
```

### IDE 方式

用 IntelliJ IDEA 直接打开 `javalearning` 文件夹，点每个文件 main 方法旁的绿色三角即可运行。

## 学习路线建议

| 阶段 | 章节 | 说明 |
|------|------|------|
| **入门** | 01-05 | 语法基础：变量、流程控制、方法、数组字符串 |
| **核心** | 06-08 | 类、继承、多态、接口——Java 面向对象的灵魂 |
| **进阶** | 09-12 | 集合、泛型、异常、Lambda/Stream——日常开发主力 |
| **高级** | 13-14 | 文件 IO 与多线程并发 |

## 与 C/C++、Python 快速对照

| 概念 | C/C++ | Python | Java |
|------|-------|--------|------|
| 程序入口 | `int main()` | 脚本顺序执行 | `public static void main(String[] args)` |
| 动态数组 | `std::vector` | `list` | `ArrayList<T>` |
| 哈希表 | `unordered_map` | `dict` | `HashMap<K,V>` |
| 异常 | 很少用，靠返回码 | `try/except` | `try/catch`（还有强制声明的 checked 异常） |
| 字符串 | `char*` 手动管理 | `str` 不可变 | `String` 不可变 |

## 练习建议

每个文件顶部都有「学习要点」注释。建议：
1. 先通读代码和注释（注释里有大量与 C/Python 的对照）
2. 自己动手修改、实验，故意制造报错看异常信息
3. 试着在每个文件末尾增加自己的练习代码
4. 遇到不理解的 API，查 [Oracle 官方文档](https://docs.oracle.com/en/java/javase/21/docs/api/index.html)

## 推荐资源

- [Oracle Java Tutorials](https://docs.oracle.com/javase/tutorial/) — 官方教程
- [Baeldung](https://www.baeldung.com/) — 大量实战向文章
- 《Java 核心技术》（Core Java）— 经典教材
- 《Effective Java》— 进阶必读的最佳实践
