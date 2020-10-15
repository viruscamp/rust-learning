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

//pub type MassMetric = GenericMetric<"mass", MassUnitKilogram>;

// 实现质量单位 用 GenericUnit
pub type MassUnit<const NAME: &'static str, const FACTOR: f64> = GenericUnit<MassMetric, NAME, FACTOR>;

pub type MassUnitKilogram = MassUnit<"kg", 1.0>;

pub type Kilogram = Measure<MassUnitKilogram>;
pub type Gram = Measure<MassUnit<"g", 1000.0>>;
pub const POUND_PER_KILOGRAM: f64 = 2.2046;
pub type Pound = Measure<MassUnit<"pound", POUND_PER_KILOGRAM>>;
