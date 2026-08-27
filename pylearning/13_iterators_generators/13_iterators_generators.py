"""迭代器与生成器 —— 对应菜鸟教程《Python3 迭代器与生成器》"""

# 可迭代对象 -> iter() 得到迭代器 -> next() 逐个取值
lst = [1, 2, 3]
it = iter(lst)
print(next(it), next(it), next(it))
try:
    next(it)
except StopIteration:
    print("迭代结束抛出 StopIteration")

for x in lst:        # for 循环的本质就是自动迭代
    pass


# 自定义迭代器类：实现 __iter__ 和 __next__
class Countdown:
    def __init__(self, start):
        self.n = start

    def __iter__(self):
        return self

    def __next__(self):
        if self.n <= 0:
            raise StopIteration
        self.n -= 1
        return self.n + 1


for n in Countdown(3):
    print(n, end=" ")
print()


# 生成器函数：含 yield 的函数，调用时不立即执行
def fib(limit):
    a, b = 0, 1
    while a < limit:
        yield a          # 暂停并交出一个值，下次从这里继续
        a, b = b, a + b


for v in fib(10):
    print(v, end=" ")
print()

gen = fib(100)
print(next(gen), next(gen))     # 0 1

# 生成器惰性求值：处理大数据不用一次性载入内存
big = (i for i in range(10 ** 6))
print(sum(big))


# yield from 委托子生成器
def chain():
    yield from "ab"
    yield from [3, 4]


print(list(chain()))
