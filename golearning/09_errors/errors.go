// ============================================================
// 第9章 · 错误处理
// ============================================================
// 运行: go run ./09_errors
// 学习要点:
//  1. error 只是个接口：作为普通返回值传递，不是异常机制
//  2. 黄金惯例：if err != nil { return ... }——逐层显式处理
//  3. 自定义错误类型 + errors.As 提取细节
//  4. 错误包装 %w 与 errors.Is：保留完整错误链
//  5. panic/recover：留给真正异常的场景（对照 Python raise/except）
//
// ============================================================
package main

import (
	"errors"
	"fmt"
	"strconv"
)

// error 接口的定义其实只有一句话：
//
//	type error interface { Error() string }
//
// 所以任何类型实现了 Error() string 就是一个 error（又是隐式接口，第8章的知识）
type ValidationError struct {
	Field string
	Msg   string
}

func (e *ValidationError) Error() string {
	return fmt.Sprintf("字段 %s 校验失败: %s", e.Field, e.Msg)
}

func validate(age int) error {
	if age < 0 {
		// 返回具体的错误类型，调用方稍后可用 errors.As 还原出结构体拿细节
		// 注意返回指针：errors.As 匹配的是指针类型
		return &ValidationError{Field: "age", Msg: "不能为负数"}
	}
	return nil
}

// 包装错误：%w 动词把底层错误嵌进新错误形成错误链
// 对照 Python 的 raise ... from e；C 只有全局 errno，谈不上链
func loadConfig(path string) error {
	if err := validate(-1); err != nil {
		return fmt.Errorf("加载配置 %s 失败: %w", path, err)
	}
	return nil
}

func risky(deep bool) {
	if deep {
		panic("不可恢复的状态！") // 类似 Python 的 raise、C++ 的 throw
	}
}

func main() {
	// ==== 1. 最基本的错误处理 ====
	// 三种风格对比：
	//   C:      返回 -1 + 全局 errno，忘了检查也没人拦你
	//   Python: try/except 打断控制流，异常可能从任何深处飞出来
	//   Go:     错误是普通返回值，语法逼着你逐层处理，控制流一目了然
	val, err := strconv.Atoi("not_a_number")
	if err != nil { // 黄金法则：拿到 err 立刻处理或立刻向上返回，绝不晾着
		fmt.Println("解析失败:", err)
	} else {
		fmt.Println("解析成功:", val)
	}

	// errors.New 创建简单错误；fmt.Errorf 带格式化信息
	e1 := errors.New("资源不存在")
	e2 := fmt.Errorf("用户 %d: %w", 42, e1) // %w 包装 e1
	fmt.Println(e2)

	// errors.Is 沿着包装链逐层找目标错误（== 只比较最外层对象）
	fmt.Println(errors.Is(e2, e1)) // true：链上有 e1
	fmt.Println(e2 == e1)          // false：外层是新建的包装错误

	// ==== 3. 自定义错误与 errors.As ====
	err = loadConfig("/etc/app.conf")
	fmt.Println(err) // 直接打印整条链的拼接消息

	var ve *ValidationError
	if errors.As(err, &ve) { // As 从链中提取指定【类型】，拿到结构体细节
		fmt.Println("提取到校验错误 → 字段:", ve.Field, "原因:", ve.Msg)
	}

	// 经验法则：
	//   判断身份用 Is（哨兵错误/链查找），提取细节用 As（自定义类型），
	//   两者都接受被包装过的错误，永远优先于 == 和类型断言

	// ==== 5. panic / recover ====
	// panic = 进入崩溃流程，沿调用栈向上炸（Python 未捕获异常的样子）
	// recover 只能在 defer 的函数里生效，把 panic 值拦截回来
	func() {
		defer func() {
			if r := recover(); r != nil { // r 是 panic 时传出的值
				fmt.Println("恢复自 panic:", r)
			}
		}()
		risky(true)
		fmt.Println("这行不会执行") // panic 之后同函数的代码跳过
	}()
	fmt.Println("主流程还在继续")

	// 使用准则：
	//   可预期的失败（文件不存在/网络超时/输入非法）→ 用 error
	//   不可预期的 bug（数组越界/nil 解引用）→ 让 panic 自然发生别去 recover 它
	//   服务程序入口可 recover 兜底，防止单个请求拖垮整个进程

	/*
	 * ---- 一句话总结 ----
	 * 错误即值：if err != nil 写到手软但流程透明；
	 * 包装用 %w、判断用 Is、提取用 As；
	 * panic 是炸弹不是流程控制工具，recover 只配给服务边界用。
	 */
}
