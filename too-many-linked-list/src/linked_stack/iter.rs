use super::*;

pub struct Iter<'a, T>(&'a Link<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // almost same as peek
        self.0.as_ref()
        .map(|node| {
            self.0 = &node.next;
            &node.elem
        })
    }
}

impl<T> LinkedStack<T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        Iter(&self.head)
    }
}

//region verbose

// 文章的写法
pub struct IterBook<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for IterBook<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// 我的写法
pub struct IterVerbose<'a, T> {
    next: &'a Link<T>,
}

impl<'a, T> Iterator for IterVerbose<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(node) => {
                self.next = &node.next;
                Some(&node.elem)
            }
        }
    }
}

impl<T> LinkedStack<T> {
    pub fn iter_book(&self) -> impl Iterator<Item=&T> {
        IterBook{ next: self.head.as_deref() }
    }
    pub fn iter_verbose(&self) -> impl Iterator<Item=&T> {
        IterVerbose{ next: &self.head }
    }
}
//endregion