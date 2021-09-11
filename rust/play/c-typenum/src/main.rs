use std::ops::Mul;
use typenum::{Integer, Prod, P5, P7};

type X = <P7 as Mul<P5>>::Output;
type Y = Prod<P7, P5>;

fn main() {
    let x = X::default();
    let y = Y::default();
    println!("Hello, world! {:?}\n{:?}\n{}", x, y, X::to_i32());
}
