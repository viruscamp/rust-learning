# 代码写明 Rust 中的泛型型变
`Variance`译作`型变`或`可变性`或`变体`.  
表示"泛型类型的子类型关系"如何从"泛型参数的子类型关系"中推导.

## 定义
无法理解时, 不要深究, 看完后再读一遍此定义即可.

- 假设1 `C<T>`是一个泛型类或接口, `T`是类型参数.  
- 假设2 类型`Dog`是`Animal`的子类型.  

- 定义1 `Covariance/Covariant`译作`协变`:    
    如果`C<Dog>`是`C<Animal>`的子类型, 那么`C<T>`对`T`协变.  
- 定义2 `Contravariance/Contravariant`译作`逆变`:    
    如果`C<Animal>`是`C<Dog>`的子类型, 那么`C<T>`对`T`逆变.  
- 定义3 `Invariance/Invariant`译作`不变`, 或译作`抗变`:    
    `C<T>`对`T`既不是协变也不是逆变, 那么`C<T>`对`T`不变.  

## 子类型的证明
```rust, ignore
let mut t1: T1 = make_t1();
let t2: T2 = make_t2();

// 在排除 type coerced 类型强制转换后, 以下证明有效

// 1. 赋值证明
let _t1: T1 = t2; // 证明`T2` 是 `T1` 的子类型
t1 = t2; // 可赋值, 证明`T2` 是 `T1` 的子类型

// 2. 函数调用证明
fn use_t1(v: T1) {}
use_t1(t2); // 实参类型 `T2` 可以替代形参类型 `T1`, 证明 `T2` 是 `T1` 的子类型
```
函数调用证明的适用范围更广:  
- 许多类型写不出, 比如闭包
- 自动推导的生存期写不出
- 许多泛型参数是调用处才能确定类型

必须排除 type coerced 类型强制转换:  
```rust, no_run
let string1: String = String::from("abc");
let str1: &str = &string1; // 不能证明 `&String` 是 `&str` 的子类型
```

## 生存期子类型
Rust 没有实际类型 `struct`, `enum` 和 `union` 的继承,    
子类型关系只体现在生存期上, 可以通过赋值来证明.    

子类型的值可以转型为父类型:
```rust, no_run
fn lifetime_subtype<'long: 'short, 'short, T: Copy>(a: &'short T, b: &'long T) {
    let _long_to_short: &'short T = b; // 成功 子类型的值可以转型为父类型
}
``` 
泛型参数`'long: 'short`定义`'long`是`'short`的子类型,   
意味着`'long`是一个较长的生存期, 它能完全覆盖`'short`这个较短的生存期,   
那么任何一个需要`&'short i32`的地方(转型,赋值,参数)`&'long i32`都可以满足的,   
所以`&'long i32`是`&'short i32`的子类型.  

父类型的值不可以转型为子类型:
```rust, no_run, compile_fail
fn lifetime_subtype<'long: 'short, 'short, T: Copy>(a: &'short T, b: &'long T) {
    let _short_to_long: &'long T = a; // 失败 父类型的值不可以转型为子类型
}
```

较复杂的代码:
```rust, no_run
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
```rust, no_run
fn lifetime_covariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    //! `'long` 是 `'short` 的子类型, `&'long i32` 是 `&'short i32` 的子类型
    let mut vec_long: Vec<&'long i32> = vec![b];
    let mut vec_short: Vec<&'short i32> = vec![a];
    vec_short = vec_long; // 成功
}
```
编译失败 证明 `Vec<&'short i32'>` 不是 `Vec<&'long i32>` 的子类型
```rust, no_run, compile_fail
fn lifetime_covariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut vec_long: Vec<&'long i32> = vec![b];
    let mut vec_short: Vec<&'short i32> = vec![a];
    vec_long = vec_short; // 失败
}
```

### `Cell<T>`对`T`不变
编译失败 证明 `Cell<&'short i32'>` 不是 `Cell<&'long i32>` 的子类型
```rust, no_run, compile_fail
use std::cell::Cell;
fn lifetime_invariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut cell_long: Cell<&'long i32> = Cell::new(b);
    let mut cell_short: Cell<&'short i32> = Cell::new(a);
    cell_short = cell_long; // 失败
}
```
编译失败 证明 `Cell<&'long i32'>` 不是 `Cell<&'short i32>` 的子类型
```rust, no_run, compile_fail
use std::cell::Cell;
fn lifetime_invariant<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
    let mut cell_long: Cell<&'long i32> = Cell::new(b);
    let mut cell_short: Cell<&'short i32> = Cell::new(a);
    cell_long = cell_short; // 失败
}
```

### `Fn<A> -> R`对`A`逆变
编译成功 证明 `Fn(&'a str) -> bool` 是 `Fn(&'static str) -> bool` 的子类型
```rust, no_run
fn lifetime_fn_contravariant<'outer>(str_outer: &'outer str) {
    let str_static: &'static str = "static";

    fn compare_with_static(instr: &'static str) -> bool {
        instr == "abc"
    } // 类型 `Fn(&'static str) -> bool`

    fn make_compare_closure<'x>(a: &'x str) -> impl Fn(&'x str) -> bool {
        return move |instr: &'x str| { instr == a }
    } // 返回值类型 `Fn(&'x str) -> bool`

    struct S<'z>(&'z str);
    impl<'z> S<'z> {
        fn do_compare<F: Fn(&'z str) -> bool>(&self, f: F) -> bool {
            f(self.0)
        }
    }

    let s_static: S<'static> = S("xyz"); // `s_static.do_compare` 参数类型为 `Fn(&'static str) -> bool`
    s_static.do_compare(compare_with_static); // 类型相符, 当然可以用 `Fn(&'static str) -> bool` 做参数
    s_static.do_compare(make_compare_closure(str_static)); // 类型相符
    s_static.do_compare(make_compare_closure(str_outer)); // 逆变, 实参类型为 `Fn(&'outer str) -> bool`

    let s_outer: S<'outer> = S(str_outer); // `s_outer.do_compare` 参数类型为 `Fn(&'outer str) -> bool`
    //s_outer.do_compare(compare_with_static); // 协变失败
    //s_outer.do_compare(make_compare_closure(str_static)); // 协变失败
    s_outer.do_compare(make_compare_closure(str_outer)); // 类型相符

    {
        let string_inner = String::from("inner"); // 命名其生存期为 'inner
        let str_inner: &str = string_inner.as_str();

        s_static.do_compare(make_compare_closure(str_inner)); // 逆变, 实参`Fn(&'inner str) -> bool` 替代形参 `Fn(&'static str) -> bool`
        s_outer.do_compare(make_compare_closure(str_inner));  // 逆变, 实参`Fn(&'inner str) -> bool` 替代形参 `Fn(&'outer str) -> bool`
    }

    // 强制拉长生存期
    s_outer;
    s_static;
    str_outer;
}
```

### `Fn<A> -> R`对`R`协变
编译成功 证明 `Fn() -> &'static str` 是 `Fn() -> &'a str` 的子类型
```rust, no_run
fn lifetime_fn_covariant<'outer>(str_outer: &'outer str) {
    let str_static: &'static str = "static";

    fn return_static() -> &'static str {
        "abc"
    } // 类型 `Fn() -> &'static str`

    fn make_return_closure<'x>(a: &'x str) -> impl Fn() -> &'x str {
        return move || { a }
    } // 返回值类型 `Fn() -> &'x str`

    struct S<'z>(&'z str);
    impl<'z> S<'z> {
        fn set_with<F: Fn() -> &'z str>(&mut self, f: F) -> () {
            self.0 = f();
        }
    }

    let mut s_static: S<'static> = S("xyz"); // `s_static.set_with` 参数类型为 `Fn() -> &'static str`
    s_static.set_with(return_static); // 类型相符, 当然可以用 `Fn() -> &'static str` 做参数
    s_static.set_with(make_return_closure(str_static)); // 类型相符
    //s_static.set_with(make_return_closure(str_outer)); // 逆变失败

    let mut s_outer: S<'outer> = S(str_outer); // `s_outer.set_with` 参数类型为 `Fn() -> &'outer str`
    //s_outer.set_with(return_static); // 理论可以协变, 实际会导致 `s_outer` 类型推断成 `S<'static>`, 然后编译失败, 无法达到目的
    s_outer.set_with(make_return_closure(str_static)); // 协变, 实参`Fn() -> &'static str` 替代形参 `Fn() -> &'outer str`
    s_outer.set_with(make_return_closure(str_outer)); // 类型相符

    {
        let string_inner = String::from("inner"); // 命名其生存期为 'inner
        let str_inner: &str = string_inner.as_str();

        let mut s_inner: S = S(str_inner); // `s_inner.set_with` 参数类型为 `Fn() -> &'inner str`
        //s_inner.set_with(return_static); // 理论可以协变, 实际会导致 `s_inner` 类型推断成 `S<'static>`, 然后编译失败, 无法达到目的
        s_inner.set_with(make_return_closure(str_static)); // 协变, 实参`Fn() -> &'static str` 替代形参 `Fn() -> &'inner str`
        s_inner.set_with(make_return_closure(str_outer)); // 协变, 实参`Fn() -> &'outer str` 替代形参 `Fn() -> &'inner str`
        s_inner.set_with(make_return_closure(str_inner)); // 类型相符
    }

    // 强制拉长生存期
    s_outer;
    s_static;
    str_outer;
}
```

## Rust 泛型类型型变的推导

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
3. 推导以下代码中泛型类型的型变
```rust, no_run
use core::ptr::NonNull;
struct Node<T>(T);

type Link1<T> = Option<NonNull<Node<T>>>; // `NonNull` 就是 `*const T`, 对 `Node<T>` 协变, 最终对 `T` 协变
type Link2<T> = *mut Node<T>; // 对 `Node<T>` 不变, 最终对 `T` 不变
```
