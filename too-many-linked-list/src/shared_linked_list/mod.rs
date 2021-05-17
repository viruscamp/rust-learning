use std::sync::{Arc, RwLock};

pub struct SharedLinkedList<T> {
    head: Link<T>,
    //tail
}

type Link<T> = Option<Arc<RwLock<Node<T>>>>;
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    //prev
}


impl<T> SharedLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    pub fn push(&mut self) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
}