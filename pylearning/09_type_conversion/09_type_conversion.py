"""数据类型转换 —— 对应菜鸟教程《Python3 数据类型转换》"""

# int()：字符串转整数可指定进制；浮点数转整数直接截断
print(int("100"), int(3.9), int("0x1A", 16), int("0b101", 2))

# float() / str() / repr()
print(float("3.14"), float(10))
print(str(123), repr("hi\n"))
print(len(str(123)))            # 转成字符串后可求长度 -> 3

# eval()：执行字符串中的有效 Python 表达式（注意安全风险）
print(eval("3 * 7"), eval("'py' * 2"))

# 序列之间的转换
print(tuple("abc"), list("abc"), set("aabbcc"))
print(list((1, 2)), tuple([1, 2]))
print(dict([("a", 1), ("b", 2)]))
print(frozenset([1, 2, 2]))

# chr / ord / hex / oct
print(chr(65), ord("A"), hex(255), oct(8))

# 隐式转换：混合计算时整型自动提升为浮点数
print(3 + 2.0, type(3 + 2.0))

# 转换失败会抛异常
try:
    int("abc")
except ValueError as e:
    print("转换失败:", e)

# bool()：None、False、0、0.0、空序列、空映射 都为 False，其余为 True
print(bool(0), bool(""), bool([]), bool({}), bool(None))
print(bool(42), bool("py"), bool([1]), bool(-2))
