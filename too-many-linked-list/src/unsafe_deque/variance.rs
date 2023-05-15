//! [7.2. Variance and PhantomData](https://rust-unofficial.github.io/too-many-lists/sixth-variance.html)

/// for type `F<T>`
/// 1. 协变 F is covariant if F<Sub> is a subtype of F<Super> (subtyping "passes through")
/// 2. 逆变 F is contravariant if F<Super> is a subtype of F<Sub> (subtyping is "inverted")
/// 3. 不变 F is invariant otherwise (no subtyping relationship exists)

/// C# kotlin 定义类型时用 in out 标记可变性
/// Java 使用类型时用 <? extends Super> <? super Sub> 标记可变性
/// rust 自动推导可变性, 有一可变性表, struct enum union 的可变性根据其包含 field 类型推出
/// Vec<T> 包含 alloc::raw_vec::RawVec<T> 包含 core::ptr::Unique<T> 包含 std::marker::PhantomData<T> 其为协变
/// Cell<T> 包含 std::cell::UnsafeCell<T> 其为不变

/// 1. 生存期 'big 包括 'small , `&'big u32` 是 `&'small u32` 的子类型
/// ```no_run
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_u32<'big: 'small, 'small>(
///     big: &'big u32, 
///     small: &'small u32,
/// ) {
///     // 其实是 take_two<&'small u32>(big, small); 子类型用于父类型
///     take_two(big, small);
/// }
/// ```
/// 2. failed
/// ```no_run,compile_fail
/// use std::cell::Cell;
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_cell<'big: 'small, 'small>(
///     // NOTE: these two lines changed
///     big: Cell<&'big u32>, 
///     small: Cell<&'small u32>,
/// ) {
///     // 应该写作 take_two<Cell<&'small u32>>(big, small);
///     // Cell 不支持协变, 类型`Cell<&'big u32>`无法转为`Cell<&'small u32>`
///     take_two(big, small);
/// }
/// ```
/// 3. passed
/// ```no_run
/// fn take_two<T>(_val1: T, _val2: T) { }
/// fn two_refs_vec<'big: 'small, 'small>(
///     big: Vec<&'big u32>, 
///     small: Vec<&'small u32>,
/// ) {
///     // 其实是 take_two<Vec<&'small u32>>(big, small);
///     // Vec 支持协变, 类型`Vec<&'big u32>`可以转为`Vec<&'small u32>`
///     take_two(big, small);
/// }
/// ```
#[cfg(doctest)]
fn take_two<T>(_val1: T, _val2: T) {}

#[cfg(doctest)]
#[doc = include_str!("../../../variance-rust.md")]
fn variance_rust_extern_doc() {}

#[cfg(doctest)]
/// ```compile_fail
/// '_long: {
///     let a = 3;
///     let mut l = LinkedList::new();
///     l.push_front(&a); // make sure l is `LinkedList<&'_long i32>`
///     '_short: {
///         let b = 4;
///         l.push_front(&b); // push a `&'_short i32` to `LinkedList<&'_long i32>`
///     }
///     let l = l; // force extend the lifetime of `l`
/// }
/// ```
fn lifetime_subtyping() {}

#[cfg(test)]
mod lifetime_variance {
    #[test]
    #[ignore]
    /// 生存期协变
    fn lifetime_covariant() {
        let i_long = 3;
        {   
            let i_short = 4;
            lifetime_covariant_impl(&i_short, &i_long);
            dbg!(i_short);
        }
        dbg!(i_long);
    }

    fn lifetime_covariant_impl<'long: 'short, 'short>(a: &'short i32, b: &'long i32) {
        //! `'long` 是 `'short` 的子类型, `&'long i32` 是 `&'short i32` 的子类型
        let mut vec_long: Vec<&'long i32> = vec![b];
        let mut vec_short: Vec<&'short i32> = vec![a];
    
        vec_short = vec_long; // 协变成功 `Vec<&'long i32>` 是 `Vec<&'short i32>` 的子类型
    }

    #[test]
    #[ignore]
    /// 函数参数逆变
    fn lifetime_fn_contravariant() {
        let string_outer = String::from("outer");
        lifetime_fn_contravariant_impl(string_outer.as_str());
    }

    fn lifetime_fn_contravariant_impl<'outer>(str_outer: &'outer str) {
        let str_static: &'static str = "static";

        fn compare_with_static(instr: &'static str) -> bool {
            instr == "static"
        } // 类型 `Fn(&'static str) -> bool`

        fn make_compare_closure<'x>(a: &'x str) -> impl Fn(&'x str) -> bool {
            move |instr: &'x str| {
                instr == a
            }
        } // 返回值类型 `Fn(&'x str) -> bool`

        struct S<'z>(&'z str);
        impl<'z> S<'z> {
            fn do_compare<F: FnMut(&'z str) -> bool>(&self, mut f: F) -> F {
                f(self.0);
                f
            }
        }

        let mut closure_compare_static = make_compare_closure(str_static);
        let mut closure_compare_outer = make_compare_closure(str_outer);

        let s_static: S<'static> = S("xyz"); // `s_static.do_compare` 参数类型为 `Fn(&'static str) -> bool`
        s_static.do_compare(compare_with_static); // 类型相符, 当然可以用 `Fn(&'static str) -> bool` 做参数
        closure_compare_static = s_static.do_compare(closure_compare_static); // 类型相符
        closure_compare_outer = s_static.do_compare(closure_compare_outer); // 逆变, 实参类型为 `Fn(&'outer str) -> bool`

        let s_outer: S<'outer> = S(str_outer); // `s_outer.do_compare` 参数类型为 `Fn(&'outer str) -> bool`
        //s_outer.do_compare(compare_with_static); // 协变失败
        closure_compare_static = s_outer.do_compare(closure_compare_static); // 预计为协变失败, 实际可以编译, 应该是自动类型推断导致其并非协变
        closure_compare_outer = s_outer.do_compare(closure_compare_outer); // 类型相符

        {
            let string_inner = String::from("inner"); // 命名其生存期为 'inner
            let str_inner: &str = string_inner.as_str();
            let mut closure_compare_inner = make_compare_closure(str_inner);

            closure_compare_inner = s_static.do_compare(closure_compare_inner); // 逆变, 实参`Fn(&'inner str) -> bool` 替代形参 `Fn(&'static str) -> bool`
            closure_compare_inner = s_outer.do_compare(closure_compare_inner);  // 逆变, 实参`Fn(&'inner str) -> bool` 替代形参 `Fn(&'outer str) -> bool`

            let s_inner = S(str_inner);
            //closure_compare_static = s_inner.do_compare(closure_compare_static); // 协变失败
            //closure_compare_outer = s_inner.do_compare(closure_compare_outer); // 协变失败
            closure_compare_inner = s_inner.do_compare(closure_compare_inner); // 类型相符

            s_inner.do_compare(make_compare_closure(str_outer)); // 预计为协变失败, 实际可以编译, 应该是自动类型推断导致其并非协变
        }
        // 强制拉长生存期
        drop(closure_compare_outer);
        drop(closure_compare_static);

        drop(s_outer);
        drop(s_static);
        drop(str_outer);
    }

    #[test]
    #[ignore]
    /// 函数返回值协变
    fn lifetime_fn_covariant() {
        let str_outer = String::from("outer");
        lifetime_fn_covariant_impl(str_outer.as_str());
    }

    fn lifetime_fn_covariant_impl<'outer>(str_outer: &'outer str) {
        let str_static: &'static str = "static";

        fn return_static() -> &'static str {
            "abc"
        } // 类型 `Fn() -> &'static str`

        fn make_return_closure<'x>(a: &'x str) -> impl Fn() -> &'x str {
            move || { a }
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

            //s_outer.set_with(make_return_closure(str_inner)); // 逆变失败
        }

        // 强制拉长生存期
        s_outer;
        s_static;
        str_outer;
    }

    /// 函数逆变 失败的赋值证明, 自动类型推断 fn_long 和 fn_long 为同一类型
    fn lifetime_fn_contravariant_set<'long: 'short, 'short>(i_short: &'short i32, i_long: &'long i32) {
        //! `'long` 是 `'short` 的子类型, `&'long i32` 是 `&'short i32` 的子类型
        fn make_compare_closure<'x, T: PartialEq>(v: &'x T) -> impl Fn(&'x T) -> bool {
            move |inarg: &'x T| { inarg == v }
        } // 返回值类型 `Fn(&'x T) -> bool`
        let mut fn_long = make_compare_closure(i_long);
        let mut fn_short = make_compare_closure(i_short);
        fn_long = fn_short;
        fn_short = fn_long; 
    }

    /// 函数协变 失败的赋值证明, 自动类型推断 fn_long 和 fn_long 为同一类型
    fn lifetime_fn_covariant_set<'long: 'short, 'short>(i_short: &'short i32, i_long: &'long i32) {
        //! `'long` 是 `'short` 的子类型, `&'long i32` 是 `&'short i32` 的子类型
        fn make_return_closure<'x, T>(v: &'x T) -> impl Fn() -> &'x T {
            move || { v }
        } // 返回值类型 `Fn() -> &'x T`
        let mut fn_long = make_return_closure(i_long);
        let mut fn_short = make_return_closure(i_short);
        fn_short = fn_long;
        fn_long = fn_short;
    }
    
    /// 函数协变逆变 失败的赋值证明, 自动类型推断 fn_long 和 fn_long 为同一类型
    fn lifetime_fn_co_contra_variant_set<'long: 'short, 'short>(i_short: &'short i32, i_long: &'long i32, i_static: &'static i32) {
        //! `'long` 是 `'short` 的子类型, `&'long i32` 是 `&'short i32` 的子类型
        fn make_closure<'x, 'y, T>(vx: &'x T, vy: &'y T) -> impl Fn(&'x T) -> &'y T {
            move |inarg: &'x T| { vy }
        } // 返回值类型 `Fn(&'x T) -> &'y T`
        let mut fn_long = make_closure(i_long, i_long);
        let mut fn_short_static = make_closure(i_short, i_static);
        fn_long = fn_short_static;
        fn_short_static = fn_long;
    }
}

#[cfg(test)]
/// 以下内容试图证明 `trait` 子类型的'类型实例化' 符合型变的协变逆变,
/// 最终只能证明其均为协变, 那么就没必要引入协变逆变的概念.
mod trait_variance {
    trait Animal {
        fn eat(&self) {}
    }

    trait Dog : Animal {
        fn bark(&self) {}
    }

    trait Cat : Animal {
        fn moew(&self) {}
    }

    struct CorgiDog;
    impl Animal for CorgiDog {}
    impl Dog for CorgiDog {}

    struct BlueCat;
    impl Animal for BlueCat {}
    impl Cat for BlueCat {}

    #[test]
    #[ignore]
    fn trait_subtyping() {
        fn use_animal<A: Animal>(a: A) {}

        fn use_dog<D: Dog>(d: D) {
            // `Dog` 是 `Animal` 的子类型
            use_animal(d);
        }
        
        // `CorgiDog` 是 `Animal` 的子类型
        use_animal(CorgiDog);
        
        // `CorgiDog` 是 `Dog` 的子类型
        use_dog(CorgiDog);
    }

    #[test]
    #[ignore]
    // 简单用泛型条件理解即可, 没必要用协变逆变
    fn trait_covariant() {
        // `Cage<A: Animal>`对`A`协变 应该说类型实例化的过程全都是协变 没有逆变 不变
        // 简单用泛型条件理解即可, 没必要用协变逆变
        struct Cage<A: Animal>(Option<A>);

        fn put_in_cage<A: Animal>(c: Cage<A>) {}

        fn put_dog_in_cage<D: Dog>(c: Cage<D>) {
            // `Cage<Dog>` 是 `Cage<Animal>` 的子类型
            put_in_cage(c);
        }

        // `Cage<CorgiDog>` 是 `Cage<Animal>` 的子类型
        put_in_cage(Cage(Some(CorgiDog)));

        // `Cage<CorgiDog>` 是 `Cage<Dog>` 的子类型
        put_dog_in_cage(Cage::<CorgiDog>(None));
    }

    #[test]
    #[ignore]
    fn trait_fn_covariant() {
        fn use_fn<A: Animal, F: Fn() -> A>(f: F) {
        }

        fn use_fn_2<D: Dog, F: Fn() -> D>(f: F) {
            use_fn(f);
        }
    }

    #[test]
    #[ignore]
    fn trait_fn_contravariant() {
        fn use_fn<C: Cat, F: Fn(C)>(f: F) {
        }

        fn use_fn_2<C: Animal, F: Fn(C)>(f: F) {
            // use_fn(f); // 错误 那么就不是逆变关系
        }
    }

    #[test]
    #[ignore]
    /// 理解错误 请不要用 协变逆变来理解, 用类型实例化理解
    fn trait_fn_contravariant_covariant_1() {
        struct CatDog<C: Cat, D: Dog>(C, D);
        impl<C: Cat, D: Dog> CatDog<C, D> {
            fn use_fn<F: Fn(C) -> D>(&self, f: F) {
            }
        }

        fn cat_to_dog<C: Cat, D: Dog>(a: C) -> D {
            unimplemented!()
        }
        
        fn animal_to_corgi<A: Animal>(a: A) -> CorgiDog {
            CorgiDog
        }

        fn blue_cat_to_corgi(c: BlueCat) -> CorgiDog {
            CorgiDog
        }
        
        let cd = CatDog(BlueCat, CorgiDog);
        cd.use_fn(cat_to_dog);
        cd.use_fn(animal_to_corgi);  // 同时协变 逆变 是吗? 不是
        cd.use_fn(blue_cat_to_corgi); // 这个算什么?
    }

    #[test]
    #[ignore]
    /// 理解错误 请不要用 协变逆变理解, 用类型实例化理解
    fn trait_fn_contravariant_covariant_2() {
        fn cat_to_dog<C: Cat, D: Dog>(a: C) -> D {
            unimplemented!()
        }
        
        fn animal_to_corgi<A: Animal>(a: A) -> CorgiDog {
            CorgiDog
        }

        fn white_cat_to_black_dog(c: WhiteCat) -> BlackDog {
            BlackDog
        }
        
        struct BlackDog;
        impl Animal for BlackDog {}
        impl Dog for BlackDog {}

        struct WhiteCat;
        impl Animal for WhiteCat {}
        impl Cat for WhiteCat {}

        fn use_fn<C: Cat, D: Dog, F: Fn(C) -> D>(f: F) {
        }

        use_fn(cat_to_dog::<WhiteCat, BlackDog>);
        use_fn(animal_to_corgi::<BlueCat>);  // 同时协变 逆变 是吗? 不是
        use_fn(white_cat_to_black_dog); // 这个算什么?
    }
}
