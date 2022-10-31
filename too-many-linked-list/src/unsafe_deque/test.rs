//! [7.7 Testing](https://rust-unofficial.github.io/too-many-lists/sixth-testing.html)
use super::LinkedList;

fn generate_test() -> LinkedList<i32> {
    list_from(&[0, 1, 2, 3, 4, 5, 6])
}

fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
    v.iter().map(|x| (*x).clone()).collect()
}

#[test]
fn test_basic_front() {
    let mut list = LinkedList::new();

    // Try to break an empty list
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    // Try to break a one item list
    list.push_front(10);
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);

    // Mess around
    list.push_front(10);
    assert_eq!(list.len(), 1);
    list.push_front(20);
    assert_eq!(list.len(), 2);
    list.push_front(30);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(30));
    assert_eq!(list.len(), 2);
    list.push_front(40);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(40));
    assert_eq!(list.len(), 2);
    assert_eq!(list.pop_front(), Some(20));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop_front(), Some(10));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);
    assert_eq!(list.len(), 0);
}

#[test]
fn test_basic() {
    let mut m = LinkedList::new();
    assert_eq!(m.pop_front(), None);
    assert_eq!(m.pop_back(), None);
    assert_eq!(m.pop_front(), None);
    m.push_front(1);
    assert_eq!(m.pop_front(), Some(1));
    m.push_back(2);
    m.push_back(3);
    assert_eq!(m.len(), 2);
    assert_eq!(m.pop_front(), Some(2));
    assert_eq!(m.pop_front(), Some(3));
    assert_eq!(m.len(), 0);
    assert_eq!(m.pop_front(), None);
    m.push_back(1);
    m.push_back(3);
    m.push_back(5);
    m.push_back(7);
    assert_eq!(m.pop_front(), Some(1));

    let mut n = LinkedList::new();
    n.push_front(2);
    n.push_front(3);
    {
        assert_eq!(n.front().unwrap(), &3);
        let x = n.front_mut().unwrap();
        assert_eq!(*x, 3);
        *x = 0;
    }
    {
        assert_eq!(n.back().unwrap(), &2);
        let y = n.back_mut().unwrap();
        assert_eq!(*y, 2);
        *y = 1;
    }
    assert_eq!(n.pop_front(), Some(0));
    assert_eq!(n.pop_front(), Some(1));
}

#[test]
fn test_iterator() {
    let m = generate_test();
    for (i, elt) in m.iter().enumerate() {
        assert_eq!(i as i32, *elt);
    }
    let mut n = LinkedList::new();
    assert_eq!(n.iter().next(), None);
    n.push_front(4);
    let mut it = n.iter();
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next().unwrap(), &4);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_iterator_double_end() {
    let mut n = LinkedList::new();
    assert_eq!(n.iter().next(), None);
    n.push_front(4);
    n.push_front(5);
    n.push_front(6);
    let mut it = n.iter();
    assert_eq!(it.size_hint(), (3, Some(3)));
    assert_eq!(it.next().unwrap(), &6);
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert_eq!(it.next_back().unwrap(), &4);
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next_back().unwrap(), &5);
    assert_eq!(it.next_back(), None);
    assert_eq!(it.next(), None);
}

#[test]
fn test_rev_iter() {
    let m = generate_test();
    for (i, elt) in m.iter().rev().enumerate() {
        assert_eq!(6 - i as i32, *elt);
    }
    let mut n = LinkedList::new();
    assert_eq!(n.iter().rev().next(), None);
    n.push_front(4);
    let mut it = n.iter().rev();
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(it.next().unwrap(), &4);
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert_eq!(it.next(), None);
}

#[test]
fn test_mut_iter() {
    let mut m = generate_test();
    let mut len = m.len();
    for (i, elt) in m.iter_mut().enumerate() {
        assert_eq!(i as i32, *elt);
        len -= 1;
    }
    assert_eq!(len, 0);
    let mut n = LinkedList::new();
    assert!(n.iter_mut().next().is_none());
    n.push_front(4);
    n.push_back(5);
    let mut it = n.iter_mut();
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert!(it.next().is_some());
    assert!(it.next().is_some());
    assert_eq!(it.size_hint(), (0, Some(0)));
    assert!(it.next().is_none());
}

#[test]
fn test_iterator_mut_double_end() {
    let mut n = LinkedList::new();
    assert!(n.iter_mut().next_back().is_none());
    n.push_front(4);
    n.push_front(5);
    n.push_front(6);
    let mut it = n.iter_mut();
    assert_eq!(it.size_hint(), (3, Some(3)));
    assert_eq!(*it.next().unwrap(), 6);
    assert_eq!(it.size_hint(), (2, Some(2)));
    assert_eq!(*it.next_back().unwrap(), 4);
    assert_eq!(it.size_hint(), (1, Some(1)));
    assert_eq!(*it.next_back().unwrap(), 5);
    assert!(it.next_back().is_none());
    assert!(it.next().is_none());
}

#[test]
fn test_eq() {
    let mut n: LinkedList<u8> = list_from(&[]);
    let mut m = list_from(&[]);
    assert!(n == m);
    n.push_front(1);
    assert!(n != m);
    m.push_back(1);
    assert!(n == m);

    let n = list_from(&[2, 3, 4]);
    let m = list_from(&[1, 2, 3]);
    assert!(n != m);
}

#[test]
fn test_ord() {
    let n = list_from(&[]);
    let m = list_from(&[1, 2, 3]);
    assert!(n < m);
    assert!(m > n);
    assert!(n <= n);
    assert!(n >= n);
}

#[test]
fn test_ord_nan() {
    let nan = 0.0f64 / 0.0;
    let n = list_from(&[nan]);
    let m = list_from(&[nan]);
    assert!(!(n < m));
    assert!(!(n > m));
    assert!(!(n <= m));
    assert!(!(n >= m));

    let n = list_from(&[nan]);
    let one = list_from(&[1.0f64]);
    assert!(!(n < one));
    assert!(!(n > one));
    assert!(!(n <= one));
    assert!(!(n >= one));

    let u = list_from(&[1.0f64, 2.0, nan]);
    let v = list_from(&[1.0f64, 2.0, 3.0]);
    assert!(!(u < v));
    assert!(!(u > v));
    assert!(!(u <= v));
    assert!(!(u >= v));

    let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
    let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
    assert!(!(s < t));
    assert!(s > one);
    assert!(!(s <= one));
    assert!(s >= one);
}

#[test]
fn test_debug() {
    let list: LinkedList<i32> = (0..10).collect();
    assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

    let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
        .iter()
        .copied()
        .collect();
    assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
}

#[test]
fn test_hashmap() {
    // Check that HashMap works with this as a key

    let list1: LinkedList<i32> = (0..10).collect();
    let list2: LinkedList<i32> = (1..11).collect();
    let mut map = std::collections::HashMap::new();

    assert_eq!(map.insert(list1.clone(), "list1"), None);
    assert_eq!(map.insert(list2.clone(), "list2"), None);

    assert_eq!(map.len(), 2);

    assert_eq!(map.get(&list1), Some(&"list1"));
    assert_eq!(map.get(&list2), Some(&"list2"));

    assert_eq!(map.remove(&list1), Some("list1"));
    assert_eq!(map.remove(&list2), Some("list2"));

    assert!(map.is_empty());
}

#[test]
fn test_cursor_move_peek() {
    let mut m: LinkedList<u32> = LinkedList::new();
    m.extend([1, 2, 3, 4, 5, 6]);
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    assert_eq!(cursor.current(), Some(&mut 1));
    assert_eq!(cursor.peek_next(), Some(&mut 2));
    assert_eq!(cursor.peek_prev(), None);
    assert_eq!(cursor.index(), Some(0));
    cursor.move_prev();
    assert_eq!(cursor.current(), None);
    assert_eq!(cursor.peek_next(), Some(&mut 1));
    assert_eq!(cursor.peek_prev(), Some(&mut 6));
    assert_eq!(cursor.index(), None);
    cursor.move_next();
    cursor.move_next();
    assert_eq!(cursor.current(), Some(&mut 2));
    assert_eq!(cursor.peek_next(), Some(&mut 3));
    assert_eq!(cursor.peek_prev(), Some(&mut 1));
    assert_eq!(cursor.index(), Some(1));

    let mut cursor = m.cursor_mut();
    cursor.move_prev();
    assert_eq!(cursor.current(), Some(&mut 6));
    assert_eq!(cursor.peek_next(), None);
    assert_eq!(cursor.peek_prev(), Some(&mut 5));
    assert_eq!(cursor.index(), Some(5));
    cursor.move_next();
    assert_eq!(cursor.current(), None);
    assert_eq!(cursor.peek_next(), Some(&mut 1));
    assert_eq!(cursor.peek_prev(), Some(&mut 6));
    assert_eq!(cursor.index(), None);
    cursor.move_prev();
    cursor.move_prev();
    assert_eq!(cursor.current(), Some(&mut 5));
    assert_eq!(cursor.peek_next(), Some(&mut 6));
    assert_eq!(cursor.peek_prev(), Some(&mut 4));
    assert_eq!(cursor.index(), Some(4));
}

#[test]
fn test_cursor_mut_insert() {
    let mut m: LinkedList<u32> = LinkedList::new();
    m.extend([1, 2, 3, 4, 5, 6]);
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    cursor.splice_before(Some(7).into_iter().collect());
    cursor.splice_after(Some(8).into_iter().collect());
    // check_links(&m);
    assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[7, 1, 8, 2, 3, 4, 5, 6]
    );
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    cursor.move_prev();
    cursor.splice_before(Some(9).into_iter().collect());
    cursor.splice_after(Some(10).into_iter().collect());
    check_links(&m);
    assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[10, 7, 1, 8, 2, 3, 4, 5, 6, 9]
    );

    /* remove_current not impl'd
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    cursor.move_prev();
    assert_eq!(cursor.remove_current(), None);
    cursor.move_next();
    cursor.move_next();
    assert_eq!(cursor.remove_current(), Some(7));
    cursor.move_prev();
    cursor.move_prev();
    cursor.move_prev();
    assert_eq!(cursor.remove_current(), Some(9));
    cursor.move_next();
    assert_eq!(cursor.remove_current(), Some(10));
    check_links(&m);
    assert_eq!(m.iter().cloned().collect::<Vec<_>>(), &[1, 8, 2, 3, 4, 5, 6]);
    */

    let mut m: LinkedList<u32> = LinkedList::new();
    m.extend([1, 8, 2, 3, 4, 5, 6]);
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    let mut p: LinkedList<u32> = LinkedList::new();
    p.extend([100, 101, 102, 103]);
    let mut q: LinkedList<u32> = LinkedList::new();
    q.extend([200, 201, 202, 203]);
    cursor.splice_after(p);
    cursor.splice_before(q);
    check_links(&m);
    assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[200, 201, 202, 203, 1, 100, 101, 102, 103, 8, 2, 3, 4, 5, 6]
    );
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    cursor.move_prev();
    let tmp = cursor.split_before();
    assert_eq!(m.into_iter().collect::<Vec<_>>(), &[]);
    m = tmp;
    let mut cursor = m.cursor_mut();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    let tmp = cursor.split_after();
    assert_eq!(
        tmp.into_iter().collect::<Vec<_>>(),
        &[102, 103, 8, 2, 3, 4, 5, 6]
    );
    check_links(&m);
    assert_eq!(
        m.iter().cloned().collect::<Vec<_>>(),
        &[200, 201, 202, 203, 1, 100, 101]
    );
}

fn check_links<T: Eq + std::fmt::Debug>(list: &LinkedList<T>) {
    let from_front: Vec<_> = list.iter().collect();
    let from_back: Vec<_> = list.iter().rev().collect();
    let re_reved: Vec<_> = from_back.into_iter().rev().collect();

    assert_eq!(from_front, re_reved);
}
