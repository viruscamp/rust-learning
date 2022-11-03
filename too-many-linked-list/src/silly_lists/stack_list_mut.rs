//! Invalid, use `Cell<T>`
pub struct List<'a, T> {
    pub data: T,
    pub prev: Option<&'a mut List<'a, T>>,
}

pub struct Iter<'a, T> {
    next: Option<&'a List<'a, T>>,
}

impl<'a, T> List<'a, T> {
    pub fn push<U>(
        prev: Option<&'a mut List<'a, T>>, 
        data: T, 
        callback: impl FnOnce(&mut List<'a, T>) -> U,
    ) -> U {
        let mut list = List { data, prev };
        callback(&mut list)
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter { next: Some(self) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.prev.as_ref().map(|prev| &**prev);
            &node.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    // the data struct is invalid
    /*
    #[test]
    fn cell() {
        use std::cell::Cell;

        List::push(None, Cell::new(3), |list| {
            List::push(Some(list), Cell::new(5), |list| {
                List::push(Some(list), Cell::new(13), |list| {
                    // Multiply every value in the list by 10
                    for val in list.iter() {
                        val.set(val.get() * 10)
                    }

                    let mut vals = list.iter();
                    assert_eq!(vals.next().unwrap().get(), 130);
                    assert_eq!(vals.next().unwrap().get(), 50);
                    assert_eq!(vals.next().unwrap().get(), 30);
                    assert_eq!(vals.next(), None);
                    assert_eq!(vals.next(), None);
                })
            })
        })
    }
    */
}
