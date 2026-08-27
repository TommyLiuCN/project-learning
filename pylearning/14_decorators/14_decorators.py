"""装饰器 —— 对应菜鸟教程《Python 装饰器》（进阶内容）"""
import time
from functools import wraps


# 闭包：内层函数记住并访问外层函数的变量
def make_multiplier(k):
    def multiply(x):
        return x * k
    return multiply


double = make_multiplier(2)
print(double(21))            # 42


# 装饰器本质：接收函数并返回新函数，@ 是语法糖
def log_calls(func):
    @wraps(func)                     # 保留原函数的元信息（名字、文档）
    def wrapper(*args, **kwargs):
        print(f"调用 {func.__name__}，参数 {args}")
        result = func(*args, **kwargs)
        print(f"{func.__name__} 返回 {result}")
        return result
    return wrapper


@log_calls                 # 等价于 add = log_calls(add)
def add(a, b):
    """计算两数之和"""
    return a + b


add(2, 3)
print(add.__name__)        # add（没有 wraps 会变成 wrapper）


# 实用示例：计时装饰器（类似 C++ 的性能打点）
def timer(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        t0 = time.perf_counter()
        result = func(*args, **kwargs)
        cost = time.perf_counter() - t0
        print(f"{func.__name__} 耗时 {cost * 1000:.3f} ms")
        return result
    return wrapper


@timer
def busy():
    sum(i * i for i in range(200_000))


busy()


# 带参数的装饰器：在装饰器外面再包一层函数
def repeat(times):
    def deco(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            result = None
            for _ in range(times):
                result = func(*args, **kwargs)
            return result
        return wrapper
    return deco


@repeat(times=3)
def hello():
    print("Hello!")


hello()
