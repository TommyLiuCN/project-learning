// ============================================================
// 第5章 · 数组与字符串
// ============================================================
// 运行: java src/05_arrays_strings/ArraysStrings.java
// 学习要点:
//   1. 数组的声明与初始化
//   2. 数组长度固定、自动检查越界（对比 C）
//   3. 二维数组（支持锯齿形）
//   4. Arrays 工具类
//   5. String 不可变 + 常用方法
//   6. StringBuilder 高效拼接
// ============================================================

import java.util.Arrays;

public class ArraysStrings {

    public static void main(String[] args) {
        // ==== 1. 数组的三种初始化 ====
        int[] a = {1, 2, 3};                 // 静态初始化：直接给值，类似 C 的 int a[] = {...}
        int[] b = new int[5];                // 动态初始化：长度5，元素默认为0（int 默认值）
        int[] c = new int[]{4, 5, 6};        // new 的同时给值
        // 对比 Python: a = [1,2,3] —— 但 Java 数组长度固定，不能 append！

        System.out.println("a[0] = " + a[0] + ", b 长度 = " + b.length);
        // 注意是 length 属性（无括号），String 是 length() 方法，别搞混

        // ==== 2. 越界检查 ====
        try {
            int x = a[10];                   // C 里这是未定义行为(段错误)，Java 会抛异常
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("捕获越界: " + e.getMessage());
        }
        // 好处：越界立刻报错；坏处：有一点点运行时开销

        // ==== 3. 遍历 ====
        for (int i = 0; i < a.length; i++) {     // 传统 for，需要下标时用
            System.out.print(a[i] + " ");
        }
        System.out.println();

        for (int n : a) {                        // 增强 for，类似 Python 的 for n in a:
            System.out.print(n + " ");
        }
        System.out.println();

        // Arrays.toString 一行打印整个数组——Python 直接 print(a) 就行，
        // Java 数组直接打印会得到 [I@哈希码 这种鬼东西，必须用工具类！
        System.out.println("Arrays.toString: " + Arrays.toString(a));

        // ==== 4. 二维数组 ====
        int[][] matrix = {
            {1, 2, 3},
            {4, 5, 6}
        };
        System.out.println("\n二维数组: " + Arrays.deepToString(matrix));

        // 锯齿数组：每行长度可以不同（C 也能做但要手动管理指针数组，Java 天生支持）
        int[][] jagged = new int[3][];
        jagged[0] = new int[]{1};
        jagged[1] = new int[]{1, 2};
        jagged[2] = new int[]{1, 2, 3};
        System.out.println("锯齿数组: " + Arrays.deepToString(jagged));

        // ==== 5. Arrays 工具类 ====
        int[] data = {5, 2, 8, 1, 9};
        Arrays.sort(data);                              // 原地排序（双轴快排），类似 Python list.sort()
        System.out.println("\n排序后: " + Arrays.toString(data));

        int idx = Arrays.binarySearch(data, 8);         // 二分查找（必须先排序）
        System.out.println("8 的下标: " + idx);

        int[] copy = Arrays.copyOf(data, 7);            // 复制并扩容到7位（多出的补0）
        System.out.println("扩容复制: " + Arrays.toString(copy));
        System.out.println("数组相等? " + Arrays.equals(data, copy)); // 比较内容，== 只比地址

        // ==== 6. String 不可变 ====
        String s = "hello";
        s.toUpperCase();                                // 白调用！返回新串但没人接住
        System.out.println("\n白调用后: " + s);          // 还是 hello
        s = s.toUpperCase();                            // 正确姿势：把新串重新赋值
        System.out.println("重新赋值后: " + s);
        // 和 Python 的 str 一样不可变；和 C 的 char* 不同（可原地修改但危险）

        String t = new String("hello");
        System.out.println("s == t ? " + (s == t));           // false！== 比较内存地址（像 C 比指针）
        System.out.println("s.equals(t) ? " + s.equals(t));   // true，内容比较必须用 equals

        // ---- 常用方法速查 ----
        String str = "Java,Python,C";
        System.out.println("\nlength: " + str.length());              // 字符数
        System.out.println("charAt(0): " + str.charAt(0));            // 取字符，类似 Python str[0]
        System.out.println("substring: " + str.substring(5, 11));     // 切片，含头不含尾，类似 Python [5:11]
        System.out.println("indexOf: " + str.indexOf("Python"));      // 查找子串位置，找不到返回 -1
        System.out.println("split: " + Arrays.toString(str.split(","))); // 按逗号切开成数组，类似 Python split
        System.out.println("replace: " + str.replace("Java", "Kotlin"));
        System.out.println("contains: " + str.contains("Py"));        // 类似 Python 的 in
        System.out.println("trim: [" + "  hi  ".trim() + "]");        // 去首尾空白

        // ==== 7. StringBuilder：循环拼接必用 ====
        // 循环里用 + 拼 String：每次都创建新对象，性能差（Python 同理，建议 join）；
        // StringBuilder 在内部数组上原地追加，效率高得多。
        StringBuilder sb = new StringBuilder();
        for (int i = 1; i <= 5; i++) {
            sb.append(i).append(",");    // 支持链式调用
        }
        sb.setLength(sb.length() - 1);   // 去掉末尾多余的逗号
        System.out.println("\nStringBuilder 结果: " + sb);

        /*
         * ---- 一句话总结 ----
         * 数组定长且自动防越界；打印数组要用 Arrays.toString；
         * String 不可变、比较用 equals；循环拼接换 StringBuilder。
         */
    }
}
