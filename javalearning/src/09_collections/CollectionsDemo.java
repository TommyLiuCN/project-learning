// ============================================================
// 第9章 · 集合框架
// ============================================================
// 运行: java src/09_collections/CollectionsDemo.java
// 学习要点:
//   1. List（ArrayList）—— 对标 Python list / C++ vector
//   2. 泛型 <String> 的意义
//   3. Set（HashSet）—— 去重
//   4. Map（HashMap）—— 对标 Python dict
//   5. 三种遍历方式
//   6. Collections 工具类与选型建议
// ============================================================

import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class CollectionsDemo {

    public static void main(String[] args) {
        // ==== 1. ArrayList：最常用的动态数组 ====
        // 数组定长不实用，实际开发 90% 用集合。对标 C++ 的 std::vector、Python 的 list
        List<String> fruits = new ArrayList<>(); // 右边 <> 里类型可省略（钻石语法）
        fruits.add("苹果");                       // 没有 append，方法名叫 add
        fruits.add("香蕉");
        fruits.add("橙子");
        System.out.println(fruits);              // List 直接打印是友好的 [苹果, 香蕉, 橙子]
                                                 // （数组打印才是乱码，见第5章）

        fruits.set(1, "葡萄");                    // 替换，类似 Python fruits[1] = "葡萄"
        System.out.println("get(0): " + fruits.get(0));       // 取值用 get(i)，不能用 []
        System.out.println("size: " + fruits.size());         // 是 size() 不是 length
        System.out.println("contains: " + fruits.contains("橙子")); // 类似 Python 的 in
        System.out.println("remove: " + fruits.remove("橙子"));     // 按对象删，返回是否成功

        // ==== 2. 泛型的意义 ====
        List<String> safe = new ArrayList<>();
        safe.add("只能是字符串");
        // safe.add(123);   // 编译错误！泛型在编译期就把类型锁死
        // 不用泛型的老写法 add(Object) 什么都能塞，
        // 但取出来要强转，转错就 ClassCastException——运行时才炸，很危险。
        // 类比：Python 列表随便放，但类型错误也要到运行时才暴露；Java 提前到编译期。

        // ==== 3. HashSet：自动去重 ====
        System.out.println("\n=== Set ===");
        Set<Integer> unique = new HashSet<>();
        unique.add(1);
        unique.add(2);
        unique.add(2);
        unique.add(3);
        System.out.println("重复 add(2) 后仍是: " + unique + " (无序且去重)");
        // 对标 Python 的 set()；遍历顺序不保证——需要有序可用 LinkedHashSet/TreeSet

        List<String> dup = List.of("a", "b", "a", "c", "b"); // List.of 创建不可变列表（Java 9+）
        Set<String> dedup = new HashSet<>(dup);              // 一行去重，经典技巧
        System.out.println(dup + " 去重后: " + dedup);

        // ==== 4. HashMap：键值对 ====
        System.out.println("\n=== Map ===");
        Map<String, Integer> ages = new HashMap<>();
        ages.put("张三", 18);                     // 新增/覆盖都是 put（没有 [] 赋值语法糖）
        ages.put("李四", 22);
        System.out.println("张三的年龄: " + ages.get("张三"));
        System.out.println("get 不存在的键: " + ages.get("王五"));      // null！Python 会 KeyError，Java 返回 null
        System.out.println("getOrDefault: " + ages.getOrDefault("王五", 0)); // 安全取值，类似 dict.get(k, 0)
        ages.putIfAbsent("张三", 99);             // 已存在则不动
        System.out.println("putIfAbsent 后张三仍为: " + ages.get("张三"));

        // 经典计数模式：统计词频
        String[] words = {"java", "python", "java", "go", "java"};
        Map<String, Integer> freq = new HashMap<>();
        for (String w : words) {
            freq.merge(w, 1, Integer::sum);      // 不存在放1，存在则累加——一行搞定
            // 传统写法: freq.put(w, freq.getOrDefault(w, 0) + 1);
        }
        System.out.println("词频统计: " + freq);

        // ==== 5. 三种遍历方式 ====
        System.out.println("\n=== 遍历 Map ===");
        for (Map.Entry<String, Integer> e : ages.entrySet()) {  // 传统 entrySet
            System.out.println(e.getKey() + " = " + e.getValue());
        }
        ages.forEach((k, v) -> System.out.println("[forEach] " + k + " -> " + v)); // Lambda（第12章细讲）

        // ==== 6. Collections 工具类 ====
        System.out.println("\n=== 工具类 ===");
        List<Integer> nums = new ArrayList<>(List.of(5, 1, 9, 3));
        Collections.sort(nums);                  // 排序
        System.out.println("排序: " + nums);
        Collections.reverse(nums);               // 反转
        System.out.println("反转: " + nums);
        System.out.println("最大值: " + Collections.max(nums));

        /*
         * ---- 选型速查 ----
         * 要下标、随机访问多   → ArrayList（底层就是数组，和 C++ vector 一样扩容）
         * 头尾插删频繁        → ArrayDeque / LinkedList
         * 去重                → HashSet；要去重且保序 → LinkedHashSet；要排序 → TreeSet
         * 键值映射            → HashMap；要按 key 有序 → TreeMap
         *
         * ---- 一句话总结 ----
         * ArrayList ≈ 可增长的数组，HashSet 自动去重，HashMap 就是字典；
         * 泛型 <> 把类型错误从运行期提前到了编译期。
         */
    }
}
