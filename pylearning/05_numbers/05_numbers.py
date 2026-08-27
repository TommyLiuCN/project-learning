"""数字(Number) —— 对应菜鸟教程《Python3 数字(Number)》"""

# Python3 支持 int、float、bool、complex；没有 Python2 的 long
i, f, bl, cx = 20, 5.5, True, 4 + 3j
print(type(i), type(f), type(bl), type(cx))

# 进制字面量：二进制 0b / 八进制 0o / 十六进制 0x（整数不允许前导零如 080）
print(0b1010, 0o17, 0x69)              # 10 15 105
print(bin(10), oct(10), hex(10))       # 十进制转其他进制字符串

# 数值运算：混合计算时整型自动转为浮点数
print(5 + 4, 2 / 4, 2 // 4, 17 % 3, 2 ** 5)
print(type(3 + 2.0))                   # <class 'float'>

# bool 是 int 的子类：True 等价 1、False 等价 0
print(issubclass(bool, int), int(True), True + 1, False * 10)

# 复数：real 实部、imag 虚部、conjugate() 共轭复数
print(cx.real, cx.imag, cx.conjugate())

# 内置数学函数
print(abs(-7), max(1, 9, 3), min(4, -2), round(3.14159, 2))

# math 模块常用函数
import math

print(math.ceil(4.1), math.floor(4.9), math.sqrt(16), math.pi)

# random 模块：随机数
import random

random.seed(42)                         # 固定种子保证可重复
print(random.random())                  # [0,1) 随机浮点数
print(random.randint(1, 10))            # [1,10] 随机整数
print(random.choice(["a", "b", "c"]))   # 随机取一个元素
print(random.sample(range(100), 3))     # 不重复抽样
