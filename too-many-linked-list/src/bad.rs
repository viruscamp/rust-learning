//! [2.1. Layout](http://rust-unofficial.github.io/too-many-lists/first-layout.html)
//! 不好的数据结构

/// ```compile_fail
/// // 编译不过
/// pub enum List {
///     Empty,
///     Elem(i32, List),
/// }
/// ```

/// 1. 最后一个 node 无用
/// 2. 第一个 node 在栈上
#[derive(Debug)]
pub enum BadList1<T> {
    Empty,
    Elem(T, Box<BadList1<T>>),
}

/// 1. 最后一个 是 ElemThenEmpty(T) 有元素
/// 2. 第一个 node 还是在栈上
/// 3. 太复杂了
#[derive(Debug)]
pub enum BadList2<T> {
    Empty,
    ElemThenEmpty(T),
    ElemThenNotEmpty(T, Box<BadList2<T>>),
}

/// 导出 List3 隐藏 Node3 时有编译错误
/// private type `Node3<T>` in public interface. can't leak private type
#[derive(Debug)]
pub enum List3<T> {
    Empty,
    More(Box<Node3<T>>),
}
#[derive(Debug)]
pub struct Node3<T> {
    elem: T,
    next: List3<T>,
}
