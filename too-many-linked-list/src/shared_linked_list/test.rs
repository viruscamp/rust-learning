use super::*;
#[test]
fn basics() {
    let list = SharedLinkedList::new();
    assert_eq!(list.head(), None);

    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
}

#[test]
fn iter() {
    let list = SharedLinkedList::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(format!("{:?}", &list), format!("{:?}", [3,2,1]))
}

#[test]
fn iter_advance() {
    let list1 = SharedLinkedList::new().append(1).append(2).append(3);
    let mut iter = list1.iter();
    let list2 = loop {
        if let Some(2) = iter.peek() {
            break iter.fork();
        }
        iter.next();
    };
    assert_eq!(format!("{:?}", &list1), format!("{:?}", [3,2,1]));
    assert_eq!(format!("{:?}", &list2), format!("{:?}", [2,1]));
}


/*
list1 -> A ---+
              |
              v
list2 ------> B -> C -> D
              ^
              |
list3 -> X ---+
*/
#[test]
fn complex() {
    let list1 = SharedLinkedList::new()
        .append("D").append("C").append("B").append("A");
    let list2 = list1.tail();
    let list3 = list2.append("X");

    assert_eq!(format!("{:?}", &list1), format!("{:?}", ["A", "B", "C", "D"]));
    assert_eq!(format!("{:?}", &list2), format!("{:?}", ["B", "C", "D"]));
    assert_eq!(format!("{:?}", &list3), format!("{:?}", ["X", "B", "C", "D"]));
}