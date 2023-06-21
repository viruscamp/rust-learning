# 内部可变性 Interior Mutability

1. 让多个 `&Container<T>` 都可以修改 `T`  

2. 有内部可变性的数据结构
  - 单线程 `Cell<T>` `RefCell<T>` 通常用 `Rc` 再次包装
  - 多线程 原子类型`AtomicBool` 锁类型 `Mutex<T>` `RwLock<T>` 通常用 `Arc` 再次包装  
    通用的泛型原子类型 [atomic](https://crates.io/crates/atomic)

3. `Cell<T>` 对标 `AtomicBool` 系列原子类型  
  首先尝试 Cell 不满足条件再使用 RefCell
  - 设值 `Cell::set` `Atomic*::store`
  - 取值 `Cell::<T>::get() where T: Copy` `Atomic*::load`
  - 同时取值设值 `Cell::replace` `Atomic*::swap`

4. `RefCell<T>` 对标 `RwLock<T>`
  - 当需要 `&mut T` 时
  - 当需要锁，保证同时只有一段代码可以访问时
  - 写锁 `RefCell::borrow_mut`(失败时 panic) `RwLock::write`(失败时等待)
  - 读锁 `RefCell::borrow`(失败时 panic) `RwLock::read`(失败时等待)  
    上述几个方法同时有 try_ 版本, 失败时手动处理
  - 相信程序员可以完全保证单线程执行顺序，只需要 borrow_mut 和 borrow 无需等待 无需 try
  - 而多线程无法保证执行顺序，需要等待锁
  