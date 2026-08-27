"""模块、包与作用域 —— 对应菜鸟教程《Python3 模块》《__name__》《命名空间和作用域》"""
import sys

# 导入同目录下的自定义模块 mymodule.py
import mymodule

print(mymodule.PI)
print(mymodule.circle_area(2))

from mymodule import greeting       # 只导入指定成员

print(greeting("Python"))

import mymodule as mm               # 起别名

print(mm.greeting("别名导入"))

# __name__ 的作用：直接运行本文件时是 "__main__"，被导入时是模块名
# 把测试代码放在 if __name__ == "__main__": 里，模块可复用又不会误执行
print(__name__)

# 模块搜索路径：脚本所在目录 -> sys.path 各目录 -> site-packages
print(sys.path[:2])

# 命名空间与作用域查找顺序 LEGB：
# Local(局部) -> Enclosing(嵌套函数外层) -> Global(全局) -> Built-in(内置)
g_var = "全局变量"


def scope_demo():
    l_var = "局部变量"
    print(l_var, g_var)             # 内层可以读取全局变量


scope_demo()

import builtins

print(builtins.len("abc") == len("abc"))    # len 本来就来自内置作用域

if __name__ == "__main__":
    print("作为脚本直接运行")
