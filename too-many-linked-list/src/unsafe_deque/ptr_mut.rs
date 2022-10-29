use std::{ptr::null_mut, marker::PhantomData};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    // 这里可以不用 PhantomData, 
    //_boo: PhantomData<T>,
}

/// 不用 Option<NonNull<Node<T>>>; 那么就不是协变,  看看后果
/// `Link<T>`对`Node<T>`不变, 因为`*mut X`对`X`不变
/// 如下函数编译失败, 证明了 `LinkedList<T>` 对 `T` 并非协变, 不能满足需求
/// ```compile_fail
/// # use too_many_linked_list::unsafe_deque::ptr_mut::LinkedList;
/// fn ensure_covariant<'long: 'short, 'short>(list_long: LinkedList<&'long i32>, mut list_short: LinkedList<&'short i32>) {
///     let list_short_new: LinkedList<&'short i32> = list_long; // 证明协变
///     //let list_long_new: LinkedList<&'long i32> = list_short; // 证明逆变
/// }
/// ```
/// 相关编译错误:
///    = note: requirement occurs because of the type `ptr_mut::LinkedList<&i32>`, which makes the generic argument `&i32` invariant
///    = note: the struct `ptr_mut::LinkedList<T>` is invariant over the parameter `T`
type Link<T> = *mut Node<T>;

/// `Node<T>`对`T`协变
struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T, 
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: null_mut(),
            back: null_mut(),
            len: 0,
            //_boo: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elem: T) {
        // SAFETY: it's a linked-list, what do you want?
        unsafe {
            let new = Box::into_raw(Box::new(Node {
                front: null_mut(),
                back: null_mut(),
                elem,
            }));
            if !self.front.is_null() {
                let old = self.front;
                // Put the new front before the old one
                (*old).front = new;
                (*new).back = old;
            } else {
                // If there's no front, then we're the empty list and need 
                // to set the back too. Also here's some integrity checks
                // for testing, in case we mess up.
                debug_assert!(self.back.is_null());
                debug_assert!(self.front.is_null());
                debug_assert!(self.len == 0);
                self.back = new;
            }
            self.front = new;
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // Only have to do stuff if there is a front node to pop.
            // Note that we don't need to mess around with `take` anymore
            // because everything is Copy and there are no dtors that will
            // run if we mess up... right? :) Riiiight? :)))
            if self.front.is_null() {
                None
            } else {
                // Bring the Box back to life so we can move out its value and
                // Drop it (Box continues to magically understand this for us).
                let boxed_node = Box::from_raw(self.front);
                let result = boxed_node.elem;
    
                // Make the next node into the new front.
                self.front = boxed_node.back;
                if !self.front.is_null() {
                    // Cleanup its reference to the removed node
                    (*self.front).front = null_mut();
                } else {
                    // If the front is now null, then this list is now empty!
                    debug_assert!(self.len == 1);
                    self.back = null_mut();
                }
    
                self.len -= 1;
                Some(result)
                // Box gets implicitly freed here, knows there is no T.
            }
        }
    }
}

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
