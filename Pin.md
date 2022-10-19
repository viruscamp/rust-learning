# 教程
- [Rust的Pin与Unpin](https://folyd.com/blog/rust-pin-unpin/)
- [Pin UnPin 学习笔记](https://rustcc.cn/article?id=1d0a46fa-da56-40ae-bb4e-fe1b85f68751)

# 笔记
0. 自引用结构体  
```rust
struct SelfRef {
    a: i32,
    b: *const i32, // 指向 a
}

fn new_self_ref(a: i32) -> SelfRef {
    let mut sr = SelfRef {
        a: a,
        b: std::ptr::null(),
    };
    sr.b = &sr.a;
    // 这里 sr 状态是对的
    return sr;
}

let sr2 = new_self_ref(3);
// sr2 状态已经不对了
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
7. `Unpin` 无需pin住  
	绝大部分类型都是 `Unpin` 自动实现
8. `!Unpin` 必须pin住  
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
- ## Pin 到 堆上 `Pin<Box<T>>`
```rust
    struct T(i32, std::marker::PhantomPinned);
    let mut t: T = T(3, std::marker::PhantomPinned);
    let mut bt: Box<T> = Box::new(t);
    
    // 1. 试图在 Pin 前偷一个 &mut T 失败
    //let rt: &mut T = bt.borrow_mut();

    // 创建 !Unpin 的 Pin 是 unsafe
    let mut pbt: core::pin::Pin<Box<T>> = unsafe { core::pin::Pin::new_unchecked(bt) };

    // 2. T: Unpin 时才有 get_mut 失败
    //let t1 = pbt.get_mut(); 

    // 3. 不是 &mut T 失败
    let t2 = pbt.borrow_mut();

    // 4. 拿不到 Pin 的 Box, T 也拿不到 失败
    //let t4: T = Box::into_inner(pbt.what());
```

- ## Pin 到 堆上 `Pin<Arc<T>>`
```rust
    let mut at = std::sync::Arc::new(T(3, std::marker::PhantomPinned));
    let mut at1 = at.clone();
    let mut at = unsafe { core::pin::Pin::new_unchecked(at) };
    
    // 5. Arc 没有 as_mut 不能泄漏 &mut T 失败
    //let t3 = at1.as_mut();

    // 6. 因为有 Pin 的 Arc 存在 失败
    let t4 = std::sync::Arc::try_unwrap(at1);
```

- ## Pin 到栈上 `Pin<&mut T>` 与反例
```rust
    let mut t: T = T(3, std::marker::PhantomPinned);
    let mut t2: T = T(4, std::marker::PhantomPinned);

    let pbt: core::pin::Pin<_> = unsafe { core::pin::Pin::new_unchecked(&mut t) };

    // let t1 = pbt.get_mut(); // 虽然 Pin 这拿不到 &mut T
    std::mem::swap(&mut t, &mut t2); // 反例 但之前保留的变量可用
```




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
