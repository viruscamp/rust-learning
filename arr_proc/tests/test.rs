
#[macro_export]
macro_rules! arr {
    ($ele:expr; $repeat:literal) => {
        arr_proc::arr_proc!([$ele; $repeat])
    };
}

#[test]
fn test_arr() {
    let a: [u32; 2] = arr![1_u32;2];
    dbg!(a);
}