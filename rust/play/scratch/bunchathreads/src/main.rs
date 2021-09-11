use std::{collections::HashMap, sync::mpsc::channel, thread::spawn};

fn main() {
    let mut txs = HashMap::new();
    let mut rxs = Vec::new();

    for i in 0..50 {
        let (tx, rx) = channel::<usize>();
        txs.insert(i, tx);
        rxs.push(rx);
    }

    for (i, rx) in rxs.into_iter().enumerate() {
        let txsc = txs.clone();
        spawn(move || {
            for k in txsc.keys() {
                txsc.get(k).unwrap().send(i).unwrap();
            }
            while let Ok(msg) = rx.recv() {
                println!("thread {} received {}", i, msg);
            }
        });
    }

    std::thread::park();
}
