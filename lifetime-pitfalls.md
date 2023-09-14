# 生存期的坑

有助于展示生存期有关语法糖的配置:
`.vscode/settings.json`
```json
{
    "rust-analyzer.inlayHints.expressionAdjustmentHints.enable": "always", // 开启重借用提示
    "rust-analyzer.inlayHints.lifetimeElisionHints.enable": "always", // 函数生存期省略展开
}
```

## 常量提升
当你觉得某个引用超出了来源的生存期, 但可以编译通过时, 看看是不是常量提升.  
[临时引用的静态生命周期提升](https://zjp-cn.github.io/rust-note/forum/static-promotion.html)

```rust
struct X(i32);
fn static_i32_ref() -> &'static i32 {
    let X(ref r) = X(4); // 没有创建栈上的 X
    return r;
}
```
相当于
```rust
struct X(i32);
const CONST_X: X = X(4);
fn static_i32_ref() -> &'static i32 {
    let X(ref r) = CONST_X;
    return r;
}
```

不能常量提升的例子
```rust
struct X(i32);

let v = 4;
let X(ref r) = X(v); // 创建了一个栈上变量，然后引用

let X(ref mut r) = X(4); // 无法提升, 只能是栈上变量
```

专门坑人的例子
```rust
struct X(i32);
fn static_i32_ref() -> &'static i32 {
    const v: i32 = 4;
    // 三千行以后
    let X(ref r) = X(v); // v 是 const 所以还是常量提升
    return r;
}
```

## 临时变量生存期延长
> The drop scope of the temporary is usually the end of the enclosing statement.  
> 临时变量的 drop 时机通常在其所在语句(statement)的结尾.  

所有的链式调用都要注意, 中间的临时变量生存期是否过长.    
临时变量生存期过短编译器会报错.  
临时变量生存期过长就很麻烦了, 运行时死锁(`Mutex.lock()`), 运行时panic(`RefCell.borrow()`), 都可能发生.  

[The Reference - 表达式 - 临时变量](https://rustwiki.org/zh-CN/reference/expressions.html#%E4%B8%B4%E6%97%B6%E4%BD%8D%E7%BD%AE%E4%B8%B4%E6%97%B6%E5%8F%98%E9%87%8F)  
[The Reference - Expressions - Temporaries](https://doc.rust-lang.org/stable/reference/expressions.html#temporaries)  

The book 的例子，没有任何报错， 而是多线程退化为单线程, 性能变差.  
[The Book - 将单线程服务器变为多线程服务器](https://rustwiki.org/zh-CN/book/ch20-02-multithreaded.html)  
[The Book - Turning Our Single-Threaded Server into a Multithreaded Server](https://doc.rust-lang.org/stable/book/ch20-02-multithreaded.html)  

```rust
// 3, 2 然后死锁
fn dead_lock_1() {
    let vec_mutex = Mutex::new(vec![1,2,3]);
    // 会导致临时变量 MutexGuard 锁的有效期包括 while 循环体导致死锁
    while let Some(num) = vec_mutex.lock().unwrap().pop() {
        if num == 1 {
            vec_mutex.lock().unwrap().push(4);
        }
        println!("got {}", num);
    }
    // 临时变量不是在整个 while 结尾 drop 的, 否则只有 3, 2 都出不来
}

fn dead_lock_2() {
    let vec_mutex = Mutex::new(vec![1,2,3]);
    // 新增的 { ... } 是 block expression 不是 statement , 不能解决问题
    while let Some(num) = { vec_mutex.lock().unwrap().pop() } {
        if num == 1 {
            vec_mutex.lock().unwrap().push(4);
        }
        println!("got {}", num);
    }
}

fn no_dead_lock() {
    let vec_mutex = Mutex::new(vec![1,2,3]);
    while let Some(num) = {
        let n = vec_mutex.lock().unwrap().pop();
        // 前一个 ; 分号是临时变量所在语句的结尾, 临时变量在此处 drop
        n
    } {
        if num == 1 {
            vec_mutex.lock().unwrap().push(4);
        }
        println!("got {}", num);
    }
}
```

## 重借用 reborrow
[`&mut T` 多次使用(reborrow)的疑问](https://rustcc.cn/article?id=28fedcbc-d0c9-41e1-8d95-de73a578a078)  
[better documentation of reborrowing#788](https://github.com/rust-lang/reference/issues/788#issuecomment-1420528041)  
[Parameter coercion in Rust](http://smallcultfollowing.com/babysteps/blog/2013/11/20/parameter-coercion-in-rust/#reborrowing)  
[Stacked Borrows](https://rust-unofficial.github.io/too-many-lists/fifth-stacked-borrows.html)  

大部分情况自动重借用都可行, 少数情况必须显式重借用
```rust
struct X;
impl From<&mut i32> for X {
    fn from(i: &mut i32) -> Self {
        X
    }
}
let mut i = 4;
let r = &mut i;

fn from_auto_reborrow<'a, F, T: From<&'a mut F>>(f: &'a mut F) -> T {
    T::from(f)
}
let x: X = from_auto_reborrow(r); // 可以自动重借用
let x: X = from_auto_reborrow(r); // 可以自动重借用

fn from<F, T: From<F>>(f: F) -> T {
    T::from(f)
}
let x: X = from(&mut *r); // 必须显式重借用, 创建 x 的 reborrow 不会 move x
let x: X = from(r); // 此处不会自动重借用, 导致 move x
let x: X = from(r); // 第二次调用失败， 注释此句可编译
```
