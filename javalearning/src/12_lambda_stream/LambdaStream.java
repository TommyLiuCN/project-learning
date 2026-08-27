// ============================================================
// 第12章 · Lambda 与 Stream
// ============================================================
// 运行: java src/12_lambda_stream/LambdaStream.java
// 学习要点:
//   1. 匿名内部类 → Lambda 的演变
//   2. 四大函数式接口：Predicate / Function / Consumer / Supplier
//   3. 方法引用 ::
//   4. Stream: filter/map/sorted/reduce（对标 Python 列表推导式）
//   5. collect 收集结果
//   6. Optional 防 null
// ============================================================

import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.Optional;
import java.util.function.Consumer;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Supplier;
import java.util.stream.Collectors;

/** 演示用的员工记录类 */
record Employee(String name, String city, double salary) {
    @Override
    public String toString() { return name + "(" + city + "," + salary + ")"; }
}

public class LambdaStream {

    public static void main(String[] args) {
        List<Employee> staff = List.of(
                new Employee("张三", "北京", 12000),
                new Employee("李四", "上海", 15000),
                new Employee("王五", "北京", 9000),
                new Employee("赵六", "深圳", 20000)
        );

        // ==== 1. Lambda 从哪来 ====
        System.out.println("=== Lambda 演变 ===");
        // Java 没有"独立函数"，函数必须依附于接口——以前要写冗长的匿名内部类：
        Runnable oldWay = new Runnable() {
            @Override
            public void run() { System.out.println("[匿名内部类] 你好"); }
        };
        // Lambda 本质：只有一个抽象方法的接口(函数式接口)的简写。类似 Python 的 lambda：
        Runnable newWay = () -> System.out.println("[Lambda] 你好");
        oldWay.run();
        newWay.run();

        // 带参数的例子：(a, b) -> a + b 对比 Python: lambda a, b: a + b
        Comparator<Employee> bySalary = (a, b) -> Double.compare(a.salary(), b.salary());
        List<Employee> mutableStaff = new ArrayList<>(staff); // List.of 是不可变的，排序前先拷贝
        mutableStaff.sort(bySalary);          // List.sort 直接用
        System.out.println("按工资排序: " + mutableStaff);

        // ==== 2. 四大函数式接口 ====
        System.out.println("\n=== 函数式接口 ===");
        Predicate<String> isEmpty = s -> s.isEmpty();       // 断言：T → boolean
        Function<Integer, Integer> square = x -> x * x;     // 函数：T → R
        Consumer<String> printer = s -> System.out.println("消费: " + s); // 消费者：T → void
        Supplier<Double> random = () -> Math.random();      // 生产者：() → T

        System.out.println("\"\" 是空串? " + isEmpty.test(""));
        System.out.println("5² = " + square.apply(5));
        printer.accept("我是被消费的字符串");

        // ==== 3. 方法引用 :: ====
        System.out.println("\n=== 方法引用 ===");
        // Lambda 只是转发调用时，可以进一步简写成方法引用
        Consumer<String> out = System.out::println;         // 等价 s -> System.out.println(s)
        out.accept("方法引用更简洁");
        Function<String, Integer> len = String::length;     // 等价 s -> s.length()
        System.out.println("Java 的长度 = " + len.apply("Java"));
        // 类比 Python: 传函数名本身就是引用 print；Java 需要 :: 才能得到"函数值"

        // ==== 4. Stream：数据处理流水线 ====
        System.out.println("\n=== Stream ===");
        // 需求：找出北京员工的姓名，按工资降序
        // Python 写法: sorted([e.name for e in staff if e.city=="北京"], reverse=True)
        List<String> beijingNames = staff.stream()          // 1. 得到流
                .filter(e -> e.city().equals("北京"))        // 2. 过滤（中间操作，惰性）
                .sorted(Comparator.comparingDouble(Employee::salary).reversed())
                .map(Employee::name)                         // 3. 转换：Employee → String
                .collect(Collectors.toList());               // 4. 终结操作，收集成 List
        System.out.println("北京的员工: " + beijingNames);

        // 数值流统计
        double total = staff.stream().mapToDouble(Employee::salary).sum();
        Optional<Employee> top = staff.stream()
                .max(Comparator.comparingDouble(Employee::salary));
        System.out.printf("工资总和: %.0f, 最高: %s%n", total, top.orElseThrow());

        // reduce 归约：把所有元素折叠成一个值，类似 Python 的 functools.reduce
        int sumOfLen = staff.stream()
                .map(e -> e.name())
                .reduce(0, (acc, name) -> acc + name.length(), Integer::sum);
        System.out.println("所有名字总字数: " + sumOfLen);

        // ==== 5. collect 进阶：分组与拼接 ====
        System.out.println("\n=== collect 分组 ===");
        var byCity = staff.stream()
                .collect(Collectors.groupingBy(Employee::city));  // 类似 SQL 的 GROUP BY
        System.out.println("按城市分组: " + byCity);

        String names = staff.stream()
                .map(Employee::name)
                .collect(Collectors.joining(", ", "[", "]"));     // 拼接字符串
        System.out.println("全部员工: " + names);
        // 对比 Python: ", ".join(names)

        // ==== 6. Optional：显式的"可能没有" ====
        System.out.println("\n=== Optional ===");
        Optional<String> found = staff.stream()
                .map(Employee::name)
                .filter(n -> n.startsWith("张"))
                .findFirst();
        System.out.println("找到: " + found.orElse("没这个人"));  // 有值取值，没值给默认
        // Optional 把 null 检查变成类型问题——编译器逼你考虑"空"的情况，
        // 对比 Python 返回 None 后到处 if x is not None。

        /*
         * ---- 一句话总结 ----
         * Lambda 是函数式接口的简写；Stream 三板斧 filter/map/collect；
         * 心里想着 Python 推导式 [f(x) for x in xs if cond(x)] 就能写出等价的 Java 流。
         */
    }
}
