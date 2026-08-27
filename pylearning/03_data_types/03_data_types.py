"""基本数据类型总览 —— 对应菜鸟教程《Python3 基本数据类型》"""

# 变量没有类型，"类型"属于变量所引用的对象
x = 10             # 整数 int
y = 3.14           # 浮点数 float
name = "Alice"     # 字符串 str
is_active = True   # 布尔 bool

a, b, c = 1, 2, "three"

# type() 查看对象类型
print(type(x))            # <class 'int'>
print(type(y), type(name), type(is_active))

# isinstance() 判断类型
print(isinstance(a, int))    # True


# type() 不认为子类是父类类型；isinstance() 认为
class Animal:
    pass


class Dog(Animal):
    pass


d = Dog()
print(type(d) == Animal)      # False
print(isinstance(d, Animal))  # True

# 六种标准数据类型：
# 不可变（4 个）：Number、String、bool、Tuple
# 可变　（3 个）：List、Dictionary、Set
n, s, t = 123, "abc", (1, 2)
lst, dic, st = [1], {"k": 1}, {1, 2}
print(type(n), type(s), type(t))
print(type(lst), type(dic), type(st))

# del 删除的是变量名（引用），不是对象本身
var1 = var2 = 100
del var1, var2
try:
    print(var1)
except NameError as e:
    print("引用已删除:", e)
