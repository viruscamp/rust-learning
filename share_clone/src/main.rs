use std::ops::Deref;

fn main() {
    use std::sync::Arc;

    println!("Hello, world!");
    let a0 = Arc::new(3);
    let a1 = a0.share();
}

pub trait Share : Deref {
    fn share(&self) -> Self;

    fn clone1(&self) -> Self
        where <Self as Deref>::Target: Clone;
}

impl<T> Share for std::rc::Rc<T> {
    fn share(&self) -> Self {
        <Self as Clone>::clone(&self)
    }
    
    fn clone1(&self) -> Self
        where T: Clone
    {
        std::rc::Rc::new(self.as_ref().clone())
    }
}

impl<T> Share for std::sync::Arc<T> {
    fn share(&self) -> Self {
        <Self as Clone>::clone(&self)
    }
    
    fn clone1(&self) -> Self
        where T: Clone
    {
        std::sync::Arc::new(self.as_ref().clone())
    }
}

