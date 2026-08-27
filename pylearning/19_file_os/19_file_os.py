"""文件与 os —— 对应菜鸟教程《Python3 File》《Python3 OS File/目录方法》"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
PATH = os.path.join(HERE, "demo.txt")

# 写文件：w 覆盖写 / a 追加写；显式指定编码避免乱码
with open(PATH, "w", encoding="utf-8") as f:
    f.write("第一行 Hello Python\n")
    f.writelines(["第二行\n", "第三行\n"])

# 读文件：read 全部 / readline 一行 / readlines 返回列表
with open(PATH, "r", encoding="utf-8") as f:
    print(f.read(), end="")
with open(PATH, encoding="utf-8") as f:          # r 模式可省略
    print(f.readline(), end="")                  # 第一行
    print(f.readlines()[0], end="")

# 逐行迭代（大文件友好，不会一次性载入内存）
with open(PATH, encoding="utf-8") as f:
    for line in f:
        print(line.strip().split()[0], end=" ")
print()

# with 上下文管理器：块结束时自动 close()，等价于 C++ 的 RAII
print("with 结束后自动关闭:", f.closed)

# os 模块：目录与路径操作
print(os.getcwd())
os.makedirs(os.path.join(HERE, "subdir"), exist_ok=True)   # 递归建目录
print(sorted(os.listdir(HERE)))
print(os.path.exists(PATH), os.path.isfile(PATH), os.path.getsize(PATH))
print(os.path.basename(PATH), os.path.splitext(PATH)[1])

# pathlib：面向对象的现代路径 API（推荐新项目使用）
from pathlib import Path

p = Path(HERE) / "subdir" / "inner.txt"
p.write_text("pathlib 写入", encoding="utf-8")
print(p.read_text(encoding="utf-8"), [str(x.name) for x in Path(HERE).glob("*.txt")])
p.unlink()                                       # 删除文件

# 清理演示产物
os.remove(PATH)
os.rmdir(os.path.join(HERE, "subdir"))
print("清理完成:", sorted(x for x in os.listdir(HERE) if not x.startswith("__")))
