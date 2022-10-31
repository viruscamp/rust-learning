//! [7. A Production Unsafe Deque](https://rust-unofficial.github.io/too-many-lists/sixth-final.html)

mod variance;
mod ptr_mut;
mod nll_test;

mod iter;
mod iter_mut;
mod into_iter;
mod cursor;

#[cfg(test)]
mod test;

pub use self::iter::Iter;
pub use self::iter_mut::IterMut;
pub use self::into_iter::IntoIter;
pub use self::cursor::CursorMut;

use core::fmt::Debug;
use core::cmp::Ordering;
use core::hash::Hash;
use core::hash::Hasher;
use core::iter::FromIterator;

use core::ptr::NonNull;
use core::marker::PhantomData;

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
                // to set the back too.
                self.back = Some(new);
            }
            // These things always happen!
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn push_back(&mut self, elem: T) {
        // SAFETY: it's a linked-list, what do you want?
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                back: None,
                front: None,
                elem,
            })));
            if let Some(old) = self.back {
                // Put the new back before the old one
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                // If there's no back, then we're the empty list and need
                // to set the front too.
                self.front = Some(new);
            }
            // These things always happen!
            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            // Only have to do stuff if there is a front node to pop.
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
                    self.back = None;
                }

                self.len -= 1;
                result
                // Box gets implicitly freed here, knows there is no T.
            })
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            // Only have to do stuff if there is a back node to pop.
            self.back.map(|node| {
                // Bring the Box front to life so we can move out its value and
                // Drop it (Box continues to magically understand this for us).
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                // Make the next node into the new back.
                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    // Cleanup its reference to the removed node
                    (*new.as_ptr()).back = None;
                } else {
                    // If the back is now null, then this list is now empty!
                    self.front = None;
                }

                self.len -= 1;
                result
                // Box gets implicitly freed here, knows there is no T.
            })
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        // Oh look it's drop again
        while self.pop_front().is_some() {}
    }

    pub fn iter(&self) -> Iter<T> {
        self.into()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into()
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        self.into()
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        //while self.pop_front().is_some() {}
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

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
fn lifetime_extend() {}
