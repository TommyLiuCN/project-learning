"""推导式 —— 对应菜鸟教程《Python3 推导式》"""

# 列表推导式：[表达式 for 变量 in 序列]
squares = [x ** 2 for x in range(1, 7)]
print(squares)                       # [1, 4, 9, 16, 25, 36]

# 带 if 过滤
evens = [x for x in range(10) if x % 2 == 0]
print(evens)

# 带 if...else 三元表达式时必须写在 for 前面
tags = ["偶" if x % 2 == 0 else "奇" for x in range(5)]
print(tags)

# 多重循环：笛卡尔积
pairs = [(x, y) for x in [1, 2] for y in [3, 4]]
print(pairs)

# 经典例子：矩阵转置
matrix = [[1, 2, 3], [4, 5, 6]]
transposed = [[row[i] for row in matrix] for i in range(3)]
print(transposed)                    # [[1, 4], [2, 5], [3, 6]]

# 字典推导式
d = {x: x ** 2 for x in (2, 4, 6)}
print(d)

xs = ["A", "B", "C"]
ys = ["a", "b", "c"]
print({k: v for k, v in zip(xs, ys)})   # zip 配对生成映射

# 集合推导式：结果自动去重
st = {x.strip() for x in ("he", " he", "she")}
print(st)

# 生成器表达式：惰性求值省内存，用 next() 或 for 取值
gen = (x ** 2 for x in range(5))
print(next(gen), list(gen))          # 0 [1, 4, 9, 16]
