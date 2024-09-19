# 教程
- [Rust的Pin与Unpin](https://folyd.com/blog/rust-pin-unpin/)
- [Pin UnPin 学习笔记](https://rustcc.cn/article?id=1d0a46fa-da56-40ae-bb4e-fe1b85f68751)

# 类型
- `T: Unpin` 无须钉住
- `T: !Unpin` 必须钉住
    * 如果其公开API可以直接创建, 那么一定可以 move 就违反了语义
    * 一般其构造函数内部使用`new_unchecked`返回 `Pin<&mut T>`
- `struct Pin`, 不要写 `Pin<T: !Unpin>` 一般条件是 `Pin<P<T: !Unpin>> where P: Deref<Target=T>`
    * 对 `T: Unpin` 类型是一个普通的指针型包装类  
    构造 `fn new(t: T) -> Pin<T>`  
    解构 `fn into_inner(pin: Pin<T>) -> T`  
    可变引用 `impl<T> DerefMut for Pin<T>`  
    * 对 `T: !Unpin` 限制极大, 无法的 safe 得到 `T`, `&mut T`  
    构造 `unsafe fn new_unchecked(pointer: P) -> Pin<P>`  
    解构 `unsafe fn into_inner_unchecked(pin: Pin<P>) -> P`  
    可变引用 `unsafe fn get_unchecked_mut(self) -> &'a mut T`  

# 笔记
0. 自引用结构体  
```rust
// 如何表达对此类型的额外约束条件: &a as *const _ = b
struct SelfRef {
    a: i32,
    b: *const i32, // 指向 a
}

let mut sr = SelfRef {
    a: 4,
    b: std::ptr::null(),
}; // 此处非法
sr.b = &sr.a; // 此处才合法

assert_eq!(&sr.a as *const i32, sr.b); // ok

let sr2 = sr; // move 或 copy 导致的位置变动
assert_eq!(&sr2.a as *const i32, sr2.b); // fail
```
1. 异步函数闭包 基本都会用自引用结构体  
    大部分由 rust compiler 生成
2. 自引用结构体 在内存中移动会出错  
    * move `let t1 = SelfRef::new();`
	* swap `mem::swap(&mut t, &mut t2);`
	* replace `let t1 = mem::replace(&mut t, &t2);`
3. 一个指针比如 `Box<T>` 能拿到 `&mut T` 或 `T` 就可以做  
```rust
    struct T(i32);
    let mut t: T = T(3);
    let rmt: &mut T = &mut t; // 拿到 &mut T

    let mut bt: Box<T> = Box::new(t);
    let rmbt: &mut T = bt.as_mut(); // 从 Box 拿到 &mut T

    //let mut t: T = *bt; // 从 Box 拿到 T
    let mut t1 = Box::into_inner(bt); // 同上一行 更清晰 unstable

    std::mem::swap(rmbt, &mut t1); // 可以 swap
```
4. `Pin` 是指针 一般用于包裹另一指针 `Pin<P: Deref<T>>`  
    `Pin<P<T>> where T:Unpin` 可以拿到 `&mut T`  
    `Pin<P<T>> where T:!Unpin` 保证拿不到 `&mut T` 和 `T`  
5. `Pin` 指针才有意义  
    比如 `Pin<&mut T>` `Pin<Box<T>>` `Pin<Arc<T>>` 等
6. `Pin<P<T: Unpin>>` 与 `P<T>` 内存和使用都无区别, 仅用于满足类型要求
7. `T: Unpin` 无须pin住  
	绝大部分类型都是 `Unpin` 自动实现
8. `T: !Unpin` 必须pin住  
9. 实现 `!Unpin`
    - 默认 `!Unpin`
        - `PhantomPinned`, 给下面的 struct 包含用
        - async 块, 编译器生成
    - 手动实现 `impl !Unpin for T {}`, 需要 unstable
    - struct/enum 内包括 `!Unpin` 就是 `!Unpin`, 最常用
10. 实际用途是 `async`
    - 编译器生成的 async 块是`Future`类型的状态机
	- 每个`.await`对应一个状态, 要保存当前栈上变量
	- 有 `let z1=1; let z2=&z1;` 时自引用就出现了
	- 非运行状态的`Future`(比如等待`.await`时)是会被传递的, 比如在多线程 Executor 变换执行线程

# 代码实例
- ## Pin 住堆上值 `Pin<Box<T>>`
```rust
    struct T(i32, std::marker::PhantomPinned);
    let t: T = T(3, std::marker::PhantomPinned);
    let mut bt: Box<T> = Box::new(t);
    
    // 1. 试图在 Pin 前偷一个 &mut T, 在 pin 后使用
    // 在 pin 时失败: cannot move out of `bt` because it is borrowed
    //let rt: &mut T = bt.borrow_mut();

    // 创建 !Unpin 的 Pin 是 unsafe
    // 从 1.63 开始有 safe 的做法了
    let mut pbt = Box::into_pin(bt);

    // 1. 试图在 Pin 前偷一个 &mut T, 在 pin 后使用
    // rt;

    // 2. T: Unpin 时才有 get_mut 失败
    //let t1 = pbt.get_mut(); 

    // 3. 不是 &mut T 失败
    //let t2: &mut T = pbt.borrow_mut();

    // 4. 拿不到 Pin 的 Box, T 也拿不到 失败
    //let t4: T = Box::into_inner(pbt.what());
```

- ## Pin 住堆上值 `Pin<Arc<T>>`
```rust
    // safe 的做法
    let mut pat: Pin<Arc<T>> = std::sync::Arc::pin(T(3, std::marker::PhantomPinned));

    // 从已有 Arc<T> 创建 Pin<Arc<T>> 的 unsafe 做法
    let mut at = std::sync::Arc::new(T(3, std::marker::PhantomPinned));
    let mut at1 = at.clone();
    let mut pat2 = unsafe { core::pin::Pin::new_unchecked(at) };

    // 1. Arc 没有 as_mut 不能泄漏 &mut T 失败
    //let t3 = at1.as_mut();

    // 2. 因为有 Pin 的 Arc 存在 失败: 得到 None
    let t2 = at1.get_mut();

    // 3. 因为有 Pin 的 Arc 存在 失败: 得到 Err(_)
    let t3 = std::sync::Arc::try_unwrap(at1);

    drop(pat2);
```

- ## Pin 住栈上值 `Pin<&mut T>`
```rust
    // 从 1.68 开始有
    let mut pt = pin!(T(3, std::marker::PhantomPinned));

    let mut t: T = T(3, std::marker::PhantomPinned);

    // 以前的 unsafe 做法
    let pbt: core::pin::Pin<_> = unsafe { core::pin::Pin::new_unchecked(&mut t) };

    // let t1 = pbt.get_mut(); // 不到 &mut T
    
    drop(pbt); // 延长
```

# unsafe
为什么 `core::pin::Pin::new_unchecked()` 是 unsafe 的.  
从前面代码可看出 `Pin<Ptr<T>>` 存活期间, 无法通过 safe 代码获取 `&mut T`.  
```rust
    let mut t: T = T(3, std::marker::PhantomPinned);
    let pbt: core::pin::Pin<_> = unsafe { core::pin::Pin::new_unchecked(&mut t) };
    drop(pbt);

    t.0 = 4; // 此处可以修改 T, 可能做出非法修改

    let pbt2: core::pin::Pin<_> = unsafe { core::pin::Pin::new_unchecked(&mut t) }; // 新的 Pin 状态可能非法
```
rust 认为安全的创建 `Pin<Ptr<T>>` 的方法, 必须在之后就无法泄漏 t.
```rust
// move 变量到 Pin 内
let mut pbt: Pin<Box<T>> = std::boxed::Box::pin(t);
let mut pat: Pin<Arc<T>> = std::sync::Arc::pin(t);

// 栈上临时变量, 通过临时变量生存期延迟, 绑定生存期到 pt
let mut pt = pin!(T(3, std::marker::PhantomPinned));
```

# `!Unpin` 对象的一般需求
假设有 `T: !Unpin` 和 `Ptr<T>: DerefMut<T>` 
1. `Pin<Ptr<T>` 一定处于合法状态
2. 提供对外的安全修改方法 `fn mutate(self: Pin<&mut Self>)`  
    内部使用 unsafe 修改
3. 外部不能显式构造  
    显式构造意味着所有域都是 pub 的, 那么就可以创建, 修改到非法状态, pin
4. 如果提供 `fn new() -> Self` 形式的构造函数
4.1 初始状态可以安全 move
4.2 初始状态下任意 pub 域修改后不影响安全 move
4.4 初始状态下pub 的修改方法(`fn mutate1(&mut Self)`)调用后不影响安全 move
5. 可以提供 `fn new() -> Pin<Box<Self>>` 形式的构造函数

# 泛型约束
Unpin 可以用于泛型约束，但是 !Unpin 不行。当需要区分时:
```rust
#![feature(specialization)]
default impl<T: T1> T2 for T {}
impl<T: T1 + Unpin> T2 for T {}
```

`Pin<P:Deref<T>>` 常用法
	对 T: Unpin 可以解构出 T 可以取出 &mut T
	对 T: !Unpin 不能解构 只能取出 &T

`Pin<P:Deref<T>>` 当 P 是 Unpin 时，也是 Unpin 的，跟 T 是否 Unpin 没有关系。rust内置绝大部分指针类型都是 Unpin的，一般我们认为 Pin<P> 都是 Unpin

`Pin<P>` 跟 `P` 的内存布局完全无区别, `let p2 = p1; let pin2 = Pin::new(p1);` 两句运行时无区别
