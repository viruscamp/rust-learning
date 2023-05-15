//! Cast from `f64` to `f32`
//! 1. any `f32` will be casted to `f64` and then cast back the same: `NAN`,`INFINITY`,`NEG_INFINITY`,`+0.0`,`-0.0`, `f32::MAX`, `f32::MIN`
//! 2. `x > f32::MAX as f64` will get `Error::Overflow`
//! 3. `x < f32::MIN as f64` will get `Error::Underflow`
//! 4. Other value in `f64` like `32.05`, `f64::MIN_POSITIVE` cast to `f32` will loss precison, and then cast back not the same

use cast::*;

#[test]
fn same_f32_f64_f32() {
    fn same(i: f32) {
        let o = f32(f64(i)).unwrap();
        let it = unsafe { core::mem::transmute::<f32, u32>(i) };
        let ot = unsafe { core::mem::transmute::<f32, u32>(o) };
        println!("{i} {it}");
        println!("output: {o} {ot}");
        assert_eq!(it, ot);
    }

    same(f32::NAN);
    same(f32::INFINITY);
    same(f32::NEG_INFINITY);
    same(0.0f32);
    same(-0.0f32);

    same(f32::MAX);
    same(f32::MIN);

    same(f32::EPSILON);
    same(f32::MIN_POSITIVE);

    same(32.05);
    same(-3443.222);
}

#[test]
fn same_f64_f32_f64() {
    fn same(i: f64) {
        let o = f64(f32(i).unwrap());
        let it = unsafe { core::mem::transmute::<f64, u64>(i) };
        let ot = unsafe { core::mem::transmute::<f64, u64>(o) };
        println!("input: {i} {it:0x}");
        println!("output: {o} {ot:0x}");
        assert_eq!(it, ot);
    }

    same(f64::NAN);
    same(f64::INFINITY);
    same(f64::NEG_INFINITY);
    same(0.0f64);
    same(-0.0f64);

    same(f32::MAX as f64);
    same(f32::MIN as f64);

    same(f32::EPSILON as f64);
    same(f32::MIN_POSITIVE as f64);

    same(1.0);
    same(-32.0);
    same(32432.5);
}

#[test]
fn not_exact() {
    fn not_same(i: f64) {
        let o = f64(f32(i).unwrap());
        let it = unsafe { core::mem::transmute::<f64, u64>(i) };
        let ot = unsafe { core::mem::transmute::<f64, u64>(o) };
        println!("input: {i} {it:0x}");
        println!("output: {o} {ot:0x}");
        assert_ne!(it, ot);
    }

    not_same(32.05);
    not_same(-3443.222);

    not_same(f64::MIN_POSITIVE);
    //not_same(f64::EPSILON);

    dbg!(f64::EPSILON, f32::EPSILON, f64::EPSILON as f32, f32::EPSILON as f64);
}

#[test]
fn overflow() {
    assert_eq!(f32(f64::MAX), Err(Error::Overflow));
    assert_eq!(f32(f32::MAX as f64 * 2.0), Err(Error::Overflow));
}

#[test]
fn underflow() {
    assert_eq!(f32(f64::MIN), Err(Error::Underflow));
    assert_eq!(f32(f32::MIN as f64 * 2.0), Err(Error::Underflow));
}
