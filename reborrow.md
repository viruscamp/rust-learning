# 重借用 reborrow

我碰到的问题是[^1], 目前文档不够[^2], 只有一篇开发人员的blog[^3], 在 The Book 和 The Reference 内基本没提。

## 问题
要是没有自动重借用，下面的简单代码都会失败
```rust
fn set_pos(p: &mut Pos) {
    p.set_x(3); // `&mut T` 不是 `Copy`, 所以应该已经消耗了 p
    p.set_y(4); // 此处 p 为何有效?
}
struct Pos(i32, i32);
impl Pos {
    fn set_x(&mut self, x: i32) { self.0 = x; }
    fn set_y(&mut self, y: i32) { self.1 = y; }
}
```

## 定义
解引用后再次引用, 生成一个类型相同，但生存期相同或更短的借用。  
在重借用的变量存活期间，被重借用的变量存在但无效。  
在重借用的变量 leave scope 后, 被重借用的变量再次有效。  
```rust
fn reborrow(r: &i32, rm: &mut i32) {
    // 显式重借用
    let r1 = &*r;
    let rm1 = &mut *rm;

    // 强转重借用, 不写类型就是 move
    let r2: &i32 = r1;
    let rm2: &mut i32 = rm;

    // 可变借用重借用为不可变
    let rm3 = &*rm;
}
```
对于不可变借用如 `&i32` 通常不关心重借用, 因为有 `impl<T> Copy for &T`.  

## 借用栈 borrow stack
对于可变借用, 可以使用 borrow stack 借用栈，来做分析，虽然同时有多个可变借用存在，但只有一个有效，那么不会违反借用规则. [^4]  
borrow stack 如下所示， 栈底为 owner, 栈内为所有存活的借用变量，栈顶为有效借用
```rust
fn borrow_stack() {
    let mut a = (3, 4u32); // a
    let a1 = &mut a; // a, a1

    let (ref mut z, ref mut x) = a1;
    // a, a1, z
    // a, a1, x

    //a1.clone(); // invalid
    
    let z1 = &mut *z; // a, a1, z, z1
    *z = 4; // a, a1, z
    z.clone(); // a, a1, z, z_temp: &i32
}
```

## 自动重借用
### 自动重借用，函数消耗的变量是自动生成的重借用，调用结束后，被重借用的变量再次有效。  
```rust
fn f1(t: &mut T);
fn f2(&mut self);
// 以上类似签名的函数，实际调用时,都有一个自动重借用
// 对 `&T` 可以认为发生了重借用, 也可以认为 `Copy` 生效
f1(t);
//相当于
f1(&mut *t);

a.f2();
//相当于
(&mut *a).f2();
```

### 例外，可能需要手写 `&mut *t`  
  这些例外很可能随着 rust 版本升级而变化
  - 例外1 泛型 `F=&mut X`
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
  - 例外2 多个借用参数
```rust
fn ex2(t: &mut T, x: &mut X);
ex2(t, x);
//可能只对 t 重借用了，而没有重借用 x
```

## 参考
[^1]: [`&mut T` 多次使用(reborrow)的疑问](https://rustcc.cn/article?id=28fedcbc-d0c9-41e1-8d95-de73a578a078)
[^2]: [better documentation of reborrowing#788](https://github.com/rust-lang/reference/issues/788#issuecomment-1420528041)
[^3]: [Parameter coercion in Rust](http://smallcultfollowing.com/babysteps/blog/2013/11/20/parameter-coercion-in-rust/#reborrowing)
[^4]: [Stacked Borrows](https://rust-unofficial.github.io/too-many-lists/fifth-stacked-borrows.html)
