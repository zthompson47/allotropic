struct OwnedParsed<'_self> {
    buf: Vec<u8>,
    repr: Parsed<'_self>,
}

struct Parsed<'a> {
    name: &'a str,
}

fn parse<'a>(input: &'a [u8]) -> Result<Parsed<'a>, ()> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
