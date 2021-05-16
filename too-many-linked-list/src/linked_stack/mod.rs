#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
    //prev
}

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedStack<T> {
    head: Link<T>,
    //tail
}

mod iter;
mod into_iter;
mod iter_mut;

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
    /// assert_eq!(s.peek(), s.peek(), "twice peek should get same result");
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem )
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
    
    //region verbose

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

    /// ```should_panic
    /// assert!(false);
    /// ```
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

    //endregion

    /*
    pub fn push_back(&mut self, t: T) {
        let mut link = &mut self.head;
        while let Some(ref mut b) = link.0 {
            link = &mut b.next;
        }
        link.0 = Some(Box::new(Node { elem: t, next: Link(None)}))
    }
    pub fn pop_back(&mut self) -> Option<T> {
        todo!();
        let mut link = &mut self.head;
        if link.0.is_none() {
            return None;
        }
        None
    }
    */
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
