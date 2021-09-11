use std::marker::PhantomData;

fn main() {
    let ac: Val<T: Thing> = 
}

trait Thing<A, B, C, D, E, F, G> {}
trait On {}
trait Off {}

struct Quant<T> {
    v: PhantomData<T>,
}

struct Val<T> {
    val: Quant<T>,
}
