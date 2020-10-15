#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]

use crate::measure::*;

// 定义质量
#[derive(Debug, Clone, Copy)]
pub struct MassMetric();
impl Metric for MassMetric {
    const NAME: &'static str = "mass";
    type BaseUnit = MassUnitKilogram;
}

// 无法使用 GenericMetric , 类型循环依赖
// type MassMetric1 = GenericMetric<"mass", GenericUnit<MassMetric1, "kg", 1.0>>;

// 实现质量单位 用 GenericUnit
pub type MassUnit<const NAME: &'static str, const FACTOR: f64> = GenericUnit<MassMetric, NAME, FACTOR>;
pub type MassMeasure<const NAME: &'static str, const FACTOR: f64> = Measure<MassMetric, MassUnit<NAME, FACTOR>>;

pub type MassUnitKilogram = MassUnit<"kg", 1.0>;

pub type Kilogram = Measure<MassMetric, MassUnitKilogram>;
pub type Gram = MassMeasure<"g", 1000.0>;
pub const POUND_PER_KILOGRAM: f64 = 2.2046;
pub type Pound = MassMeasure<"pound", POUND_PER_KILOGRAM>;
