use std::{cell::UnsafeCell, sync::{Arc, atomic::{AtomicBool, AtomicUsize, Ordering}}, thread::spawn};

fn main() {
    sanity();
    shared_counter();
    //non_send_shared_counter();
}

fn sanity() {
    let m = Mutex::new("Hello, world!");
    let r = m.with_lock(|d| d.replace("Hello", "Jello"));
    println!("{}", r);
}

fn shared_counter() {
    let num: &_ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..100)
        .map(|_| {
            spawn(move || {
                // Attempt to trigger race condition - seems to work
                //std::thread::sleep(std::time::Duration::from_millis(1));
                //std::thread::sleep(std::time::Duration::from_secs(1));
                for _ in 0..1000 {
                    num.with_lock(|x| {
                        *x += 1;
                        // Attempt to trigger race condition - seems to work
                        //std::thread::sleep(std::time::Duration::from_millis(1));
                    });
                    // Attempt to trigger race condition - seems to work
                    //std::thread::sleep(std::time::Duration::from_millis(1));
                    //num.with_lock(|x| {
                    //    let _ = std::mem::take(x);
                    //});
                }
            })
        })
        .collect();

    spawn(move || {
        num.with_lock(|_| panic!());
    });

    for handle in handles {
        handle.join().unwrap();
    }
    let res = num.with_lock(|x| *x);
    assert_eq!(res, 100 * 1000);
    println!("res: {}", res);
}

/*
use std::rc::Rc;
fn non_send_shared_counter() {
    let num: &_ = Box::leak(Box::new(Mutex::new(Rc::new(0))));
    let handles: Vec<_> = (0..10).map(|_| {
        spawn(move || {
            for _ in 0..100 {
                num.with_lock(|x| {
                    let mut y = x.clone();
                    *Rc::get_mut(&mut y).unwrap() += 1;
                });
            }
        })
    }).collect();
    for handle in handles{
        handle.join().unwrap();
    }
    let res = num.with_lock(|x| **x);
    assert_eq!(res, 10 * 100);
    println!("res: {}", res);
}
*/

unsafe impl<T> Sync for Mutex<T> where T: Send {}
//unsafe impl<T> Sync for Mutex<T> {}

const LOCKED: bool = true;
const UNLOCKED: bool = false;

struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Mutex {
            locked: AtomicBool::new(UNLOCKED),
            data: UnsafeCell::new(data),
        }
    }

    fn _with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // Spinlock
        while self.locked.load(Ordering::Relaxed) == LOCKED {}
        self.locked.store(LOCKED, Ordering::Relaxed);
        // Help trigger race condition
        //std::thread::yield_now();
        // Safety: we hold the lock, therefore we can create a mutable reference
        let ret = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        while self
            .locked
            //.compare_exchange_weak(UNLOCKED, LOCKED, Ordering::SeqCst, Ordering::SeqCst)
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Improves performance by keeping value in shared (not exclusive) memory state
            // vs. compare_exchange_weak checks in exclusive state each time
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                //std::thread::yield_now();
            }
            //std::thread::yield_now();
        }
        let ret = f(unsafe { &mut *self.data.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        ret
    }
}

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &_ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &_ = Box::leak(Box::new(AtomicUsize::new(0)));
    let t1 = spawn(move || {
        // Give r1 a chance to become 42
        //std::thread::sleep(std::time::Duration::from_millis(2));
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });
    let t2 = spawn(move || {
        // The compiler can reverse the order of the next two lines...
        //let r2 = x.load(Ordering::Relaxed);
        //y.store(42, Ordering::Relaxed);
        // ...to this:
        y.store(42, Ordering::Relaxed);
        #[allow(clippy::let_and_return)]
        let r2 = x.load(Ordering::Relaxed);

        r2
    });
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // It's now possible that: r1 == r2 == 42; ???

    println!("r1: {}, r2: {}", r1, r2);
    assert_eq!(r1, 42);
    assert_eq!(r2, 42);
}

#[test]
fn sequential_consistent() {
    let x: &_ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &_ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &_ = Box::leak(Box::new(AtomicUsize::new(0)));

    let _write_x = spawn(move || {
        x.store(true, Ordering::Release);
    });

    let _write_y = spawn(move || {
        y.store(true, Ordering::Release);
    });

    let read_x_then_y = spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let read_y_then_x = spawn(move || {
        while !y.load(Ordering::Acquire) {}
        if x.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    read_x_then_y.join().unwrap();
    read_y_then_x.join().unwrap();

    assert!(z.load(Ordering::Relaxed) != 0);

    /******************
       Possible values for `z`:

       2: write_x, write_y, read_x_then_y, read_y_then_x
       1: write_x, read_x_then_y, write_y, read_y_then_x
NOPE!  0: while loops in read_* reordered to after if stmt, then:
          read_x_then_y, read_y_then_x, write_x, write_y
    */
}

#[test]
fn loom_sequential_consistent() {
    use loom::{sync::atomic::{AtomicBool, AtomicUsize}, thread::spawn};
    loom::model(|| {
        let x: &_ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &_ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &_ = Box::leak(Box::new(AtomicUsize::new(0)));

        let _write_x = spawn(move || {
            x.store(true, Ordering::SeqCst);
        });

        let _write_y = spawn(move || {
            y.store(true, Ordering::SeqCst);
        });

        let read_x_then_y = spawn(move || {
            while !x.load(Ordering::SeqCst) {
                loom::thread::yield_now();
            }
            if y.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::SeqCst);
            }
        });

        let read_y_then_x = spawn(move || {
            while !y.load(Ordering::SeqCst) {
                loom::thread::yield_now();
            }
            if x.load(Ordering::SeqCst) {
                z.fetch_add(1, Ordering::SeqCst);
            }
        });

        read_x_then_y.join().unwrap();
        read_y_then_x.join().unwrap();

        assert_ne!(z.load(Ordering::SeqCst), 0);
    });
}


