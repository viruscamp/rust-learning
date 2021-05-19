# 单元测试与集成测试
[Rust 怎么写测试](https://lenshood.github.io/2020/03/14/rust-test/)

# assert 等宏用法
```rust
assert!(false); // bool 型
assert_eq!(s.peek(), s.peek()); // 等于 要实现 PartialEq
assert_ne!(s.peek(), None); // 不等 要实现 PartialEq
assert_eq!(s.peek(), s.peek(), "twice peek should get same result"); // 可以加上报错显示的额外信息

assert_eq!(&4, 4); // 小心借用类型 no implementation for `&{integer} == {integer}`
assert_eq!(s.iter().collect::<Vec<&i32>>(), [&0i32;0]); // 带类型的空数组
```

# 文档测试(doctest)
## 通用写法
```rust
    /// Create a new empty LinkedStack.
    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&0i32;0]);
    /// ```
    pub fn new() -> Self;
```
## 特殊用法 doctest Attributes
1. should_panic 常用
```rust
/// ```should_panic
/// assert!(false);
/// ```
```
对应单元测试函数的 `#[should_panic]`  
`#[should_panic(expected = "{error message}")]` 暂时无法在 doctest 内表达

2. compile_fail 常用
```rust
/// ```compile_fail
/// let x = 5;
/// x += 2; // shouldn't compile!
/// ```
```
3. 不常用
    - ignore 不编译 对应单元测试函数的 `#[ignore]`
    - no_run 不运行 有编译错误会报错
    - edition2018 edition2015 选择特定版本语法
4. 多个同时使用
```rust
/// ```should_panic, ignore
/// assert!(true);
/// ```
pub fn x() {}
```

## 使用 ? 语法返回 `Result<_>` 与隐藏代码
要用 ? 语法, 必须追加返回值  
返回语句可以用 `# ` 在文档中隐藏  
`# ` 常用于隐藏不重要的语句以突出重点  
```rust
/// ```
/// use std::io;
/// let mut input = String::new();
/// io::stdin().read_line(&mut input)?;
/// # Ok::<(), io::Error>(())
/// ```
```

## 测试命名
同一类型的不同 impl 内的同名函数，目前只能靠行号区分
```
test src\mod.rs - Struct1::fmt (line 145) ... ok
test src\mod.rs - Struct1::fmt (line 126) ... ok
```
函数 test 和 doctest 的自定义命名方法均未知

## 测试外部文档内的代码 与 仅在 doctest 有效的代码
```rust
#![feature(external_doc)]

#[doc(include = "../README.md")] // 引入外部文档, 测试其代码
#[cfg(doctest)]
pub struct ReadmeDoctests; // 仅在 doctest 有效的代码
```
