"""面向对象 —— 对应菜鸟教程《Python3 面向对象》

与 C++ 类比：类 / 继承 / 多态概念相同，
区别是无需头文件、成员动态绑定、self 显式传递。
"""


class Student:
    school = "PySchool"                     # 类属性（所有实例共享）

    def __init__(self, name, score):        # 构造方法，self 相当于 this
        self.name = name                    # 实例属性
        self.score = score
        self.__secret = "hidden"            # 双下划线开头 -> 私有成员（名称改写）

    def show(self):
        print(f"{self.name}: {self.score}")

    def get_secret(self):                   # 通过公有方法访问私有属性
        return self.__secret

    def __str__(self):                      # 魔法方法：print(obj) 时调用
        return f"Student({self.name}, {self.score})"


s = Student("Tom", 90)
s.show()
print(s)                                    # Student(Tom, 90)
print(Student.school, s.school)
print(s.get_secret())
# print(s.__secret)  # AttributeError: 已被改写为 _Student__secret


# 继承
class GraduateStudent(Student):
    def __init__(self, name, score, advisor):
        super().__init__(name, score)       # 调用父类构造
        self.advisor = advisor

    def show(self):                         # 方法重写 -> 多态
        print(f"[研究生] {self.name}: {self.score} 导师={self.advisor}")


g = GraduateStudent("Jerry", 95, "Dr.Li")
g.show()


# 多态：同一接口，不同实现
def introduce(person):
    person.show()


for p in (s, g):
    introduce(p)

print(isinstance(g, Student), issubclass(GraduateStudent, Student))


# property：把方法包装成只读属性
class Circle:
    def __init__(self, r):
        self.r = r

    @property
    def area(self):
        return 3.14159 * self.r ** 2


print(Circle(2).area)


# classmethod / staticmethod
class Tool:
    count = 0

    @classmethod
    def incr(cls):                          # cls 接收类本身
        cls.count += 1
        return cls.count

    @staticmethod
    def ping():                             # 不依赖实例和类状态
        return "pong"


Tool.incr()
print(Tool.count, Tool.ping())


# 运算符重载：让自定义类型支持 + 等运算符
class Vector:
    def __init__(self, x, y):
        self.x, self.y = x, y

    def __add__(self, other):               # 使 v1 + v2 合法
        return Vector(self.x + other.x, self.y + other.y)

    def __repr__(self):
        return f"Vector({self.x}, {self.y})"


print(Vector(1, 2) + Vector(3, 4))
