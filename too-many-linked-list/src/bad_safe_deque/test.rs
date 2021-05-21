use super::*;

#[test]
fn basics() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop_front(), None);

    // Populate list
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push_front(4);
    list.push_front(5);

    // Check normal removal
    assert_eq!(list.pop_front(), Some(5));
    assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_push_pop_front() {
    let mut list = List::new();
    assert_eq!(list.pop_front(), None);

    list.push_front(8);
    list.push_front(3);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(8));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_push_pop_back() {
    let mut list = List::new();
    assert_eq!(list.pop_back(), None);

    list.push_back(8);
    list.push_back(3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(8));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_push_front_pop_back() {
    let mut list = List::new();
    assert_eq!(list.pop_back(), None);

    list.push_front(8);
    list.push_front(3);
    assert_eq!(list.pop_back(), Some(8));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn peek() {
    let mut list = List::new();
    assert!(list.peek_front().is_none());
    assert!(list.peek_back().is_none());
    assert!(list.peek_mut_front().is_none());
    assert!(list.peek_mut_back().is_none());

    list.push_front(1); list.push_front(2); list.push_front(3);

    assert_eq!(&*list.peek_front().unwrap(), &3);
    assert_eq!(&mut *list.peek_mut_front().unwrap(), &mut 3);
    assert_eq!(&*list.peek_back().unwrap(), &1);
    assert_eq!(&mut *list.peek_mut_back().unwrap(), &mut 1);
}

#[test]
fn into_iter() {
    let mut list = List::new();
    list.push_front(1); list.push_front(2); list.push_front(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);
}