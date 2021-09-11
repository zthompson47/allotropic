use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread::{spawn, JoinHandle},
};

pub struct ThreadPool {
    _handles: Vec<JoinHandle<()>>,
    sender: Sender<Box<dyn FnOnce() + Send>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let (sender, receiver) = channel::<Box<dyn FnOnce() + Send + 'static>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone = receiver.clone();
            let handle = spawn(move || {
                while let Ok(work) = clone.lock().unwrap().recv() {
                    println!("Start");
                    work();
                    println!("Finish");
                }
            });
            _handles.push(handle);
        }
        ThreadPool { _handles, sender }
    }

    pub fn execute<T: FnOnce() + Send + 'static>(&self, work: T) {
        self.sender.send(Box::new(work)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::atomic::{AtomicU32, Ordering},
        thread::sleep,
        time::Duration,
    };

    use super::*;

    #[test]
    fn update_shared_value() {
        let n = Arc::new(AtomicU32::new(0));
        let pool = ThreadPool::new(10);
        let n_clone = n.clone();
        let work = move || {
            n_clone.fetch_add(1, Ordering::SeqCst);
        };

        pool.execute(work.clone());
        pool.execute(work);
        sleep(Duration::from_secs(2));

        assert_eq!(n.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn thread_drops_work() {
        let s = String::from("asdf");
        let pool = ThreadPool::new(10);
        let work = || {
            println!("{}", String::from(s));
        };

        pool.execute(work);
    }
}
