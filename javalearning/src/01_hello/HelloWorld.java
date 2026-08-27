// ============================================================
// 第1章 · Hello World —— Java 程序的最小骨架
// ============================================================
// 运行: java src/01_hello/HelloWorld.java
// 学习要点:
//   1. 类与 main 方法是固定入口
//   2. println / print / printf 三种输出
//   3. Scanner 键盘输入（对比 C scanf、Python input）
//   4. nextInt 后接 nextLine 的经典坑
//   5. 三种注释
// ============================================================

import java.util.Scanner; // 引入工具类，类似 C 的 #include、Python 的 import

/**
 * 文件名必须和 public 类名一致：HelloWorld.java 里是 public class HelloWorld。
 * 对比 C 的 int main() 和 Python 的脚本顺序执行，
 * Java 一切代码都必须住在类里，入口写法是死的：
 * public static void main(String[] args)
 */
public class HelloWorld {

    public static void main(String[] args) {
        // ---- 1. 最简单的输出 ----
        // System.out.println 相当于 C 的 printf("...\n")、Python 的 print()
        System.out.println("Hello, World!");

        // ---- 2. print vs println vs printf ----
        System.out.print("这行不带换行，");       // print 不换行
        System.out.println("接着打印后换行");
        System.out.printf("我叫 %s，今年 %d 岁%n", "小明", 20); // printf 和 C 几乎一样
        // 区别：%n 是跨平台换行符，\n 是固定 \n

        // ---- 3. 字符串拼接用 + ----
        // Python 也支持 + 拼接；C 需要手动 strcat 或 sprintf，Java 直接加号
        String name = "Java";
        int year = 1995;
        System.out.println(name + " 诞生于 " + year + " 年");

        // ---- 4. 用户输入：Scanner ----
        // 对比 Python 的 input()（一行搞定），Java 需要先 new 一个 Scanner 对象
        // 对比 C 的 scanf("%d", &x)，不需要取地址符 &
        Scanner scanner = new Scanner(System.in);
        System.out.print("请输入你的名字: ");
        String userName = scanner.nextLine(); // 读一整行（含空格），类似 Python input()

        System.out.print("请输入你的年龄: ");
        int age = scanner.nextInt();          // 读一个整数，遇到空格/回车停止

        System.out.println("欢迎，" + userName + "！明年你 " + (age + 1) + " 岁");

        // ---- 5. 经典坑：nextInt() 之后 nextLine() 会读到空行 ----
        // nextInt 只消费数字，不消费行尾的回车符；
        // 紧跟的 nextLine 会把这个"残留回车"读走，返回空字符串。
        // 解决办法：先补一个 scanner.nextLine() 吃掉回车。
        scanner.nextLine(); // 故意吃掉残留的换行符
        System.out.print("请输入一句话介绍自己: ");
        String intro = scanner.nextLine();
        System.out.println("你的介绍: " + intro);

        // ---- 6. 命令行参数 ----
        // java HelloWorld 参数1 参数2 → args 数组里就是 ["参数1", "参数2"]
        // 对比 C 的 argv[1]、Python 的 sys.argv[1:]
        System.out.println("命令行参数个数: " + args.length);
        for (String arg : args) {
            System.out.println("  参数: " + arg);
        }

        scanner.close(); // 用完关闭，释放底层资源（Python 有 with 自动关，Java 见第11章 try-with-resources）

        /*
         * ---- 7. 三种注释回顾 ----
         * 1) 单行注释 //
         * 2) 多行注释
         * 3) 文档注释 /** ... （可被 javadoc 工具提取生成 API 文档）
         */
    }
}
