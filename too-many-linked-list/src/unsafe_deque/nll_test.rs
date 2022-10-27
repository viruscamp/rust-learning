/// NLL 导致的非直觉的作用域扩大

#[cfg(doctest)]
/// ```
/// struct MyCell<T>(T);
/// impl<T> MyCell<T> {
///     pub fn set(&mut self, t: T) {
///         self.0 = t;
///     }
/// }
/// let a = 3;
/// let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
/// let _ = {
///     let b = 4;
///     fb.set(&b);
///     false
///     // 直觉上 b 在此结束生存期
/// };
/// // 直觉上 fb 在此结束生存期
/// // NLL 智能的使 b 和 fb 的生存期同时结束, 使得 fb 可以使用 b
/// ```
fn extend_inner_lifetime_end() {}

#[cfg(doctest)]
/// ```compile_fail
/// struct MyCell<T>(T);
/// impl<T> MyCell<T> {
///     pub fn set(&mut self, t: T) {
///         self.0 = t;
///     }
/// }
/// let a = 3;
/// let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
/// let _ = {
///     let b = 4;
///     fb.set(&b);
///     false
///     // b 在此结束生存期
/// };
/// fb; // 强制扩展 fb 生存期
/// ```
fn force_extend_outer_lifetime_end_compile_fail() {}

#[cfg(doctest)]
/// ```compile_fail
/// struct MyCell<T>(T);
/// impl<T> MyCell<T> {
///     pub fn set(&mut self, t: T) {
///         self.0 = t;
///     }
/// }
/// impl<T> Drop for MyCell<T> {
///     fn drop(&mut self) {}
/// }
/// let a = 3;
/// let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
/// let _ = {
///     let b = 4;
///     fb.set(&b);
///     false
///     // b 在此结束生存期
/// };
/// // drop 导致的隐式强制扩展 fb 生存期
/// ```
fn force_extend_outer_lifetime_end_with_drop_compile_fail() {}

#[cfg(doctest)]
/// ```compile_fail
/// struct MyCell<T>(T);
/// impl<T> MyCell<T> {
///     pub fn set(&mut self, t: T) {
///         self.0 = t;
///     }
/// }
/// let a = 3;
/// let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
/// let _ = loop {
///     // loop 使得 fb 必须在此有效 而不能使 b 满足此要求
///     let b = 4;
///     fb.set(&b);
///     if true { break false }
///     // b 在此结束生存期
/// };
/// // fb 在此结束生存期
/// ```
fn force_extend_outer_lifetime_begin_with_loop_compile_fail() {}

#[cfg(doctest)]
/// ```
/// struct MyCell<T>(T);
/// impl<T> MyCell<T> {
///     pub fn set(&mut self, t: T) {
///         self.0 = t;
///     }
/// }
/// let a = 3;
/// let mut fb = MyCell(&a); // 将 fb 绑定到 a 的生存期
/// let _ = loop {
///     let b = 4;
///     fb.set(&b);
///     break false; // 过于智能的消除了 loop
/// };
/// ```
fn force_extend_outer_lifetime_begin_with_loop_passed() {}
