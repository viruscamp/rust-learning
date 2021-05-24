use crate::measure::*;

// 定义长度
#[derive(Debug, Clone, Copy)]
pub struct LengthMetric();
impl Metric for LengthMetric {
    const NAME: &'static str = "length";
    type BaseUnit = LengthUnit<"m", 1.0>;
}

// 实现长度单位 不用 GenericUnit
#[derive(Debug, Clone, Copy)]
pub struct LengthUnit<const NAME: &'static str, const FACTOR: f64>();
impl<const NAME: &'static str, const FACTOR: f64> Unit<LengthMetric> for LengthUnit<NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
}

pub type Meter = Measure<LengthMetric, <LengthMetric as Metric>::BaseUnit>;
pub type Cm = Measure<LengthMetric, LengthUnit<"cm", 100.0>>;
pub type Mm = Measure<LengthMetric, LengthUnit<"mm", 1000.0>>;
pub const INCH_PER_METER: f64 = 1000.0/24.5;
pub type Inch =  Measure<LengthMetric, LengthUnit<"inch", INCH_PER_METER>>;

pub type Mm2 = Measure<LengthMetric, GenericUnit<LengthMetric, "mm", 1000.0>>; // 混合默认实现 可以和 Meter Mm 互转
