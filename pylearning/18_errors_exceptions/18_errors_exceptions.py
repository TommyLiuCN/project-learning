"""错误和异常 —— 对应菜鸟教程《Python3 错误和异常》"""

# try / except 捕获指定异常
try:
    result = 10 / 0
except ZeroDivisionError:
    print("不能除以零")

# 多个 except 分支 + 获取异常对象；Exception 兜底要放最后
try:
    num = int("abc")
except (ValueError, TypeError) as e:
    print(f"转换出错: {e}")
else:
    print("未发生异常才执行 else")
finally:
    print("finally 无论是否异常都执行（常用于释放资源）")


# 异常沿调用栈向上传播，可在任意外层捕获
def level3():
    raise RuntimeError("最深处的错误")


def level2():
    level3()


def level1():
    try:
        level2()
    except RuntimeError as e:
        print(f"在最外层捕获: {e}")


level1()


# raise 主动抛出异常
def set_age(age):
    if not 0 < age < 150:
        raise ValueError(f"非法年龄: {age}")
    return age


print(set_age(20))
try:
    set_age(200)
except ValueError as e:
    print(e)


# 自定义异常类：继承 Exception
class ScoreError(Exception):
    """成绩非法"""


def check(score):
    if not 0 <= score <= 100:
        raise ScoreError(f"成绩必须在 0~100: {score}")
    return "OK"


try:
    check(120)
except ScoreError as e:
    print("自定义异常:", e)

# assert 断言：条件为假抛 AssertionError
assert 1 + 1 == 2
try:
    assert 1 + 1 == 3, "数学坏了"
except AssertionError as e:
    print("断言失败:", e)

# 常见内置异常速查
statements = ("undefined_name", "1 + 'a'", "[1][5]", '{"k":1}["x"]', "1/0")
caught = []
for stmt in statements:
    try:
        eval(stmt)
    except Exception as e:
        caught.append(type(e).__name__)
print(caught)   # NameError TypeError IndexError KeyError ZeroDivisionError
