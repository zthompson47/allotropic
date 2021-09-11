fn main() {
    let a = String::new();
    let b: &mut &str = &mut a.as_ref();
    let c: &'static str = "Static, world!";
    *b = c;

    println!("{}", b);

    drop(a);
}
