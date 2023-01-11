#![feature(trait_alias)]

trait A {}
trait B {}

trait X: A + B {
    fn x(&self) {
        println!("X::x");
    }
}
impl<T: A + B> X for T {}

trait Y = A + B;