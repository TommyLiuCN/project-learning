// ============================================================
// 第4章 · 方法（函数）
// ============================================================
// 运行: java src/04_methods/MethodsDemo.java
// 学习要点:
//   1. 方法定义与调用
//   2. Java 只有值传递（对象传的是"引用的拷贝"）
//   3. 方法重载 overload（C/Python 都没有）
//   4. 可变参数（类似 Python 的 *args）
//   5. 递归
//   6. static 与实例方法的区别
// ============================================================

public class MethodsDemo {

    // ---- 1. 方法定义 ----
    // 格式: 修饰符 返回类型 方法名(参数列表) { ... }
    // 对比 C 函数: int add(int a, int b) —— 几乎一样，只是必须写在类里
    // 对比 Python def add(a, b): —— Java 必须声明类型、没有冒号缩进
    static int add(int a, int b) {
        return a + b;
    }

    // 没有返回值用 void（相当于 Python 隐式返回 None、C 的 void）
    static void greet(String name) {
        System.out.println("你好, " + name + "!");
    }

    // ---- 2. 值传递演示 ----
    static void changePrimitive(int x) {
        x = 999;              // 改的是拷贝，不影响外面
    }

    static void appendToList(java.util.List<String> list) {
        list.add("新元素");    // 有效！传进来的"引用的拷贝"仍指向同一个对象
    }

    static void reassignList(java.util.List<String> list) {
        list = new java.util.ArrayList<>(); // 无效！只改了局部拷贝指向，外面的引用不变
        list.add("看不到我");
    }

    // ---- 3. 方法重载：同名不同参 ----
    // C 不允许同名函数；Python 定义两个同名方法会直接覆盖。
    // Java 根据参数个数/类型自动选择——这叫重载 (overload)
    static double area(double side) {           // 正方形
        return side * side;
    }

    static double area(double width, double height) { // 矩形
        return width * height;
    }

    static double area(double a, double b, double c, double d) { // 梯形
        return (a + c) * b / 2;
    }
    // 注意：仅返回值类型不同不构成重载，编译器无法区分！

    // ---- 4. 可变参数 ----
    // int... nums 本质是 int[] 数组，类似 Python 的 *args
    static int sum(int... nums) {
        int total = 0;
        for (int n : nums) {
            total += n;
        }
        return total;
    }

    // ---- 5. 递归 ----
    // 和 C / Python 的递归写法完全一样，注意必须有终止条件
    static long factorial(int n) {
        if (n <= 1) return 1;          // 终止条件，忘了就 StackOverflowError（栈溢出）
        return n * factorial(n - 1);
    }

    public static void main(String[] args) {
        greet("小明");
        System.out.println("add(3,5) = " + add(3, 5));

        System.out.println("\n=== 值传递 ===");
        int num = 1;
        changePrimitive(num);
        System.out.println("基本类型传入后: " + num);            // 还是 1

        java.util.List<String> list = new java.util.ArrayList<>();
        list.add("初始");
        appendToList(list);
        System.out.println("往 list 里添加后: " + list);         // 有"新元素"

        reassignList(list);
        System.out.println("重新赋值后: " + list);               // 没有"看不到我"
        // 结论：Java 永远传值的拷贝；基本类型拷贝值，对象拷贝"遥控器"，
        //       用遥控器能操作原对象，但换新遥控器不影响外面。

        System.out.println("\n=== 重载 ===");
        System.out.println("area(3) = " + area(3));
        System.out.println("area(3,4) = " + area(3, 4));
        System.out.println("area(2,4,6,4) = " + area(2, 4, 6, 4));

        System.out.println("\n=== 可变参数 ===");
        System.out.println("sum() = " + sum());
        System.out.println("sum(1,2,3) = " + sum(1, 2, 3));
        System.out.println("sum(1,2,...,10) = " + sum(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));

        System.out.println("\n=== 递归 ===");
        System.out.println("5! = " + factorial(5));

        /*
         * ---- 关于 static ----
         * main 必须是 static：static 方法属于类本身，不需要 new 对象就能调用。
         * 实例方法属于对象，必须先 new（详见第6章）。
         * 类比 Python：static 方法 ≈ 模块级函数；实例方法 ≈ 带 self 的方法。
         */
    }
}
