// ============================================================
// 第5章 · 复合类型：数组、切片与映射
// ============================================================
// 运行: go run ./05_composite_types
// 学习要点:
//  1. 数组 [N]T：定长、值类型（赋值即整体拷贝！这点和 C 不同）
//  2. 切片 []T：动态数组视图（指针+len+cap 三件套），日常主力
//  3. append 的扩容行为与共享底层数组的坑
//  4. make 创建切片/map；nil 切片 vs 空 切片
//  5. map[K]V 哈希表（Python dict / C++ unordered_map）
//  6. comma-ok 写法：区分"键不存在"和"值为零值"
//
// ============================================================
package main

import (
	"fmt"
	"sort"
)

func modifyArray(a [3]int) { a[0] = 999 } // 形参是整份数组的拷贝

func modifySlice(s []int) { s[0] = 999 } // 拷贝的是切片头，底层数组仍共享！

func main() {
	// ==== 1. 数组：了解即可，实际开发几乎都用切片 ====
	arr := [3]int{1, 2, 3}    // 长度是类型的一部分：[3]int 和 [4]int 是不同类型！
	arr2 := [...]int{1, 2, 3} // ... 让编译器替你数元素个数
	fmt.Println(arr, arr2, len(arr))

	// 关键差异：数组是【值类型】，赋值/传参会完整拷贝整个数组
	// （C 数组名传参会退化为指针；Python list 是引用语义——Go 数组两头都不像）
	copyArr := arr
	copyArr[0] = 100
	fmt.Println("原数组不受影响:", arr[0])

	modifyArray(arr)
	fmt.Println("函数内修改不影响外面:", arr[0]) // 还是 1

	// ==== 2. 切片：动态数组，90% 场景用它 ====
	// 类比：Python 的 list、C++ 的 vector。
	// 本质是一个隐藏的底层数组 + 切片头(指向数组的指针, len 长度, cap 容量)，
	// 所以切片是"窗口/视图"而不是数据本身
	sl := []int{1, 2, 3} // 注意没有长度 → 这是切片不是数组
	fmt.Println(sl, len(sl), cap(sl))

	// make 创建指定长度/容量的切片，元素全部为零值
	// 对照 C: malloc(n * sizeof(int))；对照 Python 没有直接对应物
	buf := make([]int, 3, 10) // len=3（全是0），cap=10
	fmt.Println(buf, len(buf), cap(buf))

	// append 追加元素：容量不够时分配更大的新数组并迁移，
	// 可能换底层数组——所以【必须接收返回值】！
	sl = append(sl, 4)
	sl = append(sl, 5, 6)
	other := []int{7, 8}
	sl = append(sl, other...) // ... 展开追加另一个切片
	fmt.Println(sl, len(sl), cap(sl))

	// 切片表达式 s[low:high)：半开区间，和 Python 一致（但【不支持负索引和步长】）
	fmt.Println(sl[1:3], sl[:2], sl[2:])

	// 大坑：子切片与原切片【共享底层数组】！
	// （Python list 切片是拷贝出新列表，这里完全不同）
	sub := sl[1:3]
	sub[0] = 222
	fmt.Println("sub 改了, sl 跟着变:", sl[1])
	// 想要独立副本：copy 或 slices.Clone(Go1.21+)。copy(dst, src) 返回拷贝个数
	cp := make([]int, len(sl))
	copy(cp, sl)
	cp[0] = -1
	fmt.Println("copy 出来的是独立副本:", cp[0], sl[0])

	// 切片传参：拷贝的只是切片头(约24字节)，底层数组共享 → 函数内能改原数据
	modifySlice(sl)
	fmt.Println("函数内修改生效:", sl[0])

	// nil 切片合法：len/cap 为 0，可以直接 append（Python 的 None 可不能 append）
	var nilSl []int
	fmt.Println(nilSl == nil, len(nilSl))
	nilSl = append(nilSl, 1)
	fmt.Println(nilSl)

	// 二维切片：[][]int，逐层 make（对比 C 的 int a[3][4]，Go 外层是独立的多个切片）
	matrix := make([][]int, 2)
	for i := range matrix {
		matrix[i] = make([]int, 3)
	}
	matrix[1][2] = 9
	fmt.Println(matrix)

	// ==== 3. map：哈希表 ====
	// 类比：Python dict、C++ std::unordered_map；C 里只能手写哈希表
	m := map[string]int{
		"apple":  5,
		"banana": 3,
	}
	m["cherry"] = 8     // 新增或覆盖
	delete(m, "banana") // 内置删除；键不存在也不报错
	v := m["apple"]
	fmt.Println(m, v, len(m))

	// comma-ok：取不存在的键得到零值且不报错，
	// 但无法区分"值为 0"和"不存在"——第二个返回值登场（Python 用 in 判断）
	x, ok := m["banana"]
	fmt.Println(x, ok) // 0 false
	if price, exists := m["apple"]; exists {
		fmt.Println("apple 有价格:", price)
	}

	// 遍历顺序是【随机】的！语言规范故意的，强迫你不要依赖顺序。
	// 需要有序：把 key 收集成切片 → 排序 → 再遍历
	keys := make([]string, 0, len(m))
	for k := range m { // 只要 key 就写一个变量
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Printf("%s=%v ", k, m[k])
	}
	fmt.Println()

	// map 是引用语义（传参共享底层数据）；元素不可取地址(&m["a"] 编译错)，因为扩容会搬家

	// var 声明的 map 是 nil map：读可以、delete 可以、【写入会 panic】！必须先 make
	var m2 map[string]int
	fmt.Println(m2 == nil)
	m2 = make(map[string]int)
	m2["now"] = 1
	fmt.Println(m2)

	/*
	 * ---- 一句话总结 ----
	 * 数组是值、切片是视图；append 要接返回值；
	 * 子切片共享底层是最大陷阱；map 取值记得 comma-ok、写入前必须 make。
	 */
}
