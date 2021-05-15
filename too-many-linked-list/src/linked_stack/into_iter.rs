use super::*;

pub struct IntoIter<T>(LinkedStack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for LinkedStack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
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

impl<T> LinkedStack<T> {
    pub fn into_iter_verbose(self) -> IntoIterVerbose<T> {
        IntoIterVerbose{ next: self.head }
    }
}

//endregion
