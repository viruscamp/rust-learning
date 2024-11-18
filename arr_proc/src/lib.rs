use proc_macro::TokenStream;
use quote::quote;
use std::ops::Deref;
use syn::*;

#[proc_macro]
pub fn arr_proc(input: TokenStream) -> TokenStream {
    let repeat_expr: ExprRepeat = parse(input)
        .expect("Like arr!([Test::default(); 16])");

    let mut len = None;
    // 获取表达式中的长度信息并转为usize
    if let Expr::Group(expr_group) = repeat_expr.len.deref() {
        if let Expr::Lit(expr_lit) = &*expr_group.expr.deref() {
            if let Lit::Int(int_lit) = &expr_lit.lit {
                len = Some(int_lit.base10_parse::<usize>().expect("Failed to parse integer literal"));
            }
        }
    }
    let len = len.expect("unexpected struct");
    // 解析并拼接成数组
    let _expr = repeat_expr.expr;
    // 1.生成数组的集合
    let mut _all = quote!();
    for _i in 0..len {
        // 2.将每个元素向数组中追加
        if let Expr::Path(path) = _expr.as_ref() {
            // 如果是element宏的情况会调用element宏并传入index
            let _mac_name = &path;
            _all = quote! { #_all #_mac_name!(#_i, capacity, default_length_per_shard), };
        } else {
            _all = quote! { #_all #_expr, };
        }
    }
    // 3.加上中括号
    let arr = quote! { [ #_all ] };
    return arr.into();
}
