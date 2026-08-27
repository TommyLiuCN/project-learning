"""基础语法 —— 对应菜鸟教程《Python3 基础语法》《Python3 注释》"""

# 单行注释：井号开头
"""
多行注释（本质是模块级文档字符串）。
Python 用缩进表示代码块，而不是 C/C++ 的花括号 {}。
"""

import keyword

# 1. 缩进即语法：同一代码块缩进必须一致（惯例 4 个空格）
if True:
    print("缩进通常为 4 个空格")
print("缩进不一致会导致 IndentationError")

# 2. 续行符 \ 与括号内自然换行
total = 1 + \
    2 + \
    3
nums = [
    1, 2,
    3, 4,
]
print(total, nums)

# 3. 多变量赋值
a = b = c = 1               # 三个变量指向同一个整型对象 1
x, y, z = 1, 2, "runoob"    # 同时赋不同类型的值
print(a, b, c, x, y, z)

# 4. 标识符规则：字母/下划线开头，大小写敏感；关键字不能作标识符
print(len(keyword.kwlist), "个关键字:")
print(keyword.kwlist)

# 5. import 的三种方式
from math import sqrt          # 导入指定成员
import random as rnd           # 起别名
import os                      # 整个模块

print(sqrt(16), rnd.randint(1, 6), os.name)
