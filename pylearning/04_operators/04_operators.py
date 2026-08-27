"""运算符 —— 对应菜鸟教程《Python3 运算符》"""

# ---- 算术运算符 ----
print("算术:", 5 + 4, 4.3 - 2, 3 * 7)
print("/ 得浮点数:", 2 / 4)          # 0.5
print("// 整除向下取整:", 2 // 4)     # 0
print("% 取余:", 17 % 3)              # 2
print("** 乘方:", 2 ** 5)             # 32

# ---- 比较运算符：返回 bool ----
print("比较:", 1 == 1, 2 != 3, 3 > 2, 2 >= 2, 1 < 0, 1 <= 1)

# ---- 赋值运算符 ----
n = 10
n += 5
n -= 3
n *= 2
n //= 3
n %= 4
n **= 2
print("复合赋值结果:", n)

# 海象运算符 := （Python 3.8+）：赋值的同时返回值
if (m := 7) > 5:
    print("海象运算符:", m)

# ---- 位运算符（与 C 语言一致）----
p, q = 0b1100, 0b1010
print("位运算:", p & q, p | q, p ^ q, ~q, q << 2, p >> 2)

# ---- 逻辑运算符：and / or / not（不是 && || !）----
print("逻辑:", True and False, True or False, not True)

# ---- 成员运算符 in / not in ----
print("成员:", "ru" in "runoob", 3 not in [1, 2])

# ---- 身份运算符 is / is not：比较是否同一对象（内存地址）----
aa = [1, 2]
bb = [1, 2]
print("值相等:", aa == bb)          # True
print("不是同一对象:", aa is bb)    # False
print(id(aa) == id(bb))             # id() 返回对象内存地址

# 注意：is 与 == 不同！比较字面量时应使用 == 而非 is
print(1 is True)    # False 且会有 SyntaxWarning，正确写法是 1 == True -> True
