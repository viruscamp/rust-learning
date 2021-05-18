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
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
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

    let mut iter = list.iter_mut();
    assert_eq!(iter.insert_after(3), None); // 失败

    list.push(1);
    let mut iter = list.iter_mut();
    assert_eq!(iter.insert_after(3), Some(())); // 成功
    assert_eq!(iter.peek(), Some(&1));

    assert_eq!(list.iter().collect::<Vec<_>>(), [&1, &3]);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_split_after() {
    let mut list1 = List::<i32>::new();
    let mut iter = list1.iter_mut();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1;0]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    let mut list1 = List::<i32>::new();
    list1.push(1);
    let mut iter = list1.iter_mut();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&1]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1;0]);

    
    let mut list1 = List::<i32>::new();
    list1.push(1);list1.push(2);
    let mut iter = list1.iter_mut();
    let mut list2 = iter.split_after();
    assert_eq!(list1.iter().collect::<Vec<_>>(), [&2]);
    assert_eq!(list2.iter().collect::<Vec<_>>(), [&1]);
}