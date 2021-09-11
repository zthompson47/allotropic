use std::cell::UnsafeCell;

use crate::refcell::{Ref, RefCell};

pub struct Rc<'parent, T>
where
    T: ?Sized,
{
    parent: Option<Ref<'parent, &'parent Self>>,
    inner: Box<T>,
}

impl<T> Rc<'_, T> {
    fn new(value: T) -> Self {
        Rc {
            inner: UnsafeCell::new(value),
            parent: None,
        }
    }
}

impl<T> Clone for Rc<'_, T> {
    fn clone(&self) -> Self {
        Rc {
            parent: Some(match self.parent {
                None => self,
                Some(p) => p,
            }),
            inner: self.inner,
        }
    }
}
