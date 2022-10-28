# Rust 中的泛型型变

Rust 没有 `struct`, `enum` 和 `union` 的继承, 而`trait`虽然有继承, 但`trait`不能当作类型使用.  

Rust 的子类型关系只出现在生存期上.
```rust
fn lifetime<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {}
```
`'long: 'short`定义`'long`是`'short`的子类型, 
意味着`'long`是一个较长的生存期, 它能完全覆盖`'short`这个较短的生存期, 
那么任何一个需要`&'short i32`的地方(赋值,参数)`&'long i32`都可以满足的, 
所以`&'long i32`是`&'short i32`的子类型.

静态生存期`&'static T`是任意生存期`&'x T`的子类型.

Rust 泛型类型的可变性不是由语法定义,而是固定的几个基础类型的可变性表,
然后组合类型 `struct`, `enum` 和 `union` 根据其包含域类型的可变性确定, 
域类型有多种可变性时, 组合类型为不变.

`Cell<T>` 包含 `std::cell::UnsafeCell<T>` 其对`T`不变.  
`Vec<T>` 包含 `alloc::raw_vec::RawVec<T>` 包含 `core::ptr::Unique<T>` 包含 `std::marker::PhantomData<T>` 其对`T`协变.  

| Type                          | Variance in `'a`  | Variance in `T`   |
|-------------------------------|-------------------|-------------------|
| `&'a T`                       | covariant         | covariant         |
| `&'a mut T`                   | covariant         | invariant         |
| `*const T`                    |                   | covariant         |
| `*mut T`                      |                   | invariant         |
| `[T]` and `[T; n]`            |                   | covariant         |
| `fn() -> T`                   |                   | covariant         |
| `fn(T) -> ()`                 |                   | contravariant     |
| `std::cell::UnsafeCell<T>`    |                   | invariant         |
| `std::marker::PhantomData<T>` |                   | covariant         |
| `dyn Trait<T> + 'a`           | covariant         | invariant         |

```rust
struct Node<T>(T);

type Link1<T> = Option<NonNull<Node<T>>>; // `NonNull` is `*const T` covariant for `Node<T>`
type Link2<T> = *mut Node<T>; // invariant for `Node<T>`
```
