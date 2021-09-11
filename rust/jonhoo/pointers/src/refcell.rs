use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};

use crate::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

#[derive(Debug)]
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

#[derive(Debug)]
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

#[derive(Debug)]
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(n) => self.refcell.state.set(RefState::Shared(n - 1)),
            RefState::Exclusive | RefState::Unshared => unreachable!(),
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
        }
    }
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refcell_works() {
        let v = RefCell::new(42);
        let v1 = v.borrow().unwrap();
        assert_eq!(*v1, 42);
        let v2 = v.borrow().unwrap();
        assert_eq!(*v2, 42);
        assert!(v.borrow_mut().is_none());

        #[allow(clippy::drop_ref)]
        drop(v1);
        assert!(v.borrow_mut().is_none());
        #[allow(clippy::drop_ref)]
        drop(v2);
        assert_eq!(&*v.borrow_mut().unwrap(), &mut 42);
    }

    #[test]
    fn refcell_derefmut() {
        let v = RefCell::new(42);
        let mut v1 = v.borrow_mut().unwrap();
        *v1 = 47;
        assert_eq!(*v1, 47);
    }

    #[test]
    fn real_refcell() {
        use std::cell::RefCell;

        let v = RefCell::new(42);
        let v1 = v.borrow();
        assert_eq!(*v1, 42);
        let v2 = v.borrow();
        assert_eq!(*v2, 42);
        assert!(v.try_borrow_mut().is_err());

        #[allow(clippy::drop_ref)]
        drop(v1);
        assert!(v.try_borrow_mut().is_err());
        #[allow(clippy::drop_ref)]
        drop(v2);
        assert_eq!(&*v.borrow_mut(), &mut 42);
    }
}
