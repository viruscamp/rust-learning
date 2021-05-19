本文针对函数式编程爱好者(他们想把所有的 `match Option<T>` 都用 `map` 写), 以及想搞懂这些代码的初学者.

## 建议
1. 不要返回值的话, 先试试 `if let`
2. `map_or`, `map_or_else` 两分支都有, 还是用 `match` 吧
3. `and`, `or`, `map_or` 要传值, 试试改成传闭包的 `map`, `or_else`, `map_or_else`, 懒取值大部分情况下都更好
4. 一般都用`map`, 用`and_then`的情况: 可能返回`None`, 调用返回值为`Option<_>`的函数

## 如何选择
- 下列函数均会消耗 `Option<T>`  
要保留的话, 先用 `as_ref`, `as_mut`, `as_deref`, `as_deref_mut`,

- 对应 match Some 部分有操作的, 要传闭包 `|t| {}`  
`map`, `map_or`, `map_or_else`, `and_then`
- 对应 match None 部分有操作的, 要传闭包 `|| {}`  
`map_or_else`, `or_else`
- 对应 match Some 部分直接返回值, 要传 `Option<T>`  
`and`
- 对应 match None 部分直接返回值  
`or`, `map_or`
- 这3个函数 会用 Some 包装返回值  
`map`, `map_or`, `map_or_else`

## 如何改写
以下表格适用于传闭包 `|t| {}` 的 `map`, `map_or`, `map_or_else`, `and_then`

| 我有 \\ 我要             | t: T                                    | t: &mut T                                                                 | t: &T                                                                      |
|-------------------------|-----------------------------------------|---------------------------------------------------------------------------|----------------------------------------------------------------------------|
| ot: Option<T>           | match ot { Some(t)<br><br>ot.map(\|t\| {})   | match { ot Some(ref mut t)<br><br>ot.as_mut().map(\|t\| {})                 | match { ot Some(ref t)<br><br>ot.as_ref().map(\|t\| {})                      |
| ot: &mut Option<T>      |                                         | match { ot Some(t)<br>match ot { Some(ref mut t)<br>ot.as_mut().map(\|t\| {}) | match { ot Some(ref t)<br><br>ot.as_ref().map(\|t\| {})                      |
| ot: &Option<T>          |                                         |                                                                           | match { ot Some(t)<br>match { ot Some(ref t)<br><br>ot.as_ref().map(\|t\| {}) |
|                         |                                         |                                                                           |                                                                            |
| ot: Option<Box<T>>      | ot.map(\|t\| {<br>    let t = *t;<br>}) | ot.as_deref_mut().map(\|t\| {})                                             | ot.as_deref().map(\|t\| {})                                                  |
| ot: &mut Option<Box<T>> |                                         | ot.as_deref_mut().map(\|t\| {})                                             | ot.as_deref().map(\|t\| {})                                                  |
| ot: &Option<Box<T>>     |                                         |                                                                           | ot.as_deref().map(\|t\| {})                                                  |
