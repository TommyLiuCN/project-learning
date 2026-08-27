// ============================================================
// 第8章 · 抽象类与接口
// ============================================================
// 运行: java src/08_abstract_interfaces/AbstractInterfaces.java
// 学习要点:
//   1. 抽象类与抽象方法
//   2. 接口的定义与实现
//   3. 多实现（Java 单继承多实现）
//   4. default / static 接口方法
//   5. 抽象类 vs 接口怎么选
// ============================================================

/**
 * 抽象类：不能被 new，只能被继承。
 * 对比 C++：含纯虚函数 (=0) 的基类；对比 Python 的 abc.ABC。
 */
abstract class Shape {
    protected String name;

    public Shape(String name) {
        this.name = name;
    }

    public abstract double area();          // 抽象方法：只声明不实现，子类必须重写

    public void describe() {                // 普通方法：抽象类可以有实现和状态
        System.out.printf("%s 面积 = %.2f%n", name, area());
        // 这里调用的 area() 是子类的版本——模板方法模式雏形
    }
}

class Circle extends Shape {
    double r;

    public Circle(double r) {
        super("圆");
        this.r = r;
    }

    @Override
    public double area() { return Math.PI * r * r; } // 不重写抽象方法就编译报错！
}

class Rect extends Shape {
    double w, h;

    public Rect(double w, double h) {
        super("矩形");
        this.w = w;
        this.h = h;
    }

    @Override
    public double area() { return w * h; }
}

/**
 * 接口：纯"能力契约"，不含状态。
 * 对比 Python：鸭子类型/协议（Protocol）；对比 C++：没有语言级接口，靠纯虚类模拟。
 */
interface Flyable {
    void fly();                              // 接口方法默认就是 public abstract

    String MAX_ALTITUDE = "接口字段默认 public static final（常量）"; // 接口里的"变量"其实是常量
}

interface Swimmable {
    void swim();

    default void floatOnWater() {            // default 方法（Java 8+）：带默认实现
        System.out.println("漂浮在水面上");
    }
}

/**
 * 基类：演示"继承一个类 + 实现多个接口"的组合。
 */
class Bird {
    protected String name;

    public Bird(String name) {
        this.name = name;
    }
}

/**
 * 一个类只能 extends 一个父类，但可以 implements 多个接口——
 * 这就是 Java 弥补单继承限制的方式。
 * 类比：鸭子是鸟(is-a)，同时会飞、会游(can-do)。
 */
class Duck extends Bird implements Flyable, Swimmable {

    public Duck(String name) {
        super(name);
    }

    public void quack() { System.out.println(name + "：嘎嘎"); }

    @Override
    public void fly() { System.out.println(name + " 扑腾翅膀飞起来了"); }

    @Override
    public void swim() { System.out.println(name + " 划水前进"); }

    // floatOnWater 可以不重写，直接用接口的 default 版本
}

public class AbstractInterfaces {

    public static void main(String[] args) {
        // ==== 1. 抽象类多态 ====
        Shape[] shapes = { new Circle(1), new Rect(3, 4) };
        for (Shape s : shapes) {
            s.describe();                    // describe 是父类实现，area 调子类实现
        }
        // Shape s = new Shape("x");  // 编译错误！抽象类不能实例化

        // ==== 2. 接口多态 ====
        System.out.println("\n=== 接口 ===");
        Duck duck = new Duck("唐老鸭");
        duck.quack();

        Flyable f = duck;                    // 用接口类型引用——只暴露"会飞"的能力
        f.fly();
        // f.swim();                          // 编译错误！Flyable 里没有 swim

        Swimmable s = duck;
        s.swim();
        s.floatOnWater();                    // 用的是接口的 default 实现

        // ==== 3. 多实现 ====
        System.out.println("\n=== 多实现 ===");
        System.out.println("duck instanceof Flyable: " + (duck instanceof Flyable));
        System.out.println("duck instanceof Swimmable: " + (duck instanceof Swimmable));

        /*
         * ---- 抽象类 vs 接口 怎么选 ----
         * 抽象类: "是什么"(is-a)，有状态字段和构造器，单继承
         *          例: Dog is an Animal
         * 接口:   "能做什么"(can-do)，无状态、可多实现
         *          例: Duck can Fly/Swim
         * 经验法则: 定义能力用接口，共享代码骨架用抽象类
         *
         * ---- 一句话总结 ----
         * 抽象类是半成品模板，接口是能力证书；
         * Java 用"单继承 + 多实现"避开了 C++ 多继承的菱形灾难。
         */
    }
}
