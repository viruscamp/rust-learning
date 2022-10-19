//! [7.2. Variance and PhantomData](https://rust-unofficial.github.io/too-many-lists/sixth-variance.html)
use std::cell::Cell;

/// Incorrect
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

/// 对 T 这是不变的
type Link<T> = *mut Node<T>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T, 
}

/// for type `F<T>`
/// 1. 协变 F is covariant if F<Sub> is a subtype of F<Super> (subtyping "passes through")
/// 2. 逆变 F is contravariant if F<Super> is a subtype of F<Sub> (subtyping is "inverted")
/// 3. 不变 F is invariant otherwise (no subtyping relationship exists)

/// C# kotlin 定义类型时用 in out 标记可变性
/// Java 使用类型时用 <? extends Super> <? super Sub> 标记可变性
/// rust 自动推导可变性, 有一可变性表, struct enum union 的可变性根据其包含 field 类型推出
/// Vec<T> 包含 alloc::raw_vec::RawVec<T> 包含 core::ptr::Unique<T> 包含 std::marker::PhantomData<T> 其为协变
/// Cell<T> 包含 std::cell::UnsafeCell<T> 其为不变

/// 1. 生存期 'big 包括 'small , `&'big u32` 是 `&'small u32` 的子类型
/// ```
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_u32<'big: 'small, 'small>(
///     big: &'big u32, 
///     small: &'small u32,
/// ) {
///     // 其实是 take_two<&'small u32>(big, small); 子类型用于父类型
///     take_two(big, small);
/// }
/// ```
/// 2. failed
/// ```compile_fail
/// use std::cell::Cell;
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_cell<'big: 'small, 'small>(
///     // NOTE: these two lines changed
///     big: Cell<&'big u32>, 
///     small: Cell<&'small u32>,
/// ) {
///     // 应该写作 take_two<Cell<&'small u32>>(big, small);
///     // Cell 不支持协变, 类型`Cell<&'big u32>`无法转为`Cell<&'small u32>`
///     take_two(big, small);
/// }
/// ```
/// 3. passed
/// ```
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_vec<'big: 'small, 'small>(
///     big: Vec<&'big u32>, 
///     small: Vec<&'small u32>,
/// ) {
///     // 其实是 take_two<Vec<&'small u32>>(big, small);
///     // Vec 支持协变, 类型`Vec<&'big u32>`可以转为`Vec<&'small u32>`
///     take_two(big, small);
/// }
/// ```
#[cfg(doctest)]
fn take_two<T>(_val1: T, _val2: T) {}
