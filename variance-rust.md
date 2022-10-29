# 代码写明 Rust 中的泛型型变

## 概要
`Variance`译作`型变`或`可变性`或`变体`.  

假设1 `C<T>`是一个泛型类或接口, `T`是类型参数.  
假设2 类型`Dog`是`Animal`的子类型.  

`Covariance/Covariant`译作`协变`: 如果`C<Dog>`是`C<Animal>`的子类型, 那么`C<T>`对于`T`协变.  
`Contravariance/Contravariant`译作`逆变`: 如果`C<Animal>`是`C<Dog>`的子类型, 那么`C<T>`对于`T`逆变.  
`Invariance/Invariant`译作`不变`: `C<T>`对于`T`既不是协变也不是逆变, 那么`C<T>`对于`T`不变, 或译作`抗变`.  

## 生存期子类型
Rust 没有 `struct`, `enum` 和 `union` 的继承,   
`impl trait`不是继承`trait`,   
`trait`之间虽然有继承, 但`trait`不能直接当作类型使用.  

Rust 的子类型关系只体现在生存期上. 
```rust,no_run
fn lifetime_subtype<'long: 'short, 'short, T: Copy>(a: &'short T, b: &'long T) {
    let _long_to_short: &'short T = b; // 成功 子类型的值可以转型为父类型
}
``` 
泛型参数`'long: 'short`定义`'long`是`'short`的子类型,   
意味着`'long`是一个较长的生存期, 它能完全覆盖`'short`这个较短的生存期,   
那么任何一个需要`&'short i32`的地方(转型,赋值,参数)`&'long i32`都可以满足的,   
所以`&'long i32`是`&'short i32`的子类型.  

父类型的值不可以转型为子类型:
```rust,no_run,compile_fail
fn lifetime_subtype<'long: 'short, 'short, T: Copy>(a: &'short T, b: &'long T) {
    let _short_to_long: &'long T = a; // 失败 父类型的值不可以转型为子类型
}
```

较复杂的代码:
```rust
fn lifetime_subtype<'long: 'short, 'short, T: Copy>(a: &'short mut T, b: &'long T) {
    *a = *b;
}
static I_STATIC: i32 = 1; // 其生存期为 'static
fn main() {
    let mut i_1 = 2; // 假设其自动推导生存期为 '1
    {
        let mut i_2 = 3; // 假设其自动推导生存期为 '2
        dbg!(I_STATIC, i_1, i_2);

        //lifetime_subtype(&mut i_1, &i_2); // 无法编译
        lifetime_subtype(&mut i_2, &i_1); // 子类型关系为 `'1: '2` 满足函数泛型条件 `'long: 'short`
        dbg!(i_2);
    }
    lifetime_subtype(&mut i_1, &I_STATIC); // 子类型关系为 `'static: '1`
    dbg!(I_STATIC, i_1);
}
```
以上代码说明:  
1. 许多类型和生存期参数是 rustc 自动推导的, 我们无法明确的写出
2. 自动推导出的生存期符合子类型关系
3. 静态生存期`&'static T`是任意生存期`&'x T`的子类型

## Rust 协变 逆变 不变

### `Vec<T>`对`T`协变
编译成功 证明 `Vec<&'long i32'>` 是 `Vec<&'short i32>` 的子类型
```rust,no_run
fn lifetime_covariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut vec_long: Vec<&'long i32> = vec![b];
    let mut vec_short: Vec<&'short i32> = vec![a];

    vec_short = vec_long; // 成功
}
```
编译失败 证明 `Vec<&'short i32'>` 不是 `Vec<&'long i32>` 的子类型
```rust,no_run,compile_fail
fn lifetime_covariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut vec_long: Vec<&'long i32> = vec![b];
    let mut vec_short: Vec<&'short i32> = vec![a];

    vec_long = vec_short; // 失败
}
```

### `Cell<T>`对`T`不变
编译失败 证明 `Cell<&'short i32'>` 不是 `Cell<&'long i32>` 的子类型
```no_run,compile_fail
use std::cell::Cell;
fn lifetime_invariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut cell_long: Cell<&'long i32> = Cell::new(b);
    let mut cell_short: Cell<&'short i32> = Cell::new(a);
    cell_short = cell_long;
}
```
编译失败 证明 `Cell<&'long i32'>` 不是 `Cell<&'short i32>` 的子类型
```no_run,compile_fail
use std::cell::Cell;
fn lifetime_invariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut cell_long: Cell<&'long i32> = Cell::new(b);
    let mut cell_short: Cell<&'short i32> = Cell::new(a);
    cell_long = cell_short;
}
```

### `Fn<A> -> ()`对`T`逆变
编译成功 证明 `Fn(&'a str) -> ()` 是 `Fn(&'static str) -> ()` 的子类型
```rust,no_run
fn lifetime_contravariant<'t>(argt: &'t str) {
    fn use_static(instr: &'static str) {} // 类型 `Fn(&'static str) -> ()`
    fn use_lifetime<'a>(instr: &'a str) {} // 类型 `Fn(&'a str) -> ()`

    let closure_t = |_| {};
    closure_t(argt); // 绑定 closure_t 和 argt, 让 rust 推断 closure_t 的类型是 `Fn(&'t str) -> ()`

    struct S<'z>(&'z str);
    impl<'z> S<'z> {
        fn use_fn<F: Fn(&'z str) -> ()>(&self, f: F) {
            f(self.0)
        }
    }

    let s: S<'static> = S("abc"); // s.use_fn 实际类型为 `fn use_fn(&self, f: Fn(&'static str) -> ()) -> ()`
    s.use_fn(use_static); // 当然可以用 `Fn(&'static str) -> ()` 做参数
    s.use_fn(use_lifetime); // 指定生存期参数的函数 `Fn(&'a str) -> ()` 也可以, 已知 `'static: 'a`, 这就是逆变
    s.use_fn(closure_t); // `Fn(&'t str) -> ()` 也可以, 当然有 `'static: 't`, 这也是逆变
}
```

## Rust 类型型变的推导

Rust 泛型类型型变不是由语法定义,而是固定的几个基础类型的可变性表,
然后组合类型 `struct`, `enum` 和 `union` 根据其包含域类型的可变性确定, 
域类型有多种可变性时, 组合类型为不变.

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

### 型变推导实例
1. `Cell<T>` 包含 `std::cell::UnsafeCell<T>` 其对`T`不变.  
2. `Vec<T>` 包含 `alloc::raw_vec::RawVec<T>` 包含 `core::ptr::Unique<T>` 包含 `std::marker::PhantomData<T>` 其对`T`协变.  
3. 推导以下代码中定义类型的型变
```rust, no_run
use core::ptr::NonNull;
struct Node<T>(T);

type Link1<T> = Option<NonNull<Node<T>>>; // `NonNull` 就是 `*const T`, 对 `Node<T>` 协变, 最终对 `T` 协变
type Link2<T> = *mut Node<T>; // 对 `Node<T>` 不变, 最终对 `T` 不变
```
