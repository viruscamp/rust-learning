use std::sync::Arc;
use std::fmt;

#[cfg(test)]
mod test;

pub struct SharedLinkedList<T> {
    head: Link<T>,
    //tail
}

type Link<T> = Option<Arc<Node<T>>>;
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

    pub fn iter(&self) -> Iter<T> {
        Iter(self.head.as_ref())
    }

    pub fn append(&self, elem: T) -> SharedLinkedList<T> {
        Self {
            head: Some(
                Arc::new(Node {
                     elem,
                     next: self.head.clone() 
                })
            )
        }
    }

    pub fn tail(&self) -> SharedLinkedList<T> {
        Self {
            head: self.head.as_ref().and_then(|node| {
                node.next.clone()
            })
        }

    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }
}

impl<T> Drop for SharedLinkedList<T> {
    default fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(boxed_node) = link {
            if let Ok(mut node) = Arc::try_unwrap(boxed_node) {
                link = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T>(Option<&'a Arc<Node<T>>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.next.as_ref();
            &node.elem
        })
    }
}

impl<'a, T> Iter<'a, T> {
    pub fn peek(&self) -> Option<&'a T> {
        self.0.map(|node| {
            &node.elem
        })
    }

    pub fn fork(&self) -> SharedLinkedList<T> {
        SharedLinkedList {
            head: self.0.map(|node| (*node).clone())
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SharedLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}