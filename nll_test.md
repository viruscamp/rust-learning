# NLL 导致的非直觉的作用域扩大

## 1. NLL 完全无视词法作用域    
1.1. 是否可以认为 NLL 在`fb.set(&b);`处缩短了`fb`的起始生存期以配合`b`?    
1.2. 应该认为 NLL 缩短了`fb` 还是延长了`b` 的生存期?   
```rust,name=dsfd
struct MyCell<T>(T);
impl<T> MyCell<T> {
    pub fn set(&mut self, t: T) {
        self.0 = t;
    }
}
let a = 3;
let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
let _ = {
    let b = 4;
    fb.set(&b);
    false
    // 直觉上 b 在此结束生存期
};
// 直觉上 fb 在此结束生存期
// NLL 智能的使 b 和 fb 的生存期同时结束, 使得 fb 可以使用 b
```

## 2. 强制延长外部变量生存期得到符合直觉的编译失败
```rust,compile_fail
struct MyCell<T>(T);
impl<T> MyCell<T> {
    pub fn set(&mut self, t: T) {
        self.0 = t;
    }
}
let a = 3;
let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
let _ = {
    let b = 4;
    fb.set(&b);
    false
    // b 在此结束生存期
};
fb; // 强制延长 fb 生存期
```

## 3. `Drop`导致隐式生存期延长而编译失败 
主代码与 1 完全相同, 与 1 相比仅多一个空`Drop`, 而编译失败原因同 2
```rust,compile_fail
struct MyCell<T>(T);
impl<T> MyCell<T> {
    pub fn set(&mut self, t: T) {
        self.0 = t;
    }
}
impl<T> Drop for MyCell<T> {
    fn drop(&mut self) {}
}
let a = 3;
let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
let _ = {
    let b = 4;
    fb.set(&b);
    false
    // b 在此结束生存期
};
// drop 导致的隐式延长 fb 生存期
```

## 4. 编译失败 循环使得`fb`生存期不能缩短
有可能rust升级使其可通过编译
```rust,compile_fail
struct MyCell<T>(T);
impl<T> MyCell<T> {
    pub fn set(&mut self, t: T) {
        self.0 = t;
    }
}
let a = 3;
let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
let _ = loop {
    // loop 使得 fb 必须在此有效 而不能使 b 满足此要求
    let b = 4;
    fb.set(&b);
    if true { break false }
    // b 在此结束生存期
};
// fb 在此结束生存期
```

## 5. 编译成功 基本等同于 4
NLL 过于智能的消除了 `loop`
```rust
struct MyCell<T>(T);
impl<T> MyCell<T> {
    pub fn set(&mut self, t: T) {
        self.0 = t;
    }
}
let a = 3;
let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
let _ = loop {
    let b = 4;
    fb.set(&b);
    break false; // 过于智能的消除了 loop
};
```
