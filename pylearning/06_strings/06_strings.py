"""字符串(String) —— 对应菜鸟教程《Python3 字符串》"""

my_str = "Runoob"

# 索引从 0 开始，-1 为末尾；切片 [头:尾:步长] 含头不含尾
print(my_str[0], my_str[-1])     # R b
print(my_str[0:-1])              # Runoo
print(my_str[2:5])               # noo
print(my_str[2:])                # noob
print(my_str[::-1])              # boonuR 反转

# + 连接，* 重复
print(my_str + "TEST")           # RunoobTEST
print(my_str * 2)                # RunoobRunoob

# 转义字符与原始字符串 r
print("Ru\noob")                 # \n 换行
print(r"Ru\noob")                # 原样输出 Ru\noob

# 三引号可跨越多行
poem = """静夜思，
床前明月光。"""
print(poem)

# 字符串不可变：word[0] = 'M' 会报 TypeError，只能新建字符串
word = "Python"
new_word = "J" + word[1:]
print(new_word)                  # Jython

# 常用内置方法
print(len(my_str))                        # 长度 6
print("abc".upper(), "ABC".lower())
print("  hi  ".strip())                   # 去两端空白
print("a,b,c".split(","))                 # ['a', 'b', 'c']
print("-".join(["2026", "08", "21"]))     # 2026-08-21
print("hello".find("ll"))                 # 子串下标，找不到返回 -1
print("hello".replace("l", "L"))
print("runoob".startswith("ru"), "runoob".endswith("ob"))
print("abc123".isdigit(), "abc".isalpha())

# 格式化三种方式：f-string（推荐）/ str.format / %
name, score = "Tom", 92.5
print(f"{name} 的成绩是 {score:.1f}")
print("{} 的成绩是 {:.1f}".format(name, score))
print("%s 的成绩是 %.1f" % (name, score))

# chr / ord：字符与码点互转；in 成员判断
print(ord("A"), chr(66))         # 65 B
print("Py" in "Python")          # True
