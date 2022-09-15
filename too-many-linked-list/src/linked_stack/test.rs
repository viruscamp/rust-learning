use std::borrow::Borrow;

use super::*;

type List<T> = LinkedStack<T>;
#[test]
fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

macro_rules! stack_info {
    () => ({
        let mut level = 0;
        let mut sp = ptr::null_mut();
        backtrace::trace(|frame| {
            if sp == ptr::null_mut() {
                sp = frame.sp();
            }
            level += 1;
            true
        });
        (level, sp)
    })
}

#[test]
#[should_panic]
#[ignore]
fn recursion_drop_stackoverflow() {
    struct I(i32);
    impl LinkedStackRecursionDrop for I {}

    fn test(n: usize) {
        let mut list = List::new();
        for i in 0..n {
            list.push(I(i as i32))
        }
        println!("List size={} created ok", n);
        drop(list);
        println!("List size={} dropped ok", n);
    }

    let mut n: usize = 512;
    loop {
        test(n);
        n = n * 2;
    }
}

#[test]
fn recursion_drop() {
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::ptr;

    thread_local!(static VEC_SP_DROP: RefCell<Vec<(i32, *mut c_void)>> = RefCell::new(Vec::new()));

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct I(i32);
    impl LinkedStackRecursionDrop for I {}
    // 使用递归 drop 打断点看 call stack 增加
    // 用 backtrace::trace 拿到 drop 时的 stack pointer , 应该不同, 栈增长, sp递减
    impl Drop for I {
        fn drop(&mut self) {
            let (level, sp) = stack_info!();
            VEC_SP_DROP.with(|f| {
                (*f.borrow_mut()).push((level, sp));
            });

            println!("drop {:?} stack_level={} sp={:?}", self, level, sp);
        }
    }

    let mut list = List::new();
    list.push(I(1));
    list.push(I(2));
    list.push(I(3));
    drop(list);

    VEC_SP_DROP.with(|f| {
        let vec_sp = &*f.borrow();
        assert_eq!(vec_sp.len(), 3);
        assert_ne!(vec_sp[0], vec_sp[1]);
        assert_ne!(vec_sp[1], vec_sp[2]);
    });
}

#[test]
fn non_recursion_drop() {
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::ptr;

    thread_local!(static VEC_SP_DROP: RefCell<Vec<(i32, *mut c_void)>> = RefCell::new(Vec::new()));

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct I(i32);
    // 强制使用非递归 drop 打断点看 call stack 不变
    // 用 backtrace::trace 拿到 drop 时的 stack pointer , 应该相同
    impl Drop for I {
        fn drop(&mut self) {
            let (level, sp) = stack_info!();
            VEC_SP_DROP.with(|f| {
                (*f.borrow_mut()).push((level, sp));
            });
            println!("drop {:?} stack_level={} sp={:?}", self, level, sp);
        }
    }

    let mut list = List::new();
    list.push(I(1));
    list.push(I(2));
    list.push(I(3));
    drop(list);

    VEC_SP_DROP.with(|f| {
        let vec_sp = &*f.borrow();
        assert_eq!(vec_sp.len(), 3);
        assert_eq!(vec_sp[0], vec_sp[1]);
        assert_eq!(vec_sp[1], vec_sp[2]);
    });
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1); list.push(2); list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| {
        *value = 42
    });

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop(), Some(42));
}

#[test]
fn test_reverse() {
    let mut list = List::new();
    reverse(&mut list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&0;0]);

    list.push(3);
    reverse(&mut list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&3]);

    list.push(2);
    list.push(1);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    reverse(&mut list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&3, &2, &1]);
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter();

    assert_eq!(iter.peek(), Some(&3));
    assert_eq!(iter.peek(), Some(&3));
    assert_eq!(iter.next(), Some(&3));

    assert_eq!(iter.peek(), Some(&2));
    assert_eq!(iter.next(), Some(&2));

    assert_eq!(iter.peek(), Some(&1));
    assert_eq!(iter.peek(), Some(&1));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));

    assert_eq!(iter.peek(), Some(&1));
    assert_eq!(iter.peek_mut(), Some(&mut 1));
    if let Some(x) = iter.peek_mut() {
        *x = 4;
    }

    assert_eq!(iter.next(), Some(&mut 4));

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek_mut(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_insert_one_after() {
    let mut list = List::new();

    let mut iter = list.iter_mut_book();
    assert_eq!(iter.insert_after(3), None); // 失败

    list.push(1);
    let mut iter = list.iter_mut_book();
    assert_eq!(iter.insert_after(3), Some(())); // 成功
    assert_eq!(iter.peek(), Some(&1));

    assert_eq!(list.iter().collect::<Vec<_>>(), [&1, &3]);

    let mut iter = list.iter_mut_book();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_split_after() {
    let mut list1 = List::<i32>::new();
    let mut iter = list1.iter_mut_book();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1;0]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut_book();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut_book();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    
    let mut list1 = List::<i32>::new();
    list1.push(1);list1.push(2);
    let mut iter = list1.iter_mut_book();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&2]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1]);
}


#[test]
fn iter_mut_insert_at() {
    let mut list = List::new();

    let mut iter = list.iter_mut_my();
    iter.insert_at(3);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&3]);

    list.push(1);
    let mut iter = list.iter_mut_my();
    iter.insert_at(4);
    assert_eq!(iter.peek(), Some(&4));

    assert_eq!(list.iter().collect::<Vec<_>>(), [&4, &1, &3]);

    let mut iter = list.iter_mut_my();
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);

    iter.insert_at(8);
    assert_eq!(iter.peek(), Some(&8));
}

#[test]
fn iter_mut_split_at() {
    let mut list1 = List::<i32>::new();
    let mut iter = list1.iter_mut_my();
    let mut list2 = iter.split_at();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1;0]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut_my();
    let mut list2 = iter.split_at();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1;0]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut_my();
    iter.next();
    let mut list2 = iter.split_at();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    
    let mut list1 = List::<i32>::new();
    list1.push(1);list1.push(2);
    let mut iter = list1.iter_mut_my();
    iter.next();
    let mut list2 = iter.split_at();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&2]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1]);
}