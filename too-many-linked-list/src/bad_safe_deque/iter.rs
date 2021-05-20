use super::*;

pub struct Iter<T>(Link<T>, Link<T>);

impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_mut().map(|node| {
            todo!()
        })
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> Iter<T> {
    pub fn new(list: &List<T>) -> Iter<T> {
        Iter(list.head.as_ref().map(Arc::clone), list.tail.as_ref().map(Arc::clone))
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