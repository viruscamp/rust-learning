use std::ptr;

use super::*;

fn valid<T>(list: &List<T>) {
    match list.head.as_ref() {
        None => assert_eq!(list.tail, ptr::null_mut()),
        Some(mut node) => {
            while let Some(next_node) = node.next.as_ref() {
                node = next_node;
            }
            let node_ptr: *const Node<T> = &**node;
            assert_eq!(list.tail as *const _, node_ptr);
        },
    }
}

#[test]
fn basics() {
    let mut list = List::new();
    valid(&list);

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_back(1);
    valid(&list);
    list.push_back(2);
    list.push_back(3);
    valid(&list);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(1));
    valid(&list);
    assert_eq!(list.pop_front(), Some(2));
    valid(&list);

    // Push some more just to make sure nothing's corrupted
    list.push_back(4);
    list.push_back(5);
    valid(&list);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), None);
    valid(&list);
}

#[test]
fn peek() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    valid(&list);
    list.push_back(1); list.push_back(2); list.push_back(3);
    valid(&list);

    assert_eq!(list.peek(), Some(&1));
    assert_eq!(list.peek_mut(), Some(&mut 1));

    list.peek_mut().map(|value| {
        *value = 42
    });
    valid(&list);

    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.pop_front(), Some(42));
    valid(&list);

    assert_eq!(list.pop_front(), Some(2));
    valid(&list);
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push_back(1); list.push_back(2); list.push_back(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter() {
    let mut list = List::new();
    list.push_back(1); list.push_back(2); list.push_back(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push_back(1); list.push_back(2); list.push_back(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));

    assert_eq!(iter.peek(), Some(&3));
    assert_eq!(iter.peek_mut(), Some(&mut 3));
    if let Some(x) = iter.peek_mut() {
        *x = 4;
    }

    assert_eq!(iter.next(), Some(&mut 4));

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek_mut(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_reverse() {
    let mut list = List::new();
    reverse(&mut list);
    valid(&list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&0;0]);

    list.push_front(3);
    reverse(&mut list);
    valid(&list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&3]);

    list.push_front(2);
    list.push_front(1);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&1, &2, &3]);
    reverse(&mut list);
    valid(&list);
    assert_eq!(list.iter().collect::<Vec<_>>(), [&3, &2, &1]);
}
