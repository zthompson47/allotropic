use std::fmt::Debug;

use boks::Boks;

struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

fn main() {
    let x = 47;
    let b = Boks::ny(x);

    println!("{:?}", b);
    println!("{:?}", *b);

    let mut y = 42;
    let b = Boks::ny(&mut y);
    //let b = Box::new(&mut y);
    println!("{:?}", y);

    let mut z = 42;
    let b = Boks::ny(Oisann(&mut z));
    //let b = Box::new(Oisann(&mut z));
    println!("{:?}", z); // inner T is still accessed despite may_dangle
}
