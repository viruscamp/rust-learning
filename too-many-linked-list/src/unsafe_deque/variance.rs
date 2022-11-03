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
mod test_variance {
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
    fn trait_covariant() {
        // `Cage<A: Animal>`对`A`协变
        struct Cage<A: Animal>(Option<A>);

        fn put_in_cage<A: Animal>(c: Cage<A>) {}

        fn put_dog_in_cage<D: Dog>(c: Cage<D>) {
            // `Cage<Dog>` 是 `Cage<Animal>` 的子类型, 协变
            put_in_cage(c);
        }

        // `Cage<CorgiDog>` 是 `Cage<Animal>` 的子类型, 协变
        put_in_cage(Cage(Some(CorgiDog)));

        // `Cage<CorgiDog>` 是 `Cage<Dog>` 的子类型, 协变
        put_dog_in_cage(Cage::<CorgiDog>(None));
    }

    #[test]
    #[ignore]
    fn trait_fn_contravariant_covariant() {
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
        cd.use_fn(animal_to_corgi);  // 同时协变 逆变
        cd.use_fn(blue_cat_to_corgi); // 这个算什么?
    }

    #[test]
    #[ignore]
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
        use_fn(animal_to_corgi::<BlueCat>);  // 同时协变 逆变
        use_fn(white_cat_to_black_dog); // 这个算什么?
    }

    #[test]
    #[ignore]
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
        let mut vec_long: Vec<&'long i32> = vec![b];
        let mut vec_short: Vec<&'short i32> = vec![a];
    
        vec_short = vec_long; // 成功
    }

    #[test]
    #[ignore]
    fn lifetime_contravariant() {
        let str = String::from("hello");
        lifetime_contravariant_impl(str.as_str());
    }

    fn lifetime_contravariant_impl<'t>(argt: &'t str) {
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

    #[test]
    #[ignore]
    fn lifetime_fn_covariant() {
        let str1 = String::from("xyz");
        lifetime_fn_covariant_impl(str1.as_str());
    }

    fn lifetime_fn_covariant_impl<'t>(argt: &'t str) {
        struct S<'z>(&'z str);
        impl<'z> S<'z> {
            fn use_fn<F: Fn() -> &'z str>(&self, f: F) {
                f();
            }
        }

        let t: S = S(argt); // S<'t>
        {
            let str_small = String::from("abc");
            let closure_small = || { str_small.as_str() };
            t.use_fn(closure_small); // 函数返回值协变
        }
        let closure_t = || { argt };
        t.use_fn(closure_t); // 类型签名与参数类型相同
    }
}
