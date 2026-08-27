"""函数 —— 对应菜鸟教程《Python3 函数》《Python3 lambda》"""


def greet(name):
    """文档字符串：说明函数用途。返回问候语。"""
    return f"Hello, {name}!"


print(greet("Python"))
print(greet.__doc__.strip())     # 可通过 __doc__ 访问文档字符串


# 默认参数：必须放在必选参数之后
def power(x, n=2):
    return x ** n


print(power(3), power(3, 3))

# 关键字参数：调用时可乱序传参，可读性好
print(power(n=3, x=2))


# 不定长参数：*args 收集成元组，**kwargs 收集成字典
def summary(*args, **kwargs):
    print(args, kwargs)


summary(1, 2, 3, name="py")


# 返回多个值本质是返回元组再解包
def divmod_(a, b):
    return a // b, a % b


q, r = divmod_(17, 5)
print(q, r)

# 作用域：函数内修改全局变量需要 global 声明
count = 0


def inc():
    global count
    count += 1


inc()
inc()
print(count)


# 嵌套函数：nonlocal 引用外层函数的局部变量
def outer():
    x = 10

    def inner():
        nonlocal x
        x += 1
        return x

    return inner()


print(outer())


# 递归：阶乘（写法与 C 相同）
def factorial(n):
    return 1 if n <= 1 else n * factorial(n - 1)


print(factorial(5))

# lambda 匿名函数：只能写单个表达式
square = lambda x: x ** 2
print(square(6))

# 常与 map / filter / sorted 的 key 配合使用
print(list(map(lambda v: v * 2, [1, 2, 3])))
print(list(filter(lambda v: v > 1, [1, 2, 3])))
print(sorted([(1, "b"), (2, "a")], key=lambda p: p[1]))


# 类型注解：仅提示不强制（对应《Python 类型注解》章节）
def add(a: int, b: int) -> int:
    return a + b


print(add(1, 2))
