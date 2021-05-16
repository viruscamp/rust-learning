use std::fmt::Display;

fn _match_some<T: Display>(ot: Option<T>) -> Option<String> {
    match ot {
        None => None,
        Some(t) => Some(format!("{}", t)),
    }
}
// match Some(t) 转成 map
fn _map<T: Display>(ot: Option<T>) -> Option<String> {
    ot.map(|t| format!("{}", t))
}

// 大部分情况下都用 map ,只有 map 闭包返回值已经是 Option 时用 and_then
// 当返回类型错误, 多了一层Option时, map 改 and_then
fn _and_then<T: Display>(ot: Option<T>) -> Option<String> {
    ot.and_then(|t| Some(format!("{}", t)))
}

fn _match_some_ref<T: Display>(ot: &Option<T>) -> Option<&T> {
    match ot {
        None => None,
        Some(ref t) => Some(t),
    }
}
// match Some(ref t) 转成 .as_ref().map()
fn _map_ref<T: Display>(ot: &Option<T>) -> Option<&T> {
    ot.as_ref().map(|t| t)
}

fn _match_some_ref_mut<T: Display>(ot: &mut Option<T>) -> Option<&mut T> {
    match ot {
        None => None,
        Some(ref mut t) => Some(t),
    }
}
// match Some(ref mut t) 转成 .as_mut().map()
fn _map_ref_mut<T: Display>(ot: &mut Option<T>) -> Option<&mut T> {
    ot.as_mut().map(|t| t)
}

fn _match_none<T: Default>(ot: Option<T>) -> Option<T> {
    match ot {
        None => Some(T::default()),
        Some(t) => Some(t),
    }
}
// match None => 值 转成 or(值) 少用
fn _or<T: Default>(ot: Option<T>) -> Option<T> {
    ot.or(Some(T::default()))
}
// match None => 取值 转成 or(取值闭包) 优先使用 这是 lazy 的
fn _or_else<T: Default>(ot: Option<T>) -> Option<T> {
    ot.or_else(|| {
        println!("hello");
        Some(T::default())
    })
}

// map_or 对应 None => 值 Some(t) => 转换
// map_or_else 对应 None => 取值 Some(t) => 转换
// 这两个建议用 match

// as_ref as_mut
// 对应 match Some 部分有操作的
// map map_or map_or_else and_then
