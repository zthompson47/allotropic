use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

// UnsafeCell is already !Sync; override to test out threads
//unsafe impl<T> Sync for Cell<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn bad() {
        use std::{sync::Arc, thread::spawn};
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        spawn(move || {
            x1.set(43);
        });
        let x2 = Arc::clone(&x);
        spawn(move || {
            x2.set(44);
        });
    }
    */

    /*
    #[test]
    fn bad2() {
        let x = Cell::new(String::from("jello"));
        let first = x.get();
        x.set(String::new());
        x.set(String::from("swirled"));
        eprint!("{}", first);
    }
    */

    /*
    #[test]
    fn bad3() {
        use std::{sync::Arc, thread::spawn};
        let x = Arc::new(Cell::new(0));
        let x1 = Arc::clone(&x);
        let jh1 = spawn(move || {
            for _ in 0..1000000 {
                x1.set(x1.get() + 1)
            }
        });
        let x2 = Arc::clone(&x);
        let jh2 = spawn(move || {
            for _ in 0..1000000 {
                x2.set(x2.get() + 1)
            }
        });
        jh1.join().unwrap();
        jh2.join().unwrap();

        // Fails with x.get() < 2000000
        assert_eq!(x.get(), 2000000);
    }
    */

    #[test]
    fn it_works_not() {
        let decr = |i| i - 1;
        let incr = |i| i + 1;
        let c = Cell::new(47);
        c.set(incr(c.get()));
        for _ in 1..=6 {
            c.set(decr(c.get()));
        }
        assert_eq!(c.get(), 42)
    }

    #[test]
    fn real_cell() {
        use std::cell::Cell;
        let decr = |i| i - 1;
        let incr = |i| i + 1;
        let c = Cell::new(47);
        c.set(incr(c.get()));
        for _ in 1..=6 {
            c.set(decr(c.get()));
        }
        assert_eq!(c.get(), 42)
    }
}
