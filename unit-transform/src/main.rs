#![feature(const_generics)]

use core::ops::Add;
use core::ops::Sub;
use core::ops::Neg;
use core::marker::PhantomData;

trait OptTrue {}
struct Opt<const B: bool> {}
impl OptTrue for Opt<true> {}

trait LengthType<const F: f64> {}
//trait LengthType1<const F: f64> where F > 0.0 {}
//trait LengthType2<const F: f64> where Opt<{F > 0.0}>: OptTrue {}

#[derive(Debug, Clone, Copy)]
struct Length<const F: f64>(f64, PhantomData<dyn LengthType<F>>);
type Meter = Length<1.0>;
type Mm = Length<1000.0>;
type Inch = Length<{1000.0/25.4}>;

impl<const F1: f64> Length<F1> {
    fn new(val: f64) -> Self {
        Length(val, PhantomData)
    }

    fn value(self) -> f64 {
        self.0
    }

    fn factor() -> f64 {
        F1
    }

    fn _from<const F2: f64>(other: Length<F2>) -> Self {
        Self::new(other.0 * F1 / F2)
    }

    fn _into<const F2: f64>(self) -> Length<F2> {
        Length::new(self.0 * F2 / F1)
    }
}

impl<const F: f64> Add for Length<F> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<const F1: f64, const F2: f64> Sub<Length<F2>> for Length<F1> {
    type Output = Self;
    fn sub(self, other: Length<F2>) -> Self {
        Self::new(self.0 + other.0)
    }
}


/*
impl<const F1: f64, const F2: f64> From<Length<F2>> for Length<F1>
    where Opt<{F1 != F2}>: OptTrue
    //where F1 != F2
{
    fn from(src: Length<F2>) -> Self {
        src._into()
    }
}
*/

fn main() {
    let m1 = Meter::new(1.0);
    let inch3 = Inch::new(3.0);
    let mm40 = Mm::new(40.0);

    let a = m1 + inch3._into();
    println!("m1 + inch3 = {} m", a.value());

    let b = mm40 + inch3._into();
    println!("mm40 + inch3 = {} mm", b.value());

    let c = mm40 + m1._into();
    println!("mm40 + m1 = {} mm", c.value());
}
