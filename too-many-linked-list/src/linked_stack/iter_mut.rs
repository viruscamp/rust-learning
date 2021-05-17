use super::*;

pub type IterMut<'a, T> = IterMutBook<'a, T>;
impl<T> LinkedStack<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
    pub fn iter_mut_book(&mut self) -> IterMutBook<T> {
        IterMutBook::new(self)
    }
    pub fn iter_mut_my(&mut self) -> IterMutMy<T> {
        IterMutMy::new(self)
    }
}

// 文章的写法, 原 LinkedStack 空, iter 取完后, 为空, 断开所有借用
pub struct IterMutBook<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMutBook<'a, T> {
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

impl<'a, T> IterMutBook<'a, T> {
    pub fn new(list: &'a mut LinkedStack<T>) -> IterMutBook<'a, T> {
        IterMutBook(list.head.as_mut().map(|node| node.as_mut()))
    }
    pub fn peek(&self) -> Option<&T> {
        match self.0 {
            Some(ref node) => Some(&node.elem),
            None => None,
        }
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.0 {
            Some(ref mut node) => Some(&mut node.elem),
            None => None,
        }
    }
    pub fn split_after(&mut self) -> LinkedStack<T> {
        match self.0 {
            None => LinkedStack::new(),
            Some(ref mut node) => LinkedStack{ head: node.next.take() },
        }
    }
    // 不能插入空串 理论上保存 &mut link 可以
    pub fn insert_after(&mut self, elem: T) {
        match self.0 {
            None => {
                todo!("should insert");
            },
            Some(ref mut node) => {
                let next = node.next.take();
                node.next = Some(Box::new(Node{ elem, next }));
            }
        }
    }
}

//region error
// 始终有一个借用到 LinkedStack 内部, 阻止其 drop
pub struct IterMutMy<'a, T>(&'a mut Link<T>); // 这东西我现在写不出来
// 应该可以实现 insert_at 空串可插入 走完也可插入
/*
impl<'a, T> Iterator for IterMutMy<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(node) => {
                self.0 = &mut node.next;
                Some(&mut node.elem)
            }
        }
    }
}
*/

impl<'a, T> IterMutMy<'a, T> {
    pub fn new(list: &'a mut LinkedStack<T>) -> IterMutMy<'a, T> {
        IterMutMy(&mut list.head)
    }
    /*
    fn next_1(&mut self) -> Option<&'a mut T> {
        self.0.map(|node| {
            self.0 = &mut node.next;
            &mut node.elem
        })
    }
    */
}

//endregion
