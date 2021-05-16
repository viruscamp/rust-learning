#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]

use crate::measure::*;

// 定义时间
#[derive(Debug, Clone, Copy)]
pub struct TimeMetric();

impl Metric for TimeMetric {
    const NAME: &'static str = "Time";
    type BaseUnit = Self;
}

// 用同一个 struct TimeMetric 实现 Metric 和其 BaseUnit 可以 但没必要
impl Unit<TimeMetric> for TimeMetric {
    const NAME: &'static str = "s";
    const FACTOR: f64 = 1.0;
}

// 实现时间单位 不用 GenericUnit
#[derive(Debug, Clone, Copy)]
pub struct TimeUnit<const NAME: &'static str, const FACTOR: f64>();
impl<const NAME: &'static str, const FACTOR: f64> Unit<TimeMetric> for TimeUnit<NAME, FACTOR> {
    const NAME: &'static str = NAME;
    const FACTOR: f64 = FACTOR;
}

pub type Meter = Measure<TimeMetric, TimeMetric>;
pub type Cm = Measure<TimeMetric, TimeUnit<"cm", 100.0>>;
pub type Mm = Measure<TimeMetric, TimeUnit<"mm", 1000.0>>;
pub const INCH_PER_METER: f64 = 1000.0/24.5;
pub type Inch =  Measure<TimeMetric, TimeUnit<"inch", INCH_PER_METER>>;

pub type Mm2 = Measure<TimeMetric, GenericUnit<TimeMetric, "mm", 1000.0>>; // 混合默认实现 可以和 Meter Mm 互转
