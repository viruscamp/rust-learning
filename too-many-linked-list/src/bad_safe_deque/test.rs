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
