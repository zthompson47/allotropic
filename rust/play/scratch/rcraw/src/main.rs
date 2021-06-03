use std::rc::Rc;

fn main() {
    let s = String::from("jello world");

    let r = Rc::new(s);
    println!("count: {}", Rc::strong_count(&r));

    let rr = r.clone();
    println!("count: {}", Rc::strong_count(&r));

    let rraw = Rc::into_raw(r);
    println!("count: {}", Rc::strong_count(&rr));
}
