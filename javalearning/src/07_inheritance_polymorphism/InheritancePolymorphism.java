// ============================================================
// 第7章 · 继承与多态
// ============================================================
// 运行: java src/07_inheritance_polymorphism/InheritancePolymorphism.java
// 学习要点:
//   1. extends 继承
//   2. super 调用父类构造/方法
//   3. 方法重写 @Override（vs 重载 overload）
//   4. 多态：父类引用指向子类对象
//   5. instanceof 与向下转型
//   6. equals/hashCode 重写
// ============================================================

class Animal {
    protected String name;   // protected：子类可以直接访问

    public Animal(String name) {
        this.name = name;
        System.out.println("[Animal 构造] " + name);
    }

    public void eat() {
        System.out.println(name + " 在吃东西");
    }

    public void speak() {
        System.out.println(name + " 发出声音");
    }
}

class Cat extends Animal {   // extends = 继承。Java 只支持单继承（C++ 支持多继承）
                             // Python 也支持多继承但容易菱形混乱，Java 用接口弥补（第8章）
    public Cat(String name) {
        super(name);         // super(...) 必须是第一行：先造好"父类部分"
        System.out.println("[Cat 构造] 完成");
    }

    @Override
    public void speak() {    // 重写(override)：签名相同，覆盖父类实现
        System.out.println(name + "：喵~");
    }

    public void scratch() {  // 子类独有方法
        System.out.println(name + " 挠了挠沙发");
    }
}

public class InheritancePolymorphism {

    public static void main(String[] args) {
        // ==== 1. 继承基础 ====
        Cat cat = new Cat("咪咪");
        cat.eat();       // 从父类白嫖来的方法
        cat.speak();     // 子类重写过的版本
        cat.scratch();   // 子类自己的方法

        // ==== 2. 多态：核心概念！ ====
        System.out.println("\n=== 多态 ===");
        Animal[] zoo = { new Cat("小花"), new Animal("神秘生物") };
        for (Animal a : zoo) {
            a.speak();   // 同一句代码，运行时自动调用各自的重写版本
                         // C++ 需要 virtual 关键字才有这效果；Java 方法默认全是"虚函数"
                         // Python 天生多态（鸭子类型），连继承都不需要
        }

        // 多态的意义：方法参数用父类型，就能接收所有子类，扩展新子类无需改旧代码
        feed(new Cat("橘猫"));
        feed(new Animal("草泥马"));

        // ==== 3. instanceof 与向下转型 ====
        System.out.println("\n=== 类型判断 ===");
        Animal unknown = new Cat("汤姆");
        // unknown.scratch();  // 编译错误！编译器只认 Animal 的能力清单

        if (unknown instanceof Cat) {          // 运行时检查真实身份，类似 Python isinstance()
            Cat c = (Cat) unknown;             // 向下转型必须显式强转
            c.scratch();
        }

        // Java 16+ 模式匹配：判断+转换一步到位（推荐写法）
        if (unknown instanceof Cat c2) {
            c2.scratch();
        }
        // 强转乱转会在运行时抛 ClassCastException——所以转之前先用 instanceof 确认

        // ==== 4. 重写 vs 重载 ====
        System.out.println("\n=== 重写 vs 重载 ===");
        // 重写 override：父子类之间、方法签名相同、@Override 标记 → 多态的基石
        // 重载 overload：同一个类里、方法名相同参数不同 → 调用时的便利语法糖（见第4章）

        // ==== 5. Object 万物之祖 ====
        System.out.println("任何类都继承 Object: " + (cat instanceof Object));
        System.out.println("默认 toString: " + cat.toString());
        // 不重写 toString 会打印 类名@哈希码，所以实体类都应重写（见第6章 BankAccount）

        // equals 默认也是 == （比地址），内容比较需重写：
        String s1 = new String("abc");
        String s2 = new String("abc");
        System.out.println("String 已重写 equals: " + s1.equals(s2)); // true
    }

    // 参数是父类型 → 能喂任何动物。新增 Dog/Bird 类也不用改这个方法
    static void feed(Animal a) {
        a.speak();
        a.eat();
    }
}
