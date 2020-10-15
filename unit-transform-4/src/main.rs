#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(const_type_name)]

use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use core::any::type_name;
use std::mem::size_of;
use std::marker::PhantomData;

// 度量类型 质量 长度 时间
trait Metric<M: Metric = Self> {
    const NAME: &'static str;
    type BaseUnit: Unit<M>;

    fn name() -> &'static str {
        Self::NAME
    }
}

// 度量类型泛型实现 大概是不能用
// type MassMetric = GenericMetric<"mass", GenericUnit<MassMetric1, "kg", 1.0>>; 类型循环依赖
#[derive(Debug, Clone, Copy)]
struct GenericMetric<const NAME: &'static str, BU: Unit<Self>>(PhantomData<BU>);
impl<const NAME: &'static str, BU: Unit<Self>> Metric<Self> for GenericMetric<NAME, BU> {
    const NAME: &'static str = NAME;
    type BaseUnit = BU;
}

// 计量单位
trait Unit<M: Metric> {
    const NAME: &'static str;
    const FACTOR: f64;
    //type Metric: Metric = M;

    fn factor() -> f64 {
        Self::FACTOR
    }
    fn name() -> &'static str {
        Self::NAME
    }
}

// 计量单位泛型实现
#[derive(Debug, Clone, Copy)]
struct GenericUnit<M: Metric, const NAME: &'static str, const FACTOR: f64>(PhantomData<M>);
impl<M: Metric, const NAME: &'static str, const FACTOR: f64> Unit<M> for GenericUnit<M, NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
}

// 测量值 包括数值 单位
#[derive(Debug, Clone, Copy)]
struct Measure<M: Metric, U: Unit<M>>(f64, PhantomData<M>, PhantomData<U>);

impl<M: Metric, U: Unit<M>> Measure<M, U> {
    fn new(value: f64) -> Self {
        Self(value, PhantomData, PhantomData)
    }

    fn value(self) -> f64 {
        self.0
    }

    fn factor() -> f64 {
        U::FACTOR
    }

    // 类型转换 暂时无法实现 trait From
    // 代码膨胀大概很多 mm cm mm 3种互转 9种实现
    fn _into<U2: Unit<M>>(self) -> Measure<M, U2> {
        Measure::new(self.0 * U2::FACTOR / U::FACTOR)
    }

    // 类型转换 暂时无法实现 trait From
    fn _from<U2: Unit<M>>(src: Measure<M, U2>) -> Self {
        Self::new(src.0 * U::FACTOR / U2::FACTOR)
    }

    // 向基本类型转换的功能
    fn base(self) -> Measure<M, <M as Metric>::BaseUnit> {
        self._into()
    }
}

impl<M: Metric, U: Unit<M>> Display for Measure<M, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.0, U::name())
    }
}

// 无法实现 trait From
// note: conflicting implementation in crate `core`: - impl<T> From<T> for T;
/*
impl<M: Metric, U1: Unit<M>, U2: Unit<M>> From<Measure<M, U2>> for Measure<M, U1> //where U1 != U2
{
    fn from(src: Measure<M, U2>) -> Self {
        src._into()
    }
}
*/

// 只实现相同单位的度量值相加 不同单位用 into 手动转换
impl<M: Metric, U: Unit<M>> Add for Measure<M, U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}
// TODO Sub Neg


// 定义长度
#[derive(Debug, Clone, Copy)]
struct LengthMetric();
impl Metric for LengthMetric {
    const NAME: &'static str = "length";
    type BaseUnit = LengthUnit<"m", 1.0>;
}

// 实现长度单位 不用 GenericUnit
#[derive(Debug, Clone, Copy)]
struct LengthUnit<const NAME: &'static str, const FACTOR: f64>();
impl<const NAME: &'static str, const FACTOR: f64> Unit<LengthMetric> for LengthUnit<NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
}

type Meter = Measure<LengthMetric, <LengthMetric as Metric>::BaseUnit>;
type Cm = Measure<LengthMetric, LengthUnit<"cm", 100.0>>;
type Mm = Measure<LengthMetric, LengthUnit<"mm", 1000.0>>;
const INCH_PER_METER: f64 = 1000.0/24.5;
type Inch =  Measure<LengthMetric, LengthUnit<"inch", INCH_PER_METER>>;

type Mm2 = Measure<LengthMetric, GenericUnit<LengthMetric, "mm", 1000.0>>; // 混合默认实现 可以和 Meter Mm 互转


// 定义质量
#[derive(Debug, Clone, Copy)]
struct MassMetric();
impl Metric for MassMetric {
    const NAME: &'static str = "mass";
    type BaseUnit = MassUnitKilogram;
}

// 无法使用 GenericMetric , 类型循环依赖
// type MassMetric1 = GenericMetric<"mass", GenericUnit<MassMetric1, "kg", 1.0>>;

// 实现质量单位 用 GenericUnit
type MassUnit<const NAME: &'static str, const FACTOR: f64> = GenericUnit<MassMetric, NAME, FACTOR>;
type MassMeasure<const NAME: &'static str, const FACTOR: f64> = Measure<MassMetric, MassUnit<NAME, FACTOR>>;

type MassUnitKilogram = MassUnit<"kg", 1.0>;

type Kilogram = Measure<MassMetric, MassUnitKilogram>;
type Gram = MassMeasure<"g", 1000.0>;
const POUND_PER_KILOGRAM: f64 = 2.2046;
type Pound = MassMeasure<"pound", POUND_PER_KILOGRAM>;


fn main() {
    let _1m = Meter::new(1.0);
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

    let m1 = _20g + _2kg._into();
    println!("_20g + _2kg = {}", m1);
 
    let m2 = _20g + _1p._into();
    println!("_20g + _1p = {}", m2);

    let m3 = _1p + _2kg._into();
    println!("_1p + _2kg = {}", m3);

    let m4 = _1p.base();
    println!("_1p = {}", m4); // 转到基本单位类型

    // 不同类型单位不能转换
    //let e1 = _2kg + _1m._into();
    //println!("error: _2kg + _1m = {}", e1);

    println!("type_name(f64) = {} size_of(f64) = {}", type_name::<f64>(), size_of::<f64>());

    println!("type_name(Meter) = {} size_of(Meter) = {}", type_name::<Meter>(), size_of::<Meter>());
    println!("type_name(Pound) = {} size_of(Pound) = {}", type_name::<Pound>(), size_of::<Pound>());

    println!("type_name(Mm) = {} size_of(Mm) = {}", type_name::<Mm>(), size_of::<Mm>());
    println!("type_name(Mm2) = {} size_of(Mm2) = {}", type_name::<Mm2>(), size_of::<Mm2>());

    let _10mm2 = Mm2::new(10.0);
    println!("{}", _10mm2 + _1m._into());
    println!("{}", _40mm + _10mm2._into());
}
