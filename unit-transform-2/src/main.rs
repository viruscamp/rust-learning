#![feature(adt_const_params)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(const_type_name)]

use std::ops::Add;
use std::ops::Sub;
use core::any::type_name;
use std::marker::PhantomData;

trait MeasuringUnit<const FACTOR: f64>: Copy + Clone {
    type BaseUnit: MeasuringUnit<1.0>;
    fn factor() -> f64 {
        FACTOR
    }
}

#[derive(Debug, Clone, Copy)]
struct Measure<Unit, const FACTOR: f64>(f64, PhantomData<Unit>) where Unit: MeasuringUnit<FACTOR>;
// 可能写出错误定义 Measure<LengthUnit<100.0>, 333.3>
//struct Measure1<Unit>(f64, PhantomData<Unit>) where Unit: MeasuringUnit;

impl<Unit, const FACTOR: f64> Measure<Unit, FACTOR> where Unit: MeasuringUnit<FACTOR> {
    fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    fn value(self) -> f64 {
        self.0
    }

    // 此处使用系数时 可以从两个地方取
    fn factor() -> f64 {
        //Unit::factor()
        FACTOR
    }

    // 类型转换需要条件 Unit::BaseType == U2::BaseType 但 rustc 未实现
    fn _into<U2, const F2: f64>(self) -> Measure<U2, F2> where U2: MeasuringUnit<F2> {
        Measure::new(self.0 * F2 / FACTOR)
    }

    // 类型转换需要条件 Unit::BaseType == U2::BaseType 但 rustc 未实现
    fn _from<U2, const F2: f64>(src: Measure<U2, F2>) -> Self where U2: MeasuringUnit<F2> {
        Self::new(src.0 * Unit::factor() / U2::factor())
    }

    // 向基本类型转换的功能
    fn base(self) -> Measure<Unit::BaseUnit, 1.0> {
        self._into()
    }
}

impl<Unit, const FACTOR: f64> Add for Measure<Unit, FACTOR> where Unit: MeasuringUnit<FACTOR> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

// 定义长度单位
#[derive(Debug, Clone, Copy)]
struct LengthUnit<const FACTOR: f64>();
impl<const FACTOR: f64> MeasuringUnit<FACTOR> for LengthUnit<FACTOR> {
    type BaseUnit = LengthUnit<1.0>;
}

// 定义能不能简化成 Measure<LengthUnit<1.0>>?
type Meter = Measure<LengthUnit<1.0>, 1.0>;
type Cm = Measure<LengthUnit<100.0>, 333.3>; // 错误的定义 定义可以写出来 不能用
type Mm = Measure<LengthUnit<1000.0>, 1000.0>;
const INCH_PER_METER: f64 = 1000.0/24.5;
type Inch =  Measure<LengthUnit<INCH_PER_METER>, INCH_PER_METER>;

// 定义质量单位
#[derive(Debug, Clone, Copy)]
struct MassUnit<const FACTOR: f64>();
impl<const FACTOR: f64> MeasuringUnit<FACTOR> for MassUnit<FACTOR> {
    type BaseUnit = MassUnit<1.0>;
}

type Kilogram = Measure<MassUnit<1.0>, 1.0>;
type Gram = Measure<MassUnit<1000.0>, 1000.0>;
const POUND_PER_KILOGRAM: f64 = 2.2046;
type Pound = Measure<MassUnit<POUND_PER_KILOGRAM>, POUND_PER_KILOGRAM>;

/*
// 试图用 trait 实现 单位类型 失败
trait MassUnit2<const FACTOR: f64>: MeasuringUnit<FACTOR> {
    type BaseType = MassUnit2<1.0>;
}
type Kilogram2 = Measure<MassUnit2<1.0>, 1.0>;
*/

fn main() {
    let _1m = Meter::new(1.0);
    //let _3cm = Cm::new(3); // 错误的定义 在这用的时候就报错
    let _40mm = Mm::new(40.0);
    let _2inch = Inch::new(2.0);

    let l1 = _1m + _2inch._into();
    println!("_1m + _2inch = {:?} m", l1.value());

    let l2 = _1m + _40mm._into();
    println!("_1m + _40mm = {:?} m", l2.value());

    let l3 = _2inch + _40mm._into();
    println!("_2inch + _40mm = {:?} inch", l3.value());

    let l4 = _2inch.base();
    println!("_2inch = {:?} m", l4.value());

    let _2kg = Kilogram::new(2.0);
    let _20g = Gram::new(20.0);
    let _1p = Pound::new(1.0);
    //let _2kg2 = Kilogram2::new(2.0);

    let m1 = _20g + _2kg._into();
    println!("_20g + _2kg = {:?} g", m1.value());
 
    let m2 = _20g + _1p._into();
    println!("_20g + _1p = {:?} g", m2.value());

    let m3 = _1p + _2kg._into();
    println!("_1p + _2kg = {:?} p", m3.value());

    let m4 = _1p.base();
    println!("_1p = {:?} kg", m4.value());

    // 未实现功能 导致不同类型单位可以转换
    let e1 = _2kg + _1m._into();
    println!("error: _2kg + _1m = {:?} kg", e1.value());
}