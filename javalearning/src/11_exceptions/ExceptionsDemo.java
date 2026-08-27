// ============================================================
// 第11章 · 异常处理
// ============================================================
// 运行: java src/11_exceptions/ExceptionsDemo.java
// 学习要点:
//   1. 异常继承体系：Throwable → Error / Exception / RuntimeException
//   2. try-catch-finally
//   3. checked vs unchecked 异常（Java 特色！）
//   4. throw 抛出 与 throws 声明
//   5. 自定义异常
//   6. try-with-resources 自动关资源
// ============================================================

import java.io.IOException;

/** 自定义业务异常：继承 Exception = checked；继承 RuntimeException = unchecked */
class InsufficientBalanceException extends Exception {
    public InsufficientBalanceException(String message) {
        super(message);                       // 把消息传给父类，getMessage() 时能取到
    }
}

/** 演示用的迷你账户类 */
class SafeAccount {
    private double balance = 100;

    // throws 向调用者声明：本方法可能抛这种异常，你必须处理（checked 异常的强制契约）
    public void withdraw(double amount) throws InsufficientBalanceException {
        if (amount > balance) {
            // throw 是"动作"：真正抛出一个异常对象，立即中断执行
            throw new InsufficientBalanceException(
                    "余额 " + balance + " 不足以取款 " + amount);
        }
        balance -= amount;
        System.out.println("取款成功，余额: " + balance);
    }

    public double getBalance() { return balance; }
}

public class ExceptionsDemo {

    public static void main(String[] args) {
        // ==== 1. 捕获常见运行时异常 ====
        System.out.println("=== 捕获异常 ===");
        int[] arr = {1, 2, 3};
        try {
            System.out.println(arr[10]);      // 危险代码放进 try
            System.out.println("这行永远不会执行");
        } catch (ArrayIndexOutOfBoundsException e) {
            // 对比 C：越界是未定义行为直接崩溃或静默踩内存；
            // Java 抛异常给你一次补救机会。类似 Python 的 except IndexError:
            System.out.println("捕获数组越界: " + e.getMessage());
        }

        try {
            String s = null;
            s.length();                        // NullPointerException，最著名的异常
        } catch (NullPointerException e) {
            System.out.println("捕获空指针: 对象还没创建就调用方法");
        }

        // ==== 2. 多个 catch 与合并写法 ====
        System.out.println("\n=== 多 catch ===");
        try {
            Object o = "字符串";
            Integer num = (Integer) o;         // 强转失败 → ClassCastException
        } catch (ClassCastException | ArithmeticException e) {  // 用 | 合并无关联的异常
            System.out.println("捕获: " + e.getClass().getSimpleName());
        } catch (RuntimeException e) {          // 父类 catch 必须放最后！放前面会拦截一切
            System.out.println("兜底捕获: " + e);
        }

        // ==== 3. finally：无论是否异常都执行 ====
        System.out.println("\n=== finally ===");
        try {
            int r = 10 / 0;                    // ArithmeticException: / by zero
        } catch (ArithmeticException e) {
            System.out.println("捕获除零: " + e.getMessage());
            return;                            // 即使 return，finally 也会执行！
        } finally {
            System.out.println("finally 总会跑：适合放清理代码");
        }

        // ==== 4. checked 异常的强制契约 ====
        System.out.println("\n=== checked 异常 ===");
        SafeAccount acc = new SafeAccount();
        try {
            acc.withdraw(50);                  // 编译器强迫你处理 InsufficientBalanceException
            acc.withdraw(500);                 // 这次会炸
        } catch (InsufficientBalanceException e) {
            System.out.println("业务异常: " + e.getMessage());
        }
        // 这是 Java 特色！C++ 和 Python 都没有"必须声明/必须捕获"的强制机制。
        // 规则: 继承 Exception 的必须处理(checked)；继承 RuntimeException 的不强制(unchecked)

        // 也可以继续往上甩锅：main 自己也声明 throws，异常就交给 JVM 打印栈轨迹
        // public static void main(String[] args) throws IOException {...}

        // ==== 5. try-with-resources（Java 7+）====
        System.out.println("\n=== try-with-resources ===");
        // 资源（文件、网络、Scanner...）必须在括号里声明，
        // 结束时自动调用 close()——不管正常结束还是异常结束。
        // 对比 Python 的 with open(...) as f: 一模一样的思想；
        // C 只能手动手动 close，忘了就泄漏。
        class Resource implements AutoCloseable {
            Resource() { System.out.println("[资源] 打开"); }
            void work() { System.out.println("[资源] 干活"); }
            @Override public void close() { System.out.println("[资源] 自动关闭!"); }
        }
        try (Resource res = new Resource()) {
            res.work();
        }                                    // 不用写 close，这里自动调了

        /*
         * ---- 实战经验 ----
         * 1. 不要用异常做流程控制（性能差且难读）
         * 2. catch 之后要么处理要么抛出，别吞掉异常什么都不干
         * 3. 业务错误自定义 checked 异常；编程 bug（NPE 等）让它 unchecked 崩出来
         *
         * ---- 一句话总结 ----
         * try-catch-finally 结构和 Python 几乎一致；
         * 记住 Java 特色 checked 异常 + try-with-resources 自动关资源。
         */
    }
}
