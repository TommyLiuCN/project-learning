// ============================================================
// 第6章 · 面向对象基础：类与对象
// ============================================================
// 运行: java src/06_oop_basics/OopBasics.java
// 学习要点:
//   1. 类与对象（new 创建）
//   2. 构造方法重载与 this(...) 调用链
//   3. 封装：private + getter/setter
//   4. static 类变量与类方法
//   5. 四种访问权限
//   6. record 只读数据类（Java 16+）
// ============================================================

/**
 * 最简单的类：属性 + 行为。
 * 对比 C 的 struct（只有数据没有方法）；对比 Python 的 class（语法更松）。
 */
class Dog {
    // 成员变量（字段）—— 不初始化也有默认值：int 是 0，boolean 是 false，引用是 null
    String name;
    int age;

    // 构造方法：名字必须和类名相同、不写返回类型。
    // 对比 Python 的 __init__；对比 C++ 的构造函数——概念一致
    Dog(String name, int age) {
        this.name = name; // this 指当前对象，类似 Python 的 self（但必须显式写）
        this.age = age;
    }

    void bark() {           // 实例方法：必须通过对象调用
        System.out.println(name + "（" + age + " 岁）：汪汪！");
    }
}

/**
 * 封装示范：字段全部 private，外界只能通过公开方法访问。
 * 好处：可以在 setter 里做校验，防止非法数据（比如年龄为 -5）。
 * 对比 C 的 struct 全裸无保护；Python 用 _前缀约定私有但拦不住。
 */
class BankAccount {
    private String owner;                    // 私有字段
    private double balance;

    private static int totalAccounts = 0;    // static 字段：属于类，所有对象共享一份

    public BankAccount(String owner, double balance) {
        this.owner = owner;
        this.balance = balance;
        totalAccounts++;                     // 每创建一个账户就 +1
    }

    // 无参构造里用 this(...) 调用另一个构造方法，避免重复代码
    public BankAccount(String owner) {
        this(owner, 0.0);                    // 必须是构造方法第一行！
    }

    // ---- getter / setter ----
    public String getOwner() { return owner; }

    public double getBalance() { return balance; }  // 只读暴露，没有 setter → 外界改不了余额

    public void deposit(double amount) {
        if (amount <= 0) {                   // 封装的价值：入口处统一校验
            System.out.println("金额必须为正！");
            return;
        }
        balance += amount;
    }

    public static int getTotalAccounts() {   // static 方法：用类名调用，无需对象
        return totalAccounts;
        // static 方法里不能使用 this 和实例字段——因为没有具体对象
    }

    @Override                                // 注解：标记这是重写的方法（写错方法名会编译报错）
    public String toString() {               // 所有类默认继承 Object，重写 toString 定制打印
        return owner + " 的账户，余额 " + balance;
    }
}

/**
 * record（Java 16+）：一行声明只读数据类，
 * 自动生成构造器、getter、equals、hashCode、toString。
 * 对比 C++ 要手写一堆构造/比较代码，record 一行搞定。
 */
record Point(int x, int y) {
    double distance() {                      // record 里也可以定义方法
        return Math.sqrt(x * x + y * y);
    }
}

public class OopBasics {

    public static void main(String[] args) {
        // ==== 1. 创建对象 ====
        // new 在堆上分配内存并调用构造器。C: malloc + 手动初始化；Python: Dog("旺财",3)
        Dog d1 = new Dog("旺财", 3);
        Dog d2 = new Dog("小黑", 2);
        d1.bark();
        d2.bark();

        // ==== 2. 构造方法重载 ====
        BankAccount acc1 = new BankAccount("小明", 1000);
        BankAccount acc2 = new BankAccount("小红");          // 走无参版本，余额默认 0

        System.out.println("\n=== 封装 ===");
        acc1.deposit(500);
        acc1.deposit(-100);                                  // 被 setter 校验拦下
        System.out.println(acc1);                            // 直接打印自动调 toString()
        System.out.println("查询余额只能走 getter: " + acc1.getBalance());
        // acc1.balance = 999999;  // 编译错误！private 字段外界摸不到

        System.out.println("\n=== static ===");
        // static 属于类而非对象：两个账户了，计数是共享的
        System.out.println("总账户数: " + BankAccount.getTotalAccounts());
        // 类比 C 的全局变量、Python 的类属性——但有访问控制更安全

        System.out.println("\n=== record ===");
        Point p = new Point(3, 4);
        System.out.println(p + ", 到原点距离 = " + p.distance());
        System.out.println("x() 取值: " + p.x());             // record 的访问器不带 get 前缀

        /*
         * ---- 四种访问权限（从小到大）----
         * private     仅本类可见
         * (默认)      同一个包可见（不写修饰符时）
         * protected   同包 + 子类可见
         * public      全世界可见
         *
         * ---- 一句话总结 ----
         * Java 万物皆类的对象；封装 = private 字段 + 公开方法；
         * static 归类所有；简单数据载体直接上 record。
         */
    }
}
