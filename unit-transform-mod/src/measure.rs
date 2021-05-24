use std::fmt::Display;
use std::ops::Add;
use std::marker::PhantomData;

// 度量类型 质量 长度 时间
pub trait Metric<M: Metric = Self> {
    const NAME: &'static str;
    type BaseUnit: Unit<M>;

    fn name() -> &'static str {
        Self::NAME
    }
}

// 度量类型泛型实现 大概是不能用
// type MassMetric = GenericMetric<"mass", GenericUnit<MassMetric1, "kg", 1.0>>; 类型循环依赖
#[derive(Debug, Clone, Copy)]
pub struct GenericMetric<const NAME: &'static str, BU: Unit<Self>>(PhantomData<BU>);
impl<const NAME: &'static str, BU: Unit<Self>> Metric for GenericMetric<NAME, BU> {
    const NAME: &'static str = NAME;
    type BaseUnit = BU;
}

// 计量单位
pub trait Unit<M: Metric> {
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
pub struct GenericUnit<M: Metric, const NAME: &'static str, const FACTOR: f64>(PhantomData<M>);
impl<M: Metric, const NAME: &'static str, const FACTOR: f64> Unit<M> for GenericUnit<M, NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
}

// 测量值 包括数值 单位
#[derive(Debug, Clone, Copy)]
pub struct Measure<M: Metric, U: Unit<M>>(f64, PhantomData<M>, PhantomData<U>);

impl<M: Metric, U: Unit<M>> Measure<M, U> {
    pub fn new(value: f64) -> Self {
        Self(value, PhantomData, PhantomData)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn factor() -> f64 {
        U::FACTOR
    }

    // 类型转换 暂时无法实现 trait From
    // 代码膨胀大概很多 mm cm mm 3种互转 9种实现
    pub fn _into<U2: Unit<M>>(self) -> Measure<M, U2> {
        Measure::new(self.0 * U2::FACTOR / U::FACTOR)
    }

    // 类型转换 暂时无法实现 trait From
    pub fn _from<U2: Unit<M>>(src: Measure<M, U2>) -> Self {
        Self::new(src.0 * U::FACTOR / U2::FACTOR)
    }

    // 向基本类型转换的功能
    pub fn base(self) -> Measure<M, <M as Metric>::BaseUnit> {
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
pub impl<M: Metric, U1: Unit<M>, U2: Unit<M>> From<Measure<M, U2>> for Measure<M, U1> //where U1 != U2
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
