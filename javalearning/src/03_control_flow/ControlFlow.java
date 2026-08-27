// ============================================================
// 第3章 · 流程控制
// ============================================================
// 运行: java src/03_control_flow/ControlFlow.java
// 学习要点:
//   1. if/else（条件必须是 boolean，对比 C）
//   2. switch 传统写法 vs 箭头表达式
//   3. while / do-while
//   4. for 与增强 for
//   5. break/continue 与标签
// ============================================================

public class ControlFlow {

    public static void main(String[] args) {
        // ==== 1. if / else if / else ====
        // 语法和 C 完全一样，但条件必须是 boolean 类型
        int score = 85;
        if (score >= 90) {
            System.out.println("优秀");
        } else if (score >= 60) {
            System.out.println("及格");
        } else {
            System.out.println("不及格");
        }
        // int x = 1;
        // if (x) {}      // C 能编译，Java 编译错误！必须写 if (x != 0)

        // 三元运算符也和 C 一样：条件 ? 值1 : 值2
        String level = score >= 60 ? "及格" : "不及格";
        System.out.println("三元运算符: " + level);

        // ==== 2. switch 传统写法 ====
        int day = 6;
        switch (day) {
            case 1:
                System.out.println("周一");
                break; // 忘了 break 会"穿透"到下一个 case！经典坑（C 同款）
            case 6:
            case 7:
                System.out.println("周末");
                break;
            default:
                System.out.println("工作日");
                break;
        }

        // ==== 3. switch 箭头表达式（Java 14+）====
        // 不用写 break、不会穿透，还能直接返回值——有点像 Python 的 match-case
        String dayName = switch (day) {
            case 1 -> "周一";
            case 2, 3, 4, 5 -> "工作日"; // 多值合并用逗号
            case 6, 7 -> "周末";
            default -> "未知";
        };
        System.out.println("switch 表达式: " + dayName);

        // ==== 4. while 和 do-while ====
        int count = 0;
        while (count < 3) {           // 先判断后执行，可能一次都不执行
            System.out.print(count + " ");
            count++;
        }
        System.out.println();

        count = 10;
        do {                           // 先执行后判断，至少执行一次（C 同款）
            System.out.println("do-while 至少跑一次, count = " + count);
            count++;
        } while (count < 5);

        // ==== 5. for 循环 ====
        for (int i = 0; i < 5; i++) {  // 初始化; 条件; 更新，和 C 一模一样
            System.out.print(i + " ");
        }
        System.out.println();

        // 增强 for（for-each）：遍历数组/集合，类似 Python 的 for x in items:
        int[] nums = {10, 20, 30};
        for (int n : nums) {           // 读作 "for each int n in nums"
            System.out.print(n + " ");
        }
        System.out.println("\n");
        // 局限：增强 for 拿不到下标、遍历中不能增删元素，需要下标就用普通 for

        // ==== 6. break / continue ====
        for (int i = 0; i < 10; i++) {
            if (i == 3) continue;  // 跳过本次循环（Python 的 continue 相同）
            if (i == 6) break;     // 跳出整个循环（Python 的 break 相同）
            System.out.print(i + " ");
        }
        System.out.println();

        // 标签：跳出到外层循环（Java 特色，类似受限版 goto；Python 没有这机制）
        outer:                       // 标签名 + 冒号
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (i * j == 2) break outer; // 直接跳出两层
                System.out.print("(" + i + "," + j + ") ");
            }
        }
        System.out.println();

        // ==== 7. 综合练习：九九乘法表 ====
        System.out.println("=== 九九乘法表 ===");
        for (int i = 1; i <= 9; i++) {
            for (int j = 1; j <= i; j++) {
                System.out.printf("%d×%d=%-4d", j, i, i * j); // %-4d 左对齐占4位，printf 风格同 C
            }
            System.out.println();
        }

        /*
         * ---- 一句话总结 ----
         * 流程控制语法基本照搬 C；新增的 switch 箭头表达式更安全；
         * 记住两个 Java 特点：条件必须是 boolean、增强 for 不能拿下标。
         */
    }
}
