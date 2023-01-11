#![feature(generators, iter_from_generator)]

#[test]
fn std_iter_from_generator() {
    let it = std::iter::from_generator(|| {
        yield 1;
        yield 2;
        yield 3;
    });
    let v: Vec<_> = it.collect();
    assert_eq!(v, [1, 2, 3]);
}
