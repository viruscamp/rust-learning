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

impl<T> LinkedStack<T> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
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
