#![feature(const_generics)]
#![feature(const_type_name)]

use core::any::type_name;
use std::marker::PhantomData;

trait MeasuringUnit<const F: f64>: Copy + Clone {}

struct Measure<U, const F: f64>(f64, PhantomData<U>) where U: MeasuringUnit<F>;

#[derive(Debug, Clone, Copy)]
struct LengthUnit<const F: f64>();
impl<const F: f64> MeasuringUnit<F> for LengthUnit<F> {}

type Meter = Measure<LengthUnit<1.0>, 1.0>;
type Cm = Measure<LengthUnit<100.0>, 300.0>; // define here without error
const CM_NAME: &str = type_name::<Cm>(); // first use
// 1. 报错位置不在 16行错误定义处 而是在 17 行 第一次使用处

// 2. 能简化定义成 type Cm = Measure<LengthUnit<100.0>> 吗 可以防止出错
// 答: 不能, 参见 下面 FakeUnit 和 Fake1 Fake2

#[derive(Debug, Clone, Copy)]
struct FakeUnit();
impl MeasuringUnit<1.0> for FakeUnit {}
impl MeasuringUnit<2.0> for FakeUnit {}

type Fake1 = Measure<FakeUnit, 1.0>;
type Fake2 = Measure<FakeUnit, 2.0>;


fn main() {
    println!("{}", type_name::<Meter>());
    println!("{}", type_name::<Cm>()); // compile error here: the trait `MeasuringUnit<300f64>` is not implemented for `LengthUnit<100f64>`

    println!("{}", type_name::<Fake1>());
    println!("{}", type_name::<Fake2>());
}