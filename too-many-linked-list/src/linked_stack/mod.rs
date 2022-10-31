//! [2. A Bad Stack](http://rust-unofficial.github.io/too-many-lists/first-final.html)
//! [3. An Ok Stack](http://rust-unofficial.github.io/too-many-lists/second-final.html)
//! 附加 peek_back push_back pop_back 此三方法循环或递归找队尾, 效率极低, pop_back 用了 unsafe

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    //prev
}

type Link<T> = Option<Box<Node<T>>>;

/// 如下函数编译成功证明了 `LinkedStack<T>` 对 `T` 协变
/// ```no_run
/// # use too_many_linked_list::linked_stack::LinkedStack;
/// fn ensure_covariant<'long: 'short, 'short>(list_long: LinkedStack<&'long i32>, mut list_short: LinkedStack<&'short i32>) {
///     let list_short_new: LinkedStack<&'short i32> = list_long; // 证明协变
///     //let list_long_new: LinkedStack<&'long i32> = list_short; // 证明逆变
/// }
/// ```
pub struct LinkedStack<T> {
    head: Link<T>,
    //tail
}

mod iter;
mod into_iter;
mod iter_mut;

#[cfg(test)]
mod test;

// 循环 drop , 不这样做的话, 默认的 Drop 是递归的, 元素多时会爆栈
impl<T> Drop for LinkedStack<T> {
    default fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut boxed_node) = link {
            link = boxed_node.next.take();
        }
    }
}

// 测试用 强制使用默认的递归 drop
pub trait LinkedStackRecursionDrop {}
impl<T: LinkedStackRecursionDrop> Drop for LinkedStack<T> {
    fn drop(&mut self) {}
}

impl<T> LinkedStack<T> {
    /// Create a new empty LinkedStack.
    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&0i32;0]);
    /// ```
    pub fn new() -> Self {
        Self { head: None }
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.peek(), None);
    /// s.push(4);
    /// assert_eq!(s.peek(), Some(&4));
    /// s.push(3);
    /// assert_eq!(s.peek(), Some(&3));
    /// assert_eq!(s.peek(), s.peek(), "twice peek should get same result");
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem )
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// s.push(1);
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&1]);
    /// s.push(2);
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&2, &1], "notice the sequence of elements");
    /// ```
    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node{ elem, next: self.head.take() }));
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.pop(), None);
    /// s.push(1);s.push(2);
    /// assert_eq!(s.pop(), Some(2));
    /// assert_eq!(s.pop(), Some(1));
    /// assert_eq!(s.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.peek_back(), None);
    /// s.push(4);
    /// assert_eq!(s.peek_back(), Some(&4));
    /// s.push(3);
    /// assert_eq!(s.peek_back(), Some(&4));
    /// assert_eq!(s.peek_back(), s.peek_back(), "twice peek should get same result");
    /// ```
    pub fn peek_back(&self) -> Option<&T> {
        let mut link = &self.head;
        let mut ret: Option<&T> = None;
        while let Some(ref b) = link {
            link = &b.next;
            ret = Some(&b.elem);
        }
        ret
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// s.push_back(1);
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&1]);
    /// s.push_back(2);
    /// assert_eq!(s.iter().collect::<Vec<&i32>>(), [&1, &2], "notice the sequence of elements");
    /// ```
    pub fn push_back(&mut self, elem: T) {
        let mut link = &mut self.head;
        while let Some(ref mut b) = link {
            link = &mut b.next;
        }
        *link = Some(Box::new(Node{ elem, next: None }))
    }

    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(s.pop_back(), None);
    /// s.push(1);s.push(2);
    /// assert_eq!(s.pop_back(), Some(1));
    /// assert_eq!(s.pop_back(), Some(2));
    /// assert_eq!(s.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut link = &mut self.head;
        match link {
            None => None,
            Some(ref mut node) => {
                let mut node_ptr: *mut Node<T> = node.as_mut();
                unsafe {
                    while let Some(ref mut node) = (*node_ptr).next {
                        link = &mut (*node_ptr).next;
                        node_ptr = node.as_mut();
                    }
                }
                link.take().map(|node| node.elem)
            }
        }
    }
}

/// 反转
fn reverse<T>(ls: &mut LinkedStack<T>) {
    let mut oldhead = ls.head.take();
    let mut head = None;
    while let Some(mut node) = oldhead.take() {
        oldhead = node.next;
        node.next = head.take();
        head = Some(node);
    }
    ls.head = head;
}

// 独占借用递归 pop
fn pop_back_recursive<T>(link: &mut Link<T>) -> Option<T> {
    // 这个 match 对应 link.as_mut().and_then(|sub_node| {}) 写不出, 因为里面再次用了 link
    match link {
        None => None,
        Some(sub_node) => {
            pop_back_recursive(&mut sub_node.next).or_else(||
                link.take().map(|n| n.elem)
            )
        }
    }
}

// 子表 take 掉, 还个新的子表
fn pop_back_replace<T>(link: Link<T>) -> (Link<T>, Option<T>) {
    match link {
        None => (None, None),
        Some(mut node) => {
            let (new_next, pop_val) = pop_back_replace(node.next);
            node.next = new_next;
            match pop_val {
                None => (None, Some(node.elem)),
                Some(_) => (Some(node), pop_val),
            }
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedStack<T> {
    /// # Examples
    /// ```
    /// use too_many_linked_list::linked_stack::*;
    /// let mut s = LinkedStack::<i32>::new();
    /// assert_eq!(format!("{:?}", &s), format!("{:?}", [0i32;0]));
    /// s.push(3);
    /// assert_eq!(format!("{:?}", &s), format!("{:?}", [3]));
    /// s.push(1);
    /// assert_eq!(format!("{:?}", &s), format!("{:?}", [1,3]), "notice the sequence of elements");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

/// 功能完全相同，但是代码啰嗦的实现
impl<T> LinkedStack<T> {
    pub fn peek_verbose(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.elem),
            None => None,
        }
    }

    pub fn push_verbose(&mut self, t: T) {
        let head = std::mem::replace(&mut self.head, None);
        let new_node = Box::new(Node { elem: t, next: head });
        self.head = Some(new_node);
    }

    pub fn pop_verbose(&mut self) -> Option<T> {
        let head = std::mem::replace(&mut self.head, None);
        match head {
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
            None => None,
        }
    }

    pub fn pop_back_verbose(&mut self) -> Option<T> {
        pop_back_recursive_verbose(&mut self.head)
    }
}

fn pop_back_recursive_verbose<T>(link: &mut Link<T>) -> Option<T> {
    // 这个 match 对应 link.as_mut().and_then(|sub_node| {
    match link {
        None => None,
        Some(sub_node) => {
            let sub_ret = pop_back_recursive(&mut sub_node.next);
            // 这个 match 对应 .or_else(
            match sub_ret {
                Some(t) => Some(t),
                None => {
                    let current_node = link.take();
                    // 这个 match 对应 .map(|n| n.elem)
                    match current_node { // 此判断应该是多余的
                        Some(n) => Some(n.elem),
                        None => None, // 此处不可能出现
                    }
                }
            }
        }
    }
}
