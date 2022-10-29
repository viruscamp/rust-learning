//! [7. A Production Unsafe Deque](https://rust-unofficial.github.io/too-many-lists/sixth-final.html)

mod variance;
mod ptr_mut;
mod nll_test;

use std::ptr::NonNull;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // We semantically store values of T by-value. 非必要
    _boo: PhantomData<T>,
}

/// 对`Node<T>`协变, 因为 `NonNull<X>` 内部是 ``*const X``, 对`X`协变
/// 如下函数编译成功证明了 `LinkedList<T>` 对 `T` 协变
/// ```no_run
/// # use too_many_linked_list::unsafe_deque::LinkedList;
/// fn ensure_covariant<'long: 'short, 'short>(list_long: LinkedList<&'long i32>, mut list_short: LinkedList<&'short i32>) {
///     let list_short_new: LinkedList<&'short i32> = list_long; // 证明协变
///     //let list_long_new: LinkedList<&'long i32> = list_short; // 证明逆变
/// }
/// ```
type Link<T> = Option<NonNull<Node<T>>>;

#[derive(Debug, Copy, Clone)]
struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elem: T) {
        // SAFETY: it's a linked-list, what do you want?
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.front {
                // Put the new front before the old one
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                // If there's no front, then we're the empty list and need 
                // to set the back too. Also here's some integrity checks
                // for testing, in case we mess up.
                debug_assert!(self.back.is_none());
                debug_assert!(self.front.is_none());
                debug_assert!(self.len == 0);
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // Only have to do stuff if there is a front node to pop.
            // Note that we don't need to mess around with `take` anymore
            // because everything is Copy and there are no dtors that will
            // run if we mess up... right? :) Riiiight? :)))
            self.front.map(|node| {
                // Bring the Box back to life so we can move out its value and
                // Drop it (Box continues to magically understand this for us).
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;
    
                // Make the next node into the new front.
                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    // Cleanup its reference to the removed node
                    (*new.as_ptr()).front = None;
                } else {
                    // If the front is now null, then this list is now empty!
                    debug_assert!(self.len == 1);
                    self.back = None;
                }
    
                self.len -= 1;
                result
                // Box gets implicitly freed here, knows there is no T.
            })
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(doctest)]
/// ```compile_fail
/// '_long: {
///     let a = 3;
///     let mut l = LinkedList::new();
///     l.push_front(&a); // make sure l is `LinkedList<&'_long i32>`
///     '_short: {
///         let b = 4;
///         l.push_front(&b); // push a `&'_short i32` to `LinkedList<&'_long i32>`
///     }
///     let l = l; // force extend the lifetime of `l`
/// }
/// ```
fn lifetime_contravariant() {}

#[cfg(test)]
mod test {
    use super::LinkedList;

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
}
