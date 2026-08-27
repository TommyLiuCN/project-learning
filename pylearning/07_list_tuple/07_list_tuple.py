"""列表与元组 —— 对应菜鸟教程《Python3 列表》《Python3 元组》"""

# ---- List 可变序列：元素类型可以不同 ----
my_list = ["abcd", 786, 2.23, "runoob", 70.2]
tinylist = [123, "runoob"]

print(my_list[0])            # abcd
print(my_list[1:3])          # [786, 2.23]
print(my_list[2:])           # 从索引 2 到末尾
print(tinylist * 2)          # 重复两次
print(my_list + tinylist)    # 拼接

# 切片步长：第三个参数为负数表示逆向
demo = [1, 2, 3, 4, 5, 6]
print(demo[1::2])            # [2, 4, 6]
print(demo[-1::-1])          # 反转

# 列表元素可以修改（可变类型）
a = [1, 2, 3, 4, 5, 6]
a[0] = 9
a[2:5] = [13, 14, 15]
print(a)
a[2:5] = []                  # 用空列表删除切片元素
print(a)                     # [9, 2, 6]

# 常用方法
nums = [3, 1, 4]
nums.append(5)               # 尾部追加
nums.insert(1, 99)           # 指定位置插入
nums.remove(99)              # 按值删除第一个匹配
last = nums.pop()            # 弹出并返回末尾元素
print(nums, last)
nums.extend([7, 8])          # 合并另一个列表
nums.sort()                  # 原地升序排序
print(nums, sorted(nums, reverse=True))
print(nums.index(7), nums.count(1))

# 嵌套列表（类似二维数组）
matrix = [[1, 2, 3], [4, 5, 6]]
print(matrix[1][2])          # 6

# ---- Tuple 不可变序列 ----
my_tuple = ("abcd", 786, 2.23)
tinytuple = (123, "runoob")
print(my_tuple[1:3])
print(my_tuple + tinytuple)

# my_tuple[0] = 11  # TypeError: 元组元素不可修改

tup1 = ()             # 空元组
tup2 = (20,)          # 单元素元组必须带逗号
not_a_tuple = (42)    # 这只是整数 42
print(tup2, type(tup2), not_a_tuple, type(not_a_tuple))

# 元组不可变，但其中的可变对象（如 list）内容仍可修改
mixed = (1, [2, 3])
mixed[1].append(4)
print(mixed)                 # (1, [2, 3, 4])


# 函数返回多个值时本质就是元组解包
def min_max(values):
    return min(values), max(values)


lo, hi = min_max([3, 1, 4, 1, 5])
print(lo, hi)
