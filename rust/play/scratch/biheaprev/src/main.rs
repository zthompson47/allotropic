use std::{collections::BinaryHeap, cmp::Reverse};

fn main() {
    let mut bh = BinaryHeap::new();

    for i in 0..10 {
        bh.push(Reverse(i));
    }

    println!("{:?}", bh);

    let v = bh.peek();

    if let Some(Reverse(v)) = v {
        println!("{}", v);
    }

    let w = bh.pop();

    if let Some(Reverse(w)) = w {
        println!("{}", w);
    }
}
