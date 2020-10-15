#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]

use crate::measure::*;

use crate::measure_length::LengthMetric;
use crate::measure_mass::MassMetric;

use std::marker::PhantomData;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct CompositeMetric<M1: Metric, const D1: i8, M2: Metric, const D2: i8>(PhantomData<M1>, PhantomData<M2>);

impl<M1: Metric, const D1: i8, M2: Metric, const D2: i8> Metric for CompositeMetric<M1, D1, M2, D2> {
    const NAME: &'static str = "CompositeMetric-TODO";
    type BaseUnit = CompositeUnit<M1, D1, M2, D2>;
}

#[derive(Debug, Clone, Copy)]
pub struct CompositeUnit<M1: Metric, const D1: i8, M2: Metric, const D2: i8>(PhantomData<M1>, PhantomData<M2>);
impl<M1: Metric, const D1: i8, M2: Metric, const D2: i8> Unit<CompositeMetric<M1, D1, M2, D2>> for CompositeUnit<M1, D1, M2, D2> {
}

// 只实现相同单位的度量值相加 不同单位用 into 手动转换
impl<
    M1: Metric, const D1L: i8, const D1R: i8,
    M2: Metric, const D2L: i8, const D2R: i8
>
 Mul<Measure<CompositeMetric<M1, D1L, M2, D2L>, CompositeUnit<M1, D1L, M2, D2L>>> for Measure<CompositeMetric<M1, D1R, M2, D2R>, CompositeUnit<M1, D1R, M2, D2R>> {
    type Output = Measure<CompositeMetric<M1, {D1L + D1R}, M2, {D2L + D2R}>, CompositeUnit<M1, {D1L + D1R}, M2, {D2L + D2R}>>;

    fn mul(self, rhs: Measure<CompositeMetric<M1, D1L, M2, D2L>, CompositeUnit<M1, D1L, M2, D2L>>) -> Self::Output {
        Self::new(self.value() * rhs.value())
    }
}