#![feature(const_generics)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(const_type_name)]

mod measure;
mod measure_length;
mod measure_mass;
mod measure_composite;

use measure_length::*;
use measure_mass::*;

use measure_composite::*;

use core::any::type_name;
use std::mem::size_of;


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
