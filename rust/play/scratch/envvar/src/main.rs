use std::env;

fn main() {
    println!("{:?}", env::var("asdf"));
    env::set_var("asdf", "on");
    println!("{:?}", env::var("asdf"));
    env::set_var("asdf", "off");
    println!("{:?}", env::var("asdf"));
}
