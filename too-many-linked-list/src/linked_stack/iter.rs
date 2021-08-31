use super::*;

pub type Iter<'a, T> = IterMy<'a, T>;
impl<T> LinkedStack<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
    pub fn iter_my(&self) -> IterMy<T> {
        IterMy::new(self)
    }
    pub fn iter_book(&self) -> IterBook<T> {
        IterBook::new(self)
    }
    pub fn iter_verbose(&self) -> IterVerbose<T> {
        IterVerbose::new(self)
    }
}

// 始终有一个借用到 LinkedStack 内部, 阻止其 drop
pub struct IterMy<'a, T>(&'a Link<T>);

impl<'a, T> Iterator for IterMy<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.as_ref().map(|node| {
            self.0 = &node.next;
            &node.elem
        })
    }
}

impl<'a, T> IterMy<'a, T> {
    pub fn new(list: &LinkedStack<T>) -> IterMy<T> {
        IterMy(&list.head)
    }
    pub fn peek(&self) -> Option<&T> {
        // self.0 自动 deref 成 Option<Box<T>>, 而 Option<Box<T>> 非 Copy, 最终 map 消耗它
        // self.0.map(|node| &node.elem) // 错误
        self.0.as_ref().map(|node| &node.elem)
    }
}

// region verbose

// 文章的写法, 原 LinkedStack 空, iter 取完后, 为空, 断开所有借用
pub struct IterBook<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for IterBook<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // 因为 &T 是 Copy 所以 Option<&T> 是 Copy , 最终 map 都不会消耗 self.next
        // self.next.map(|node| {
        //     self.next = node.next.as_deref();
        //     &node.elem
        // })
        match self.next {
            Some(node) => {
                self.next = node.next.as_deref();
                Some(&node.elem)
            },
            None => None,
        }
    }
}

impl<'a, T> IterBook<'a, T> {
    pub fn new(list: &LinkedStack<T>) -> IterBook<T> {
        IterBook{ next: list.head.as_deref() }
    }
    pub fn peek(&self) -> Option<&T> {
        //self.next.map(|node| &node.elem )
        match self.next {
            Some(node) => Some(&node.elem),
            None => None,
        }
    }
}

// 我的写法 与第一个 IterMy 没有本质区别
pub struct IterVerbose<'a, T> {
    next: &'a Link<T>,
}

impl<'a, T> Iterator for IterVerbose<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = &node.next;
                Some(&node.elem)
            },
            None => None,
        }
    }
}

impl<'a, T> IterVerbose<'a, T> {
    pub fn new(list: &LinkedStack<T>) -> IterVerbose<T> {
        IterVerbose{ next: &list.head }
    }
    pub fn peek(&self) -> Option<&T> {
        match self.next {
            Some(node) => Some(&node.elem),
            None => None,
        }
    }
}

// endregion
