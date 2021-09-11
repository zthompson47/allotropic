// https://www.youtube.com/watch?v=8O0Nt9qY_vo 1:30:03, what does question mean?
// "Since memory is sized-aligne can compiler fit other NonNull variants in 0, 1, 2, 3?"
use std::{marker::PhantomData, ptr::NonNull};

use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    //refcount: usize,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    //inner: *const RcInner<T>,
    inner: NonNull<RcInner<T>>, // !Send, which is what we want Rc to be
    // Need to mark this type as owning a T so drop check will disallow using
    // a reference to T after T is dropped by Rc<T> being dropped.
    //_marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: Cell::new(1),
        });

        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            //inner: &*inner, // Nope - Box would be dropped
            //_marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        match c {
            1 => {
                // Drop reference to inner which is invalid after Box is dropped
                #[allow(clippy::drop_ref)]
                drop(inner);

                drop(unsafe { Box::from_raw(self.inner.as_ptr()) });
            }
            _ => inner.refcount.set(c - 1),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            //_marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &unsafe { self.inner.as_ref() }.value
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::{channel, Sender};

    use super::*;

    struct Dropped(Sender<()>, String);

    impl Drop for Dropped {
        fn drop(&mut self) {
            self.0.send(()).unwrap()
        }
    }

    #[test]
    fn rc_works() {
        let (tx, rx) = channel();
        let rc = Rc::new(Dropped(tx, String::from("jello")));
        assert_eq!(*rc.1, String::from("jello"));
        let rc1 = rc.clone();
        assert_eq!(*rc1.1, String::from("jello"));
        drop(rc);
        assert!(rx.try_recv().is_err());
        drop(rc1);
        assert!(rx.try_recv().is_ok());
    }

    #[derive(Debug, Default, PartialEq)]
    struct Def;

    impl Drop for Def {
        fn drop(&mut self) {
            println!("-->>Drop Def");
        }
    }

    #[derive(Debug, PartialEq)]
    struct Foo<'a, T: Default> {
        v: &'a mut T,
    }

    impl<T: Default> Drop for Foo<'_, T> {
        fn drop(&mut self) {
            println!("-->>Drop Foo before");
            let a = std::mem::take(self.v);
            println!("-->>Drop Foo after");
        }
    }

    #[test]
    fn drop_check() {
        let (the_foo, t);
        println!("BEFORE box::leak");
        t = Box::leak(Box::new(Def));
        println!("AFTER box::leak");
        the_foo = Rc::new(Foo { v: t });
        println!("AFTER rc::new");

        assert_eq!(*the_foo.v, Def);

        drop(*t);
        println!("-->> END OF TEST");
    }
}
