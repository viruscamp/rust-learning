use std::ptr::null_mut;

mod iter;
mod iter_mut;
mod into_iter;
#[cfg(test)]
mod tests;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List{ head: None, tail: null_mut() }
    }
    /// pop from head
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let old_head = *old_head;
            if old_head.next.is_none() {
                self.tail = null_mut();
            }
            self.head = old_head.next;
            old_head.elem
        })
    }
    /// push to tail
    pub fn push(&mut self, elem: T) {
        let mut new_node = Box::new(Node { elem, next: None });
        let tail_ptr: *mut _ = &mut *new_node;
        if self.tail.is_null() {
            self.head = Some(new_node);
        } else {
            let old_tail = unsafe {&mut *self.tail};
            old_tail.next = Some(new_node);
        };
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
