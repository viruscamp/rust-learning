use super::*;

pub type IterMut<'a, T> = IterMutMy<'a, T>;
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
        self.0.take().map(|node| {
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
        self.0.as_ref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|node| &mut node.elem)
    }
    pub fn split_after(&mut self) -> LinkedStack<T> {
        LinkedStack { 
            head: self.0.as_mut().and_then(|node| node.next.take())
        }
    }
    // 不能插入空串 理论上保存 &mut link 可以
    pub fn insert_after(&mut self, elem: T) -> Option<()> {
        self.0.as_mut().map(|node| {
            let next = node.next.take();
            node.next = Some(Box::new(Node{ elem, next }));
        })
    }
    // 删掉下一个节点
    pub fn delete_after(&mut self) -> Option<T> {
        todo!()
    }
    // split + join
    pub fn replace_after(&mut self, join: LinkedStack<T>) -> LinkedStack<T> {
        todo!()
    }
    // 合并
    pub fn insert_list_after(&mut self, join: LinkedStack<T>) -> Option<()> {
        todo!()
    }
}

// 始终有一个借用到 LinkedStack 内部, 阻止其 drop
pub struct IterMutMy<'a, T>(&'a mut Link<T>); // 这东西我现在写不出来
// 应该可以实现 insert_at 空串可插入 走完也可插入
impl<'a, T> Iterator for IterMutMy<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| unsafe {
            let node_ptr: *mut Node<T> = Box::into_raw(node);
            *self.0 = Some(Box::from_raw(node_ptr));
            self.0 = &mut (*node_ptr).next;
            &mut (*node_ptr).elem
        })
    }
}

impl<'a, T> IterMutMy<'a, T> {
    pub fn new(list: &'a mut LinkedStack<T>) -> IterMutMy<'a, T> {
        IterMutMy(&mut list.head)
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.as_deref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.as_deref_mut().map(|node| &mut node.elem)
    }
    pub fn split_at(&mut self) -> LinkedStack<T> {
        LinkedStack {
            head: self.0.take()
        }
    }
    pub fn insert_at(&mut self, elem: T) {
        *self.0 = Some(Box::new(Node {
            elem,
            next: self.0.take(),
        }))
    }
    // 删掉当前节点
    pub fn delete_at(&mut self) -> Option<T> {
        todo!()
    }
    // split + join
    pub fn replace_at(&mut self, join: LinkedStack<T>) -> LinkedStack<T> {
        todo!()
    }
    // 合并
    pub fn insert_list_at(&mut self, join: LinkedStack<T>) -> Option<()> {
        todo!()
    }
    fn next_verbose(&mut self) -> Option<&'a mut T> {
        match self.0 {
            None => None,
            Some(node) => {
                unsafe {
                    let node_ptr: *mut Node<T> = node.as_mut();
                    self.0 = &mut (*node_ptr).next;
                    Some(&mut (*node_ptr).elem)
                }
            }
        }
    }
}
