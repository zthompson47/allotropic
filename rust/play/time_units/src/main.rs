fn main() {
    println!("Hello, world!");
}

trait Quantity<V, U> {
    fn value(&self) -> V;
    fn unit(&self) -> U;
}

trait Unit {
}

struct TimeUtc {
    year: u32,

}
