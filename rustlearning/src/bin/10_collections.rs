// ============================================================
// 第10章 · 常用集合：Vec、String、HashMap
// ============================================================
// 运行: cargo run --bin 10_collections
// 学习要点:
//   1. Vec<T> 动态数组：增删改查与遍历
//   2. String UTF-8 字符串：拼接、切片的坑、chars()
//   3. HashMap<K,V>：插入、访问、entry 幂等更新
//   4. HashSet 去重
//   5. 集合与所有权的交互（存进去就被集合拥有）
//   6. BTreeMap 有序映射
// ============================================================

use std::collections::hash_map::Entry;
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    // ==== 1. Vec<T>：动态数组 ====
    // 对比 C 的 int arr[N]/malloc+realloc 手工扩容；Python 的 list（存的是对象引用）。
    // Rust 的 Vec 存值本身，扩容搬家用 move 语义，元素类型编译期确定。
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![10, 20, 30];                   // vec! 宏初始化，像 Python 的 [10,20,30]

    println!("v = {v:?}, len={}, v2={v2:?}", v.len());

    // 访问元素的两种方式：[] 索引越界会 panic；get 返回 Option 走安全路线
    let third = v2[2];                           // 直接下标，越界 panic（对比 C 数组越界是 UB）
    println!("v2[2] = {third}");
    match v2.get(99) {
        Some(x) => println!("取到 {x}"),
        None => println!("get(99) 返回 None——不崩溃"),
    }

    // 遍历时修改需要 &mut：借用规则保证不会边读边改
    for x in &mut v {
        *x *= 10;                                // 解引用后修改
    }
    println!("翻倍后 v = {v:?}");

    v.pop();                                     // 弹出尾部
    v.retain(|x| *x > 5);                        // 条件过滤保留
    println!("pop+retain 后 v = {v:?}");

    // 同一 Vec 存多种类型？用枚举（第6章）：
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Text(String),
    }
    let row = vec![Cell::Int(1), Cell::Text("混合".to_string()), Cell::Int(42)];
    println!("异构行: {row:?}");
    for cell in &row {                           // 用 match 取出异构数据
        match cell {
            Cell::Int(n) => print!("整型{n} "),
            Cell::Text(t) => print!("文本'{t}' "),
        }
    }
    println!();

    // ==== 2. String：UTF-8 且可增长 ====
    // 对比 C 的 char*（裸字节+手动长度）；Java String 不可变；Python str 不可变。
    // Rust String 可变（堆上缓冲区），字符串字面量 &str 是只读切片——两者关系第5章讲过。
    let mut s = String::from("你好");
    s.push_str(", Rust");
    s.push('!');                                 // push 单个 char
    println!("{s}");

    let joined = format!("{s} 第{}次相遇", 1);    // format! 拼接最顺手（像 Python f-string）
    println!("{joined}");
    let concat = "a".to_string() + "-" + "b";    // + 号走 Add，左值被消费（move）
    println!("{concat}");

    // UTF-8 的坑：不能按字节下标取「字符」（多字节编码会让索引语义混乱）
    println!("字节长度={} 字符数={}", "中文ab".len(), "中文ab".chars().count());
    for c in "中A文B".chars() {
        print!("{c} ");
    }
    println!();
    // 切片必须落在字符边界上，否则 panic：
    let hello = "helloworld";
    println!("切片 &hello[0..5] = {}", &hello[0..5]);

    // ==== 3. HashMap<K, V> ====
    // 对比 C++ unordered_map / Python dict / Java HashMap。Rust 标准库自带 SipHash 抗碰撞攻击。
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("小明"), 95);      // key 的所有权移入 map
    scores.insert(String::from("小红"), 88);

    // get 返回 Option<&V>：不存在不崩溃
    if let Some(sc) = scores.get("小明") {
        println!("小明的分数: {sc}");
    }
    // insert 同 key 会覆盖旧值并返回旧值（Option）
    let old = scores.insert("小明".to_string(), 100);
    println!("覆盖前旧值: {:?}", old);

    // entry + or_insert：「没有就建」的幂等写法，词频统计神器
    // 对比 Python: d[k] = d.get(k, 0) + 1 或 defaultdict(int)
    let text = "rust is fun and rust is fast";
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("词频: {freq:?}");

    // Entry API 更细的控制：vacant/occupied 两态
    let mut config: HashMap<String, String> = HashMap::new();
    match config.entry("theme".to_string()) {
        Entry::Vacant(slot) => {
            slot.insert("dark".to_string());
            println!("新建 theme");
        }
        Entry::Occupied(_) => println!("theme 已存在，跳过"),
    }

    // ==== 4. HashSet：去重与集合运算 ====
    let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<i32> = [3, 4, 5].into_iter().collect();
    let inter: HashSet<_> = a.intersection(&b).copied().collect();     // 交集
    let union: HashSet<_> = a.union(&b).copied().collect();            // 并集
    println!("交集={inter:?}, 并集大小={}", union.len());
    // 对比 Python set 的 & |；C 得自己写或用第三方库

    // ==== 5. BTreeMap：有序映射 ====
    let mut ordered = BTreeMap::new();           // 按 key 排序（对比 Python 的 dict 保插入序）
    ordered.insert(3, "c");
    ordered.insert(1, "a");
    ordered.insert(2, "b");
    println!("BTreeMap 自动排序: {ordered:?}");

    /*
     * ---- 一句话总结 ----
     * Vec/String/HashMap 覆盖 90% 场景，全部在堆上、离开作用域自动释放；
     * 下标会 panic，get 给 Option——按需选择；
     * 存入集合即移交所有权，集合销毁时元素跟着 drop，无需手动 free。
     */
}
