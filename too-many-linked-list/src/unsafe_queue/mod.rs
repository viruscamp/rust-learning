//! [6. An Unsafe Queue](http://rust-unofficial.github.io/too-many-lists/fifth-final.html)

use core::ptr::NonNull;

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
type Link<T> = Option<Box<Node<T>>>;

//type WeakLink<T> = *mut Node<T>; // 无法使 `List<T>` 对 `T` 协变
type WeakLink<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{ head: None, tail: None }
    }
    /// pop from head
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let old_head = *old_head;
            if old_head.next.is_none() {
                self.tail = None;
            }
            self.head = old_head.next;
            old_head.elem
        })
    }
    pub fn push_front(&mut self, elem: T) {
        let mut new_node = Box::new(Node { elem, next: self.head.take() });
        let new_node_ptr = NonNull::new(new_node.as_mut());
        self.head = Some(new_node);
        if self.tail.is_none() {
            self.tail = new_node_ptr;
        }
    }
    /// push to tail
    pub fn push_back(&mut self, elem: T) {
        let mut new_node = Box::new(Node { elem, next: None });
        let tail_ptr = NonNull::new(new_node.as_mut());
        match self.tail {
            None => {
                self.head = Some(new_node);
            }
            Some(ref mut ptr) => {
                let old_tail = unsafe { ptr.as_mut() };
                old_tail.next = Some(new_node);
            }
        }
        self.tail = tail_ptr;
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

/// 反转
pub fn reverse<T>(ls: &mut List<T>) {
    let mut oldhead = ls.head.take();
    let tail = oldhead.as_deref_mut()
        .and_then(|x| NonNull::new(x));
    let mut head = None;
    while let Some(mut node) = oldhead.take() {
        oldhead = node.next;
        node.next = head.take();
        head = Some(node);
    }
    ls.head = head;
    ls.tail = tail;
}
