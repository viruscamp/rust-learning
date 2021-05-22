use super::*;

// 始终有一个借用到 LinkedStack 内部, 阻止其 drop
pub struct Iter<'a, T>(&'a Link<T>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // almost same as peek
        self.0.as_ref().map(|node| {
            self.0 = &node.next;
            &node.elem
        })
    }
}

impl<'a, T> Iter<'a, T> {
    pub fn new(list: &List<T>) -> Iter<T> {
        Iter(&list.head)
    }
    pub fn peek(&self) -> Option<&T> {
        //self.0.as_ref().map(|node| &node.elem)
        match self.0 {
            Some(node) => Some(&node.elem),
            None => None,
        }
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
