fn main() {
    let s = String::new();
    let x: &'static str = "jello swirled";
    let mut y /* : &'a str*/ = &*s;
    y = x; // 'static -> 'a
}
