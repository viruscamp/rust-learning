#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(const_type_name)]

use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use core::any::type_name;
use std::marker::PhantomData;

// 度量类型 质量 长度 时间
trait Metric {
    fn name() -> &'static str;
}

/*
trait MetricT<const NAME: &'static str>: Metric {
    fn name() -> &'static str {
        NAME
    }
}
*/

struct MetricS<const NAME: &'static str>();
impl<const NAME: &'static str> Metric for MetricS<NAME> {
    fn name() -> &'static str {
        NAME
    }
}

trait Unit: Copy + Clone {
    type Metric: Metric;
    type BaseUnit: Unit; // where Unit::Metric == Self::Metric
    fn factor() -> f64;
    fn name() -> &'static str;
}

/*
trait UnitT<const METRIC_NAME: &'static str, const UNIT_NAME: &'static str, const FACTOR: f64>: Unit {
    type Metric = MetricT<METRIC_NAME>;
    type BaseUnit = UnitT<METRIC_NAME, UNIT_NAME, 1.0>;
    fn factor() -> f64 {
        FACTOR
    }
    fn name() -> &'static str {
        UNIT_NAME
    }
}
*/

#[derive(Debug, Clone, Copy)]
struct UnitS<const METRIC_NAME: &'static str, const UNIT_NAME: &'static str, const FACTOR: f64>();
impl<const METRIC_NAME: &'static str, const UNIT_NAME: &'static str, const FACTOR: f64> Unit for UnitS<METRIC_NAME, UNIT_NAME, FACTOR> {
    type Metric = MetricS<METRIC_NAME>;
    type BaseUnit = UnitS<METRIC_NAME, UNIT_NAME, 1.0>;  // TODO 推导不出基本单位类型 此处错了
    fn factor() -> f64 {
        FACTOR
    }
    fn name() -> &'static str {
        UNIT_NAME
    }
}

#[derive(Debug, Clone, Copy)]
struct Measure<U: Unit>(f64, PhantomData<U>);

impl<U: Unit> Display for Measure<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.0, U::name())
    }
}

impl<U: Unit> Measure<U> {
    fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    fn value(self) -> f64 {
        self.0
    }

    fn factor() -> f64 {
        U::factor()
    }

    // 类型转换需要条件 Unit::BaseType == U2::BaseType 但 rustc 未实现
    fn _into<U2: Unit>(self) -> Measure<U2> {
        Measure::new(self.0 * U2::factor() / U::factor())
    }

    // 类型转换需要条件 Unit::BaseType == U2::BaseType 但 rustc 未实现
    fn _from<U2: Unit>(src: Measure<U2>) -> Self {
        Self::new(src.0 * U::factor() / U2::factor())
    }

    // 向基本类型转换的功能
    fn base(self) -> Measure<U::BaseUnit> {
        self._into()
    }
}

// 只实现相同单位的度量值相加 不同单位用 into 手动转换
impl<U: Unit> Add for Measure<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

// 定义长度单位
const LENGTH_METRIC_NAME: &str = "length";
//#[derive(Debug, Clone, Copy)]
struct LengthMetric();
impl Metric for LengthMetric {
    fn name() -> &'static str {
        LENGTH_METRIC_NAME
    }
}

#[derive(Debug, Clone, Copy)]
struct LengthUnit<const NAME: &'static str, const FACTOR: f64>();
impl<const NAME: &'static str, const FACTOR: f64> Unit for LengthUnit<NAME, FACTOR> {
    type Metric = LengthMetric;
    type BaseUnit = LengthUnit<"m", 1.0>;
    fn factor() -> f64 {
        FACTOR
    }
    fn name() -> &'static str {
        NAME
    }
}

type Meter = Measure<LengthUnit<"m", 1.0>>;
type Cm = Measure<LengthUnit<"cm", 100.0>>;
type Mm = Measure<LengthUnit<"mm", 1000.0>>;
const INCH_PER_METER: f64 = 1000.0/24.5;
type Inch =  Measure<LengthUnit<"inch", INCH_PER_METER>>;

// 定义质量单位 要像定义长度一样定义 不能用 MetricS UnitS
const MASS_METRIC_NAME: &str = "mass";
type MassMetric = MetricS<MASS_METRIC_NAME>;
//trait MassUnit<const NAME: &'static str, const FACTOR: f64>: UnitT<MASS_METRIC_NAME, NAME, FACTOR> {}
//#[derive(Debug, Clone, Copy)]
//struct MassUnit<const NAME: &'static str, const FACTOR: f64>();
//impl<const NAME: &'static str, const FACTOR: f64> UnitT<MASS_METRIC_NAME, NAME, FACTOR> for MassUnit<NAME, FACTOR> {}

type Kilogram = Measure<UnitS<MASS_METRIC_NAME, "kg", 1.0>>;
type Gram = Measure<UnitS<MASS_METRIC_NAME, "g", 1000.0>>;
const POUND_PER_KILOGRAM: f64 = 2.2046;
type Pound = Measure<UnitS<MASS_METRIC_NAME, "pound", POUND_PER_KILOGRAM>>; // TODO 推导不出基本单位类型

fn main() {
    let _1m = Meter::new(1.0);
    //let _3cm = Cm::new(3); // 错误的定义 在这用的时候就报错
    let _40mm = Mm::new(40.0);
    let _2inch = Inch::new(2.0);

    let l1 = _1m + _2inch._into();
    println!("_1m + _2inch = {}", l1);

    let l2 = _1m + _40mm._into();
    println!("_1m + _40mm = {}", l2);

    let l3 = _2inch + _40mm._into();
    println!("_2inch + _40mm = {}", l3);

    let l4 = _2inch.base();
    println!("_2inch = {}", l4);

    let _2kg = Kilogram::new(2.0);
    let _20g = Gram::new(20.0);
    let _1p = Pound::new(1.0);
    //let _2kg2 = Kilogram2::new(2.0);

    let m1 = _20g + _2kg._into();
    println!("_20g + _2kg = {}", m1);
 
    let m2 = _20g + _1p._into();
    println!("_20g + _1p = {}", m2);

    let m3 = _1p + _2kg._into();
    println!("_1p + _2kg = {}", m3);

    let m4 = _1p.base();
    println!("_1p = {}", m4); // TODO 推导不出基本单位类型 错误展示

    // 未实现功能 导致不同类型单位可以转换
    let e1 = _2kg + _1m._into();
    println!("error: _2kg + _1m = {}", e1);
}