use std::fmt::Display;
use std::ops::Add;
use std::marker::PhantomData;

// 度量类型 质量 长度 时间
pub trait Metric {
    const NAME: &'static str;
    type BaseUnit: Unit; //where BaseUnit::Metric == Self;

    fn name() -> &'static str {
        Self::NAME
    }
}

// 度量类型泛型实现
#[derive(Debug, Clone, Copy)]
pub struct GenericMetric<const NAME: &'static str, BU: Unit>(PhantomData<BU>);
impl<const NAME: &'static str, BU: Unit> Metric for GenericMetric<NAME, BU> {
    const NAME: &'static str = NAME;
    type BaseUnit = BU;
}

// 计量单位
pub trait Unit {
    const NAME: &'static str;
    const FACTOR: f64;
    type Metric: Metric;

    const INSTANCE: Self;

    fn factor() -> f64 {
        Self::FACTOR
    }
    fn name() -> &'static str {
        Self::NAME
    }
}

// 计量单位泛型实现
#[derive(Debug, Clone, Copy)]
pub struct GenericUnit<M: Metric, const NAME: &'static str, const FACTOR: f64>(PhantomData<M>);
impl<M: Metric, const NAME: &'static str, const FACTOR: f64> Unit for GenericUnit<M, NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
    type Metric = M;

    const INSTANCE: Self = Self(PhantomData);
}

// 测量值 包括数值 单位
#[derive(Debug, Clone, Copy)]
pub struct Measure<U: Unit>(f64, PhantomData<U>);
impl<U: Unit> Measure<U> {
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn factor() -> f64 {
        U::FACTOR
    }

    // 类型转换 暂时无法实现 trait From
    // 代码膨胀大概很多 mm cm mm 3种互转 9种实现
    pub fn _into<U2: Unit>(self) -> Measure<U2>
        //where U::Metric == U2::Metric
    {
        Measure::new(self.0 * U2::FACTOR / U::FACTOR)
    }

    // 类型转换 暂时无法实现 trait From
    pub fn _from<U2: Unit>(src: Measure<U2>) -> Self
        //where U::Metric == U2::Metric
    {
        Self::new(src.0 * U::FACTOR / U2::FACTOR)
    }

    // 向基本类型转换的功能
    pub fn base(self) -> Measure<<U::Metric as Metric>::BaseUnit> {
        self._into()
    }
}

impl<U: Unit> Display for Measure<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{} {}", self.0, U::NAME)
    }
}

/*
pub trait From1<T>: Sized {
    fn from(_: T) -> Self;
}

impl<T> From1<T> for T {
    default fn from(src: T) -> Self {
        src
    }
}

// 无法实现 trait From
// note: conflicting implementation in crate `core`: - impl<T> From<T> for T;
impl<U1: Unit, U2: Unit> From1<Measure<U2>> for Measure<U1> where U1 != U2
{
    default fn from(src: Measure<U2>) -> Measure<U1> {
        src._into()
    }
}

// 泛型特化 确实替代了 impl<T> From<T> for T
impl<U: Unit> From1<Measure<U>> for Measure<U> {
    fn from(src: Measure<U>) -> Measure<U> {
        src
    }
}
*/

// 只实现相同单位的度量值相加 不同单位用 into 手动转换
impl<U: Unit> Add for Measure<U> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}
// TODO Sub Neg
