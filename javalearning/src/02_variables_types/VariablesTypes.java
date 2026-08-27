// ============================================================
// 第2章 · 变量与数据类型
// ============================================================
// 运行: java src/02_variables_types/VariablesTypes.java
// 学习要点:
//   1. 八大基本类型（位数固定，跨平台一致）
//   2. boolean 不能用 0/1 代替（对比 C）
//   3. final 常量
//   4. 自动类型转换 vs 强制类型转换
//   5. var 局部变量类型推断
//   6. 包装类与自动装箱/拆箱
// ============================================================

public class VariablesTypes {

    public static void main(String[] args) {
        // ==== 1. 整数类型 ====
        // 和 C 的最大区别：Java 类型位数是固定的，不随平台变化
        // （C 的 long 在 Windows 是 32 位、Linux 是 64 位；Java 永远 64 位）
        byte tiny = 127;            // 8 位，类似 C 的 int8_t
        short small = 32767;        // 16 位，类似 C 的 int16_t
        int normal = 2100000000;    // 32 位，最常用，类似 C 的 int32_t
        long big = 9_000_000_000L;  // 64 位，必须加 L 后缀！下划线只是分隔符增强可读性

        System.out.println("byte: " + tiny + ", short: " + small);
        System.out.println("int: " + normal + ", long: " + big);

        // 整数除法直接舍弃小数（和 C 一样，和 Python 的 / 不同）
        System.out.println("7 / 2 = " + (7 / 2));      // 3，不是 3.5
        System.out.println("7 % 2 = " + (7 % 2));      // 1 取余
        // 注意：Python 里 7 / 2 得 3.5，Java 想要小数得转 double 或写 7.0 / 2

        // 坑：两个大 int 相乘可能溢出且不报错（和 C 相同的静默溢出）
        int overflow = 2000000000 + 2000000000;
        System.out.println("int 溢出示例: " + overflow); // 变成负数！

        // ==== 2. 浮点类型 ====
        double d = 3.14;          // 64 位，默认的小数就是 double
        float f = 3.14f;          // 32 位，必须加 f 后缀，否则编译错误
        System.out.println("double: " + d + ", float: " + f);

        // 经典坑：浮点数不能精确表示（二进制问题），和 C 一模一样
        System.out.println("0.1 + 0.2 = " + (0.1 + 0.2)); // 0.30000000000000004
        // 金额计算要用 BigDecimal 类，不能用 double！

        // ==== 3. char 与 boolean ====
        char c = 'A';       // 单引号，16 位 Unicode 字符（C 的 char 只有 8 位 ASCII）
        char chinese = '中'; // Java 的 char 可以直接放汉字
        System.out.println("char: " + c + " " + chinese);

        boolean flag = true; // 只有 true/false 两个字面量
        // 坑：if(1) 在 C 能编译，在 Java 直接编译错误！条件必须是 boolean
        if (flag) {
            System.out.println("boolean 只能是 true/false，不能当数字用");
        }

        // ==== 4. final 常量 ====
        // 对比 C 的 const 和 #define；对比 Python 没有真常量（只有全大写的约定）
        final double PI = 3.14159;
        final int MAX_USERS = 100;
        System.out.println("PI = " + PI + ", MAX_USERS = " + MAX_USERS);
        // PI = 3.15; // 编译错误：final 变量不能重新赋值

        // ==== 5. 类型转换 ====
        // 自动转换（小 → 大）：byte < short < int < long < float < double
        int i = 100;
        long autoLong = i;       // 自动，无风险
        double autoDouble = i;   // 自动
        System.out.println("自动转换: " + autoLong + ", " + autoDouble);

        // 强制转换（大 → 小）：必须显式写，可能丢失精度（类比 C 的强转）
        double pi = 3.99;
        int truncated = (int) pi;   // 直接截断小数，不是四舍五入！
        System.out.println("(int)3.99 = " + truncated);

        // 字符串 ↔ 数字（Python 用 int()/str()，Java 用包装类的方法）
        int parsed = Integer.parseInt("123");     // 字符串 → int
        String str = String.valueOf(456);         // int → 字符串
        System.out.println("解析: " + (parsed + 1) + ", 转字符串: " + str);

        // ==== 6. var 局部变量类型推断（Java 10+）====
        // 编译器自动推断类型——像 C++ 的 auto。
        // 注意：仍是静态类型！var s = "hi"; 之后 s 不能再赋值为数字（这点和 Python 完全不同）
        var message = "类型由右边推断出来是 String";
        var count = 42; // 推断为 int
        System.out.println(message + ", count 类型是 int: " + count);
        // var 只能用于局部变量，不能用于成员变量和方法参数

        // ==== 7. 包装类与自动装箱 ====
        // 八大基本类型各有一个对象版本：int→Integer, double→Double...
        // 对比 Python：一切皆对象；Java 区分"快的值"和"有方法的对象"
        Integer boxed = 100;    // 自动装箱：int → Integer（编译器帮你调 Integer.valueOf）
        int unboxed = boxed + 1; // 自动拆箱：Integer → int
        System.out.println("装箱拆箱: " + boxed + " -> " + unboxed);

        // 包装类能调用方法、放进集合（第9章集合只收对象，不收基本类型）
        System.out.println("Integer 最大值: " + Integer.MAX_VALUE);
        System.out.println("二进制形式: " + Integer.toBinaryString(10));

        /*
         * ---- 一句话总结 ----
         * 类型系统几乎照搬 C 但位数固定；boolean 是独立类型；
         * 小转大自动、大转小强转；var 只是语法糖，Java 永远是静态类型。
         */
    }
}
