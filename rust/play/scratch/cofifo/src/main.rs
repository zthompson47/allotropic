use std::time::Duration;

use mio::{Events, Poll};

fn main() {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(8);
    /*
    use mio::Interest;
    let i = Interest::READABLE;
    i.remove(Interest::READABLE);
    println!("-->>{:?}", i); // prints "-->>READABLE"
    */
    println!("START");
    let result = poll.poll(&mut events, Some(Duration::from_millis(1477)));
    println!("POLLED: {:?}", result);
}
