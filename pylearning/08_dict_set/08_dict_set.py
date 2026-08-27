"""字典与集合 —— 对应菜鸟教程《Python3 字典》《Python3 集合》"""

# ---- Dictionary 键值对映射：键唯一且必须是不可变类型 ----
tinydict = {"name": "runoob", "code": 1, "site": "www.runoob.com"}

print(tinydict["name"])        # 按键访问
print(tinydict.keys())         # 所有键
print(tinydict.values())       # 所有值
print(tinydict.items())        # 键值对视图

tinydict["code"] = 2           # 修改
tinydict["author"] = "菜鸟"    # 新增（Python 3.7 起保持插入顺序）
del tinydict["site"]           # 删除键值对
print(tinydict)
print(tinydict.get("site", "默认值"))   # 安全访问：不存在给默认值

# 遍历字典默认遍历键；items() 同时取键值
for k, v in tinydict.items():
    print(k, ":", v)

# dict() 构造函数与字典推导式
d1 = dict([("Runoob", 1), ("Google", 2)])
d2 = {x: x ** 2 for x in (2, 4, 6)}
print(d1, d2)

empty_dict = {}      # 注意：{} 创建的是空字典！
print(type(empty_dict))

# ---- Set 无序集合：元素唯一，支持集合运算 ----
sites = {"Google", "Taobao", "Runoob", "Facebook", "Zhihu", "Baidu"}
print(sites)                   # 输出顺序不固定，重复元素被自动去掉
print("Runoob" in sites)       # 成员测试 O(1)，类似散列表查找

a = set("abracadabra")
b = set("alacazam")
print(sorted(a - b))           # 差集：在 a 不在 b
print(sorted(a | b))           # 并集
print(sorted(a & b))           # 交集
print(sorted(a ^ b))           # 对称差集：只在其中一个

s = {1, 2, 3}
s.add(4)
s.discard(99)                  # 元素不存在也不报错（remove 会 KeyError）
s.update([5, 6])
print(s)

empty_set = set()    # 创建空集合必须用 set()
print(type(empty_set))

# frozenset：不可变集合，可作字典的键或另一集合的元素
fs = frozenset([1, 2, 3])
print(fs)
