fn main() {
    let obj = Obj(47);
    let mut sr = SelfRef {
        a: obj,
        b: None,
    };
    sr.b = Some(&sr.a);
    /*
    drop(sr.b);
    sr.b = None;
    */
    sr.a = Obj(99);
    //print(sr);
}

#[derive(Debug)]
struct SelfRef<'a> {
    a: Obj,
    b: Option<&'a Obj>,
}

// #[derive(Clone, Copy, Debug)]
#[derive(Debug)]
struct Obj(u8);

fn print(mut sr: SelfRef) {
    sr.a = Obj(42);
    println!("{:?}", sr);
}
