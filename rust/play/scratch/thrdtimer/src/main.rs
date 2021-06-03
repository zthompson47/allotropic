/*
use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            tx.send(()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    while let Ok(()) = rx.recv() {
        println!("tick");
    }
}
*/

use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        loop {
            println!("contact client");
            thread::sleep(Duration::from_secs(1));
        }
    });
    // Park so this example doesn't exit immediately.
    thread::park();
}
