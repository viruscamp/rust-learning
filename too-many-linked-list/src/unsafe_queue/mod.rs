//! [6. An Unsafe Queue](http://rust-unofficial.github.io/too-many-lists/fifth-final.html)

use core::ptr::NonNull;
use core::mem::replace;

mod iter;
mod iter_mut;
mod into_iter;
#[cfg(test)]
mod test;

/// 如下函数编译成功证明了 `List<T>` 对 `T` 协变
/// ```no_run
/// # use too_many_linked_list::unsafe_queue::List;
/// fn ensure_covariant<'long: 'short, 'short>(list_long: List<&'long i32>, mut list_short: List<&'short i32>) {
///     let list_short_new: List<&'short i32> = list_long; // 证明协变
///     //let list_long_new: List<&'long i32> = list_short; // 证明逆变
/// }
/// ```
pub struct List<T> {
    head: Link<T>,
    tail: WeakPtr<Node<T>>,
}

type Link<T> = Option<HeapOwner<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

//type WeakPtr<T> = *mut T; // 无法使 `List<T>` 对 `T` 协变
type WeakPtr<T> = Option<NonNull<T>>;

trait HeapOwnerExt<T> where Self: Sized {
    fn create(value: T) -> Self;
    fn into_value(self) -> T;
    fn as_ref_value(&self) -> &T;
    fn as_mut_value(&mut self) -> &mut T;
    fn as_weak_ptr(&self) -> WeakPtr<T>;
}

#[cfg(feature = "heap-owner-box")]
type HeapOwner<T> = Box<T>;
#[cfg(feature = "heap-owner-box")]
impl<T> HeapOwnerExt<T> for Box<T> {
    #[inline(always)]
    fn create(node: T) -> Self {
        Box::new(node)
    }

    #[inline(always)]
    fn into_value(self) -> T {
        *self
    }

    #[inline(always)]
    fn as_ref_value(&self) -> &T {
        self.as_ref()
    }

    #[inline(always)]
    fn as_mut_value(&mut self) -> &mut T {
        self.as_mut()
    }

    #[inline(always)]
    fn as_weak_ptr(&self) -> WeakPtr<T> {
        Some(unsafe {
            NonNull::new_unchecked(self.as_ref() as *const _ as *mut _)
        })
    }
}

#[cfg(not(feature = "heap-owner-box"))]
type HeapOwner<T> = NonNull<T>;
#[cfg(not(feature = "heap-owner-box"))]
impl<T> HeapOwnerExt<T> for NonNull<T> {
    #[inline(always)]
    fn create(value: T) -> Self {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(value))) }
    }

    #[inline(always)]
    fn into_value(self) -> T {
        *unsafe { Box::from_raw(self.as_ptr()) }
    }

    #[inline(always)]
    fn as_ref_value(&self) -> &T {
        unsafe { self.as_ref() }
    }

    #[inline(always)]
    fn as_mut_value(&mut self) -> &mut T {
        unsafe { self.as_mut() }
    }
    
    #[inline(always)]
    fn as_weak_ptr(&self) -> WeakPtr<T> {
        Some(*self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Pop until we have to stop
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{ head: None, tail: None }
    }
    /// pop from head
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let old_head = old_head.into_value();
            if old_head.next.is_none() {
                self.tail = None;
            }
            self.head = old_head.next;
            old_head.elem
        })
    }
    pub fn push_front(&mut self, elem: T) {
        let new_node = HeapOwner::create(Node { elem, next: self.head.take() });
        let new_node_ptr = new_node.as_weak_ptr();
        self.head = Some(new_node);
        if self.tail.is_none() {
            self.tail = new_node_ptr;
        }
    }
    /// push to tail
    pub fn push_back(&mut self, elem: T) {
        let new_node = HeapOwner::create(Node { elem, next: None });
        let new_tail = new_node.as_weak_ptr();
        match self.tail {
            None => {
                self.head = Some(new_node);
            }
            Some(mut node_ptr) => {
                let node = unsafe { node_ptr.as_mut() };
                node.next = Some(new_node);
            }
        }
        self.tail = new_tail;
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.as_ref_value().elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.as_mut_value().elem
        })
    }
}

/// 原地反转
pub fn reverse<T>(ls: &mut List<T>) {
    let newtail = ls.head.as_ref()
        .and_then(HeapOwnerExt::as_weak_ptr);
    let mut next = ls.head.take();
    let mut newhead = None;
    while let Some(mut node) = next.take() {
        //next = node.as_mut_value().next.take();
        //node.as_mut_value().next = newhead.take();
        //newhead = Some(node);

        next = replace(&mut node.as_mut_value().next, newhead.take());
        newhead = Some(node);
    }
    ls.head = newhead;
    ls.tail = newtail;
}
