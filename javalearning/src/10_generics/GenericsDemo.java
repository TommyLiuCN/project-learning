// ============================================================
// 第10章 · 泛型
// ============================================================
// 运行: java src/10_generics/GenericsDemo.java
// 学习要点:
//   1. 泛型类 Box<T>
//   2. 泛型方法
//   3. 有界类型参数 <T extends Comparable>
//   4. 通配符 ? extends / ? super（PECS）
//   5. 类型擦除——泛型只存在于编译期
// ============================================================

import java.util.ArrayList;
import java.util.List;

/**
 * 自定义泛型类：T 是类型占位符，用的时候才确定。
 * 对比 C++ 的 template<typename T> class Box —— 思想相同。
 * 对比 Python：不需要泛型也能写，但类型错误运行时才炸；
 *             Java 用泛型把错误提前到编译期。
 */
class Box<T> {
    private T content;

    public void put(T item)      { this.content = item; }
    public T get()               { return content; }   // 取出来就是 T 类型，无需强转
}

public class GenericsDemo {

    /**
     * 泛型方法：<E> 写在返回类型前，声明这是个泛型方法。
     * 调用时编译器自动推断 E，不用显式写。
     */
    static <E> void printAll(List<E> list) {
        for (E e : list) {
            System.out.print(e + " ");
        }
        System.out.println();
    }

    /**
     * 有界类型参数：限定 T 必须继承 Comparable → 才能保证有 compareTo 方法可调用。
     * 类似 C++ 概念上的 requires / Python 鸭子类型里"只要有 < 就行"的约束，
     * 但 Java 是在编译期强制检查。
     */
    static <T extends Comparable<T>> T maxOf(List<T> list) {
        T max = list.get(0);
        for (T t : list) {
            if (t.compareTo(max) > 0) {
                max = t;
            }
        }
        return max;
    }

    public static void main(String[] args) {
        // ==== 1. 泛型类使用 ====
        System.out.println("=== 泛型类 ===");
        Box<String> strBox = new Box<>();
        strBox.put("一箱字符串");
        String s = strBox.get();          // 不需要强转！如果用 Object 就得 (String) 强转
        System.out.println("strBox: " + s);

        Box<Integer> intBox = new Box<>();// 同一个类，装不同类型——模板复用的意义
        intBox.put(42);
        System.out.println("intBox: " + intBox.get());
        // intBox.put("字符串");           // 编译错误，类型不符

        // ==== 2. 泛型方法 ====
        System.out.println("\n=== 泛型方法 ===");
        List<String> langs = List.of("Java", "Python", "C");
        List<Integer> nums = List.of(3, 1, 4);
        printAll(langs);                  // E 自动推断为 String
        printAll(nums);                   // E 自动推断为 Integer

        // ==== 3. 有界类型 ====
        System.out.println("\n=== 有界类型 ===");
        System.out.println("最大整数: " + maxOf(nums));
        System.out.println("最大字符串: " + maxOf(langs)); // 字符串按字典序比较
        // maxOf(List.of(new Object()))   // 编译错误：Object 没有 compareTo

        // ==== 4. 通配符 ====
        System.out.println("\n=== 通配符 ===");
        // 问题: List<Integer> 不是 List<Number> 的子类！（即使 Integer 是 Number 的子类）
        List<Integer> ints = new ArrayList<>(List.of(1, 2, 3));
        // List<Number> numsRef = ints;   // 编译错误！泛型没有协变

        // 解决：通配符。? extends Number 表示"Number 或其任何子类"
        List<? extends Number> producer = ints;
        Number n = producer.get(0);       // 读没问题：读出来一定是 Number
        System.out.println("extends 通配符读: " + n);
        // producer.add(1);                // 编译错误！不能往里写（不知道具体是哪种列表）

        // ? super Integer 表示"Integer 或其父类"→ 可以安全地写入 Integer
        List<? super Integer> consumer = new ArrayList<Number>();
        consumer.add(42);                 // 写没问题
        System.out.println("super 通配符写后: " + consumer.get(0));
        // 记忆口诀 PECS: Producer Extends(往外读), Consumer Super(往里写)

        /*
         * ---- 5. 类型擦除 ----
         * Java 的泛型只在编译期做检查，编译后的字节码里 T 被替换成 Object（擦除）。
         * 所以运行时拿不到 T 的真实类型：
         *   new ArrayList<String>().getClass() == new ArrayList<Integer>().getClass()  // true!
         * 对比 C++：模板会在编译期为每种类型生成一份真实代码（实例化）。
         *
         * ---- 一句话总结 ----
         * 泛型 = 编译期的类型安全 + 免强转；
         * 常用姿势: 集合全带 <>、工具方法用 <T>、约束用 extends、读写分家想 PECS。
         */
        System.out.println("\n=== 类型擦除 ===");
        System.out.println("两个不同泛型的类是同一个? "
                + (new ArrayList<String>().getClass() == new ArrayList<Integer>().getClass()));
    }
}
