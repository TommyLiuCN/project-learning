"""条件控制与循环 —— 对应菜鸟教程《Python3 条件控制》《Python3 循环语句》"""

age = 20

# if / elif / else，注意冒号和缩进代替 C 的括号
if age < 18:
    print("未成年")
elif age < 60:
    print("成年人")
else:
    print("老年")

# 条件表达式（三元运算符）
parity = "偶数" if age % 2 == 0 else "奇数"
print(parity)

# match-case 结构化匹配（Python 3.10+）
command = "start"
match command:
    case "start":
        print("启动")
    case "stop" | "quit":      # 或模式
        print("停止")
    case _:                    # 通配符，类似 default
        print("未知命令")

# while 循环 + else：循环未被 break 打断时执行 else
count = 0
while count < 3:
    print("while:", count)
    count += 1
else:
    print("while 正常结束")

# for 可遍历任意可迭代对象（字符串、列表、range...）
for ch in "ab":
    print(ch, end=" ")
print()

for i in range(1, 10, 2):      # range(起, 止, 步长) 含头不含尾
    print(i, end=" ")
print()

# break 跳出循环 / continue 跳过本次 / pass 占位
for i in range(5):
    if i == 3:
        break
    if i == 1:
        continue
    print(i, end=" ")
print()


def todo_later():
    pass                        # 保持语法完整的空实现


# enumerate 带下标遍历；zip 并行遍历多个序列
for idx, ch in enumerate("abc"):
    print(idx, ch, end="; ")
print()
for num, letter in zip([1, 2], "xy"):
    print(num, letter, end="; ")
print()

# 经典例题：九九乘法表（嵌套循环）
for i in range(1, 10):
    for j in range(1, i + 1):
        print(f"{j}x{i}={i * j}", end="\t")
    print()
