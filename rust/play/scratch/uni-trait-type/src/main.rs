fn main() {
    let v: Vec<Box<dyn Foo>> = Vec::new();
}

trait Foo {
    type Bar;
}
