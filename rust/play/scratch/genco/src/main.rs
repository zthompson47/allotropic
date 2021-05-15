#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Yielded(val) => println!("{}", val),
        _ => panic!("unexpected value from resume"),
    }
    match Pin::new(&mut generator).resume(()) {
        GeneratorState::Complete(val) => println!("{}", val),
        _ => panic!("unexpected value from resume"),
    }
}

struct Coroutine<Y, R> {
    generator: Pin<Box<dyn Generator<Yield = Y, Return = R>>>,
}

impl<Y, R> Coroutine<Y, R> {
    fn resume() -> Request {
        Request::Pending
    }
}

enum Request {
    Pending,
}
