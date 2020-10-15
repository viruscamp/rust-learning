#![feature(associated_type_defaults)]

use std::ops::Add;

// 父接口
trait Base {
    type T: Add;
    fn num(&self) -> u8;
}

// 错误想法: 接口继承 通过子接口 为 父接口 提供默认实现
trait Sub1: Base {
    type T = i32;
    fn num(&self) -> u8 {
        1
    }
}

struct Sample();
//impl Sub1 for Sample {} // 错误想法: Sub1 需要的和 Base 需要的都在 Sub1 内提供了

// 正确思路:
// 1. trait 不是接口 没有 trait 继承
// 2. Supertraits 可以改写成 where bound 表示的是协议或限制 要实现 Sub2 必须实现 Base
// 3. Sub2 定义的 Base 的同名元素 没有覆盖 Base 内的 可以同时存在
trait Sub2 where Self: Base {
    type T = *const str; // 不是 Base::T 不满足 :Add
    fn num(&self) -> u8 {
        2
    }
}

impl Base for Sample {
    type T = u16;
    fn num(&self) -> u8 {
        0
    }
}

impl Sub2 for Sample {}

impl Sample {
    fn num(&self) -> u8 {
        10
    }
}

fn main() {
    let s = Sample();
    println!("s.num() = {}", s.num());
    println!("Base::num(&s) = {}", Base::num(&s));
    println!("Sub2::num(&s) = {}", Sub2::num(&s));
}
