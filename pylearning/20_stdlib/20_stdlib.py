"""标准库概览 —— 对应菜鸟教程《Python3 标准库概览》"""
import collections
import datetime
import itertools
import json
import os
import re
import sys
import timeit

# sys / os：系统交互
print(sys.version.split()[0], os.name)

# datetime：日期时间
now = datetime.datetime.now()
print(now.strftime("%Y-%m-%d %H:%M:%S"))
yesterday = datetime.datetime(2026, 1, 1) - datetime.timedelta(days=1)
print(yesterday.date())

# json：序列化与反序列化（网络传输、配置文件的常用格式）
data = {"name": "runoob", "scores": [90, 85], "ok": True}
text = json.dumps(data, ensure_ascii=False)
back = json.loads(text)
print(text, back["scores"][0])

# re：正则表达式
print(re.findall(r"\d+", "a1b22c333"))
print(re.sub(r"\s+", "-", "hello   world"))
m = re.match(r"(\w+)@(\w+)\.com", "tom@mail.com")
print(m.groups())

# collections：增强容器
cnt = collections.Counter("abracadabra")
print(cnt.most_common(2))
dq = collections.deque([1, 2, 3])
dq.appendleft(0)
dq.pop()
print(dq)
Point = collections.namedtuple("Point", ["x", "y"])
print(Point(1, 2).x)

# itertools：迭代工具
print(list(itertools.permutations("abc", 2)))
print(list(itertools.accumulate([1, 2, 3, 4])))

# timeit：测量小段代码性能
cost = timeit.timeit("[i**2 for i in range(1000)]", number=1000)
print(f"列表推导式耗时 {cost:.4f}s")
