"""第一个 Python 程序 —— 对应菜鸟教程《Python3 基础语法》入门"""

print("Hello, World!")
print("这是一个 Python 程序")

# 变量无需声明类型，赋值即创建（对比 C：int counter = 100;）
counter = 100          # 整型变量
miles = 1000.0         # 浮点型变量
name = "runoob"        # 字符串

print(counter)
print(miles)
print(name)

# 命令行参数（类似 C 的 main(int argc, char *argv[])）
import sys

print("脚本名:", sys.argv[0])
if len(sys.argv) > 1:
    print("参数列表:", sys.argv[1:])
