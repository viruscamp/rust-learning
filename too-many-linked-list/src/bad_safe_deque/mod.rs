//! [5. A Bad Safe Deque](http://rust-unofficial.github.io/too-many-lists/fourth-final.html)
//! 用 Arc<RwLock<_>> 替代 Rc<RefCell<_>>

use std::sync::{Arc, RwLock};
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Arc<RwLock<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

mod into_iter;
mod iter;
mod iter_mut;

#[cfg(test)]
mod test;

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_node = Node {
            elem,
            next: None,
            prev: None,
        };
        match self.head.take() {
            Some(node) => {
                new_node.next = Some(node.clone());
                let new_node = Arc::new(RwLock::new(new_node));
                node.write().unwrap().prev = Some(new_node.clone());
                self.head = Some(new_node);
            }
            None => {
                let new_node = Arc::new(RwLock::new(new_node));
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            match node.read().unwrap().next.as_ref() {
                Some(next_node) => {
                    next_node.write().unwrap().prev = None;
                    self.head = Some(next_node.clone());
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            match Arc::try_unwrap(node) {
                Ok(rwlock) => rwlock.into_inner().unwrap().elem,
                Err(_) => panic!(),
            }
        })
    }

    pub fn push_back(&mut self, elem: T) {
        match self.tail.take() {
            Some(node) => {
                let new_node = Arc::new(RwLock::new(Node {
                    elem,
                    next: None,
                    prev: Some(node.clone()),
                }));
                node.write().unwrap().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                let new_node = Arc::new(RwLock::new(Node {
                    elem,
                    next: None,
                    prev: None,
                }));
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            match node.read().unwrap().prev.as_ref() {
                Some(prev_node) => {
                    prev_node.write().unwrap().next = None;
                    self.tail = Some(prev_node.clone());
                }
                None => {
                    self.head = None;
                    self.tail = None;
                }
            }
            match Arc::try_unwrap(node) {
                Ok(rwlock) => rwlock.into_inner().unwrap().elem,
                Err(_) => panic!(),
            }
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        todo!()
    }
    pub fn peek_back(&self) -> Option<&T> {
        todo!()
    }
    pub fn peek_mut_front(&mut self) -> Option<&mut T> {
        todo!()
    }
    pub fn peek_mut_back(&mut self) -> Option<&mut T> {
        todo!()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}
