use super::*;

pub type IntoIter<T> = IntoIterBook<T>;
impl<T> IntoIterator for LinkedStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}
impl<T> LinkedStack<T> {
    pub fn into_iter_book(self) -> IntoIterBook<T> {
        IntoIterBook::new(self)
    }
    pub fn into_iter_verbose(self) -> IntoIterVerbose<T> {
        IntoIterVerbose::new(self)
    }
}

pub struct IntoIterBook<T>(LinkedStack<T>);

impl<T> Iterator for IntoIterBook<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterBook<T> {
    pub fn new(list: LinkedStack<T>) -> Self {
        Self(list)
    }
}

//region verbose

pub struct IntoIterVerbose<T> {
    next: Link<T>,
}

impl<T> Iterator for IntoIterVerbose<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // yes, it's same as pop
        match self.next.take() {
            None => None,
            Some(node) => {
                self.next = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> IntoIterVerbose<T> {
    pub fn new(list: LinkedStack<T>) -> Self {
        Self{ next: list.head }
    }
}

//endregion
