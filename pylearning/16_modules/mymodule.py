"""自定义模块示例：被同目录的 16_modules.py 导入使用"""

PI = 3.14159


def circle_area(r):
    """圆面积"""
    return PI * r ** 2


def greeting(name):
    """问候语"""
    return f"你好, {name}"
