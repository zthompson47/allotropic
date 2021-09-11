#![allow(dead_code)]
#![allow(unused_variables)]
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

type Shared = Rc<RefCell<VecDeque<u8>>>;

fn main() {
    let v = vec![0, 1, 2, 3, 4, 5].into();
    let v: Shared = Rc::new(RefCell::new(v));
    // panic1(&v);
    // panic2(&v);
    // works(&v);
    try_this_panics();
    // try_this_works();
}

fn panic1(v: &Shared) {
    for num in v.borrow_mut().drain(..) {
        println!("{:?}", num);
        mutate(&v);
    }
}

fn panic2(v: &Shared) {
    while let Some(num) = v.borrow_mut().pop_front() {
        println!("{:?}", num);
        mutate(&v);
    }
}

fn works(v: &Shared) {
    let len = v.borrow().len();
    for _ in 0..len {
        let num = v.borrow_mut().pop_front();
        if let Some(num) = num {
            println!("{:?}", num);
            mutate(&v);
        } else {
            break;
        }
    }
}

fn mutate(v: &Shared) {
    v.borrow_mut().push_back(99);
}

fn try_this_panics() {
    let v = RefCell::new(vec![1, 2, 3]);
    while let Some(num) = v.borrow_mut().pop() {
        println!("{}", num);
        v.borrow_mut().push(num + 42);
    }
}

fn try_this_works() {
    let v = RefCell::new(vec![1, 2, 3]);
    loop {
        let num = v.borrow_mut().pop();
        if let Some(num) = num {
            println!("{}", num);
            v.borrow_mut().push(num + 42);
        }
    }
}
