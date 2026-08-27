"""输入和输出 —— 对应菜鸟教程《Python3 输入和输出》"""
import io
import sys

# print 的 sep / end 参数
print("a", "b", "c")                    # 默认空格分隔，换行结尾
print("2026", "08", "21", sep="-")
print("不换行", end=" -> ")
print("继续同行")

# str.format()
print("{0} 今年 {1} 岁，{0} 喜欢 Python".format("Tom", 18))
print("{name}:{score}".format(name="Jerry", score=97))
print("{:.2f} | {:>10} | {:<10} | {:^10}".format(3.14159, "右对齐", "左对齐", "居中"))

# f-string（推荐，Python 3.6+）
pi = 3.1415926
name, score = "Amy", 92.567
print(f"{name=} {score=}")              # 调试友好：name='Amy' score=92.567
print(f"{pi:.3f} {score:8.2f}")
print(f"大数分组: {1234567:,}")

# % 风格（旧式，与 C 的 printf 类似）
print("%s 的分数是 %.1f%%" % (name, score))

# str() 面向用户 / repr() 面向解释器
print(str("hi\n"), repr("hi\n"))

# input() 从键盘读入，返回值永远是字符串：
#   age = int(input("你的年龄: "))
#   print(f"明年 {age + 1} 岁")

# 用 StringIO 模拟一次输入来演示完整流程
sys.stdin = io.StringIO("42\n")
age = int(input())
print(f"读到年龄 {age}")
sys.stdin = sys.__stdin__               # 恢复标准输入
