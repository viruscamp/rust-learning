use super::*;

// 始终有一个借用到 LinkedStack 内部, 阻止其 drop
pub struct IterMut<'a, T>(&'a mut Link<T>);
// 应该可以实现 insert_at 空串可插入 走完也可插入
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            None => None,
            Some(node) => unsafe {
                let node_ptr: *mut Node<T> = node.as_mut();
                self.0 = &mut (*node_ptr).next;
                Some(&mut (*node_ptr).elem)
            }
        }

        /*
        // 闭包一定会 borrow &self.0 跟 node borrow 的冲突了, 不能用 map
        // 导致运行时错误 (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
        self.0.as_deref_mut().map(|node| unsafe {
            let node_ptr: *mut Node<T> = node;
            self.0 = &mut (*node_ptr).next;
            &mut (*node_ptr).elem
        })
        */

        /*
        // IterMut 不持有 Option<T>, take 是改链表而不是改 IterMut
        // 导致运行时错误 (exit code: 0xc0000374, STATUS_HEAP_CORRUPTION)
        self.0.take().as_mut().map(|node| unsafe {
            let node_ptr: *mut Node<T> = node.as_mut();
            self.0 = &mut (*node_ptr).next;
            &mut (*node_ptr).elem
        })
        */
    }
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(list: &'a mut List<T>) -> IterMut<'a, T> {
        IterMut(&mut list.head)
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.as_deref().map(|node| &node.elem)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.as_deref_mut().map(|node| &mut node.elem)
    }
}

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }
}