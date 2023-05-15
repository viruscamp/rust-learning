#![feature(trait_alias)]

trait A {}
trait B {}

trait X: A + B {}
impl<T: A + B> X for T {}

trait Y = A + B;

trait Z<'a> = A + B + 'a;

trait Z1<'a>: A + B + 'a {}
impl<'a, T: A + B + 'a> Z1<'a> for T {}

fn use_x(_x: &impl X) {
    println!("use x");
}

fn use_y(_y: &impl Y) {
    println!("use y");
}

#[test]
fn test_trait_alias() {
    struct T(i32);
    impl A for T {}
    impl B for T {}

    let t = T(3);
    use_x(&t);
    use_y(&t);
}