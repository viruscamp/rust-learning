use std::ops::{Deref, DerefMut};

use super::*;

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // almost same as peek
        self.0.take()
        .map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<'a, T> IterMut<'a, T> {
    pub fn current(&self) -> Option<&T> {
        match self.0 {
            Some(ref node) => Some(&node.elem),
            None => None,
        }
    }
    pub fn current_mut(&mut self) -> Option<&mut T> {
        match self.0 {
            Some(ref mut node) => Some(&mut node.elem),
            None => None,
        }
    }
    pub fn split_after(&mut self) -> Option<LinkedStack<T>> {
        self.0.as_mut().map(|node| {
            LinkedStack{ head: node.next.take() }
        })
    }
}

impl<T> LinkedStack<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut(self.head.as_mut().map(|node| node.as_mut()))
    }
}

//region error
pub struct IterMutError<'a, T>(&'a mut Link<T>); // 这东西我现在写不出来

/*
impl<'a, T> Iterator for IterMutError<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        // 多次独占借用
        self.0.as_mut()
        .map(|node| {
            self.0 = &mut node.next;
            &mut node.elem
        })
    }
}
*/
//endregion
