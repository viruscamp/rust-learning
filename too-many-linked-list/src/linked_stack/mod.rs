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
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref()
        .map(|node| &node.elem )
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node{ elem, next: self.head.take() }));
    }

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
