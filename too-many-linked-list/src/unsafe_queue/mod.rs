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
    tail: WeakLink<T>,
}

type Link<T> = Option<OwnerLink<T>>;

//type WeakLink<T> = *mut Node<T>; // 无法使 `List<T>` 对 `T` 协变
type WeakLink<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

trait OwnerLinkExt<T> where Self: Sized {
    fn create(node: Node<T>) -> Self;
    fn into_node(self) -> Node<T>;
    fn ref_node(&self) -> &Node<T>;
    fn mut_node(&mut self) -> &mut Node<T>;
    fn as_weak_link(&self) -> WeakLink<T>;
}

#[cfg(box_link)]
type OwnerLink<T> = Box<Node<T>>;
#[cfg(box_link)]
impl<T> OwnerLinkExt<T> for Box<Node<T>> {
    #[inline(always)]
    fn create(node: Node<T>) -> Self {
        Box::new(node)
    }

    #[inline(always)]
    fn into_node(self) -> Node<T> {
        *self
    }

    #[inline(always)]
    fn ref_node(&self) -> &Node<T> {
        self.as_ref()
    }

    #[inline(always)]
    fn mut_node(&mut self) -> &mut Node<T> {
        self.as_mut()
    }

    #[inline(always)]
    fn as_weak_link(&self) -> WeakLink<T> {
        Some(unsafe {
            NonNull::new_unchecked(self.as_ref() as *const _ as *mut _)
        })
    }
}

#[cfg(not(box_link))]
type OwnerLink<T> = NonNull<Node<T>>;
#[cfg(not(box_link))]
impl<T> OwnerLinkExt<T> for NonNull<Node<T>> {
    #[inline(always)]
    fn create(node: Node<T>) -> Self {
        let b = Box::new(node);
        unsafe { NonNull::new_unchecked(Box::leak(b)) }
    }

    #[inline(always)]
    fn into_node(self) -> Node<T> {
        let b = unsafe { Box::from_raw(self.as_ptr()) };
        *b
    }

    #[inline(always)]
    fn ref_node(&self) -> &Node<T> {
        unsafe { self.as_ref() }
    }

    #[inline(always)]
    fn mut_node(&mut self) -> &mut Node<T> {
        unsafe { self.as_mut() }
    }
    
    #[inline(always)]
    fn as_weak_link(&self) -> WeakLink<T> {
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
            let old_head = old_head.into_node();
            if old_head.next.is_none() {
                self.tail = None;
            }
            self.head = old_head.next;
            old_head.elem
        })
    }
    pub fn push_front(&mut self, elem: T) {
        let new_node = OwnerLink::create(Node { elem, next: self.head.take() });
        let new_node_ptr = new_node.as_weak_link();
        self.head = Some(new_node);
        if self.tail.is_none() {
            self.tail = new_node_ptr;
        }
    }
    /// push to tail
    pub fn push_back(&mut self, elem: T) {
        let new_node = OwnerLink::create(Node { elem, next: None });
        let new_tail = new_node.as_weak_link();
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
            &node.ref_node().elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.mut_node().elem
        })
    }
}

/// 反转
pub fn reverse<T>(ls: &mut List<T>) {
    let mut oldhead: Link<T> = ls.head.take();
    let tail: WeakLink<T> = oldhead.as_ref()
        .and_then(OwnerLinkExt::as_weak_link);
    let mut head = None;
    while let Some(mut node) = oldhead.take() {
        oldhead = replace(&mut node.mut_node().next, head.take());
        head = Some(node);
    }
    ls.head = head;
    ls.tail = tail;
}
