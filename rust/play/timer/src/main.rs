use std::time::Duration;

use run::Runner;
use time::sleep;

fn main() {
    let mut rt = Runner::new();

    let f1 = async {
        for _ in 0..3 {
            println!("f1");
            sleep(Duration::from_millis(999)).await;
        }
        42
    };
    let f2 = async {
        for _ in 0..9 {
            println!("f2");
            sleep(Duration::from_millis(333)).await;
        }
        "47"
    };

    rt.spawn(async move {
        println!("{:?}", f1.await);
        println!("f1 done");
    });

    rt.spawn(async move {
        println!("{:?}", f2.await);
        println!("f2 done");
    });

    rt.run();
}

mod wait {
    use std::{
        cell::RefCell,
        collections::BinaryHeap,
        io, thread_local,
        time::{Duration, Instant},
    };

    use mio::{Events, Poll};

    thread_local! {
        pub static WAITER: RefCell<Waiter> = RefCell::new(Waiter::new().unwrap());
    }

    pub struct Waiter {
        poll: Poll,
        events: Events,
        timers: BinaryHeap<Instant>,
    }

    impl Waiter {
        pub fn new() -> io::Result<Self> {
            Ok(Waiter {
                poll: Poll::new()?,
                events: Events::with_capacity(8),
                timers: BinaryHeap::new(),
            })
        }

        pub fn push_next_wake(&mut self, req: Instant) {
            eprintln!("--->>WAIT: push_next_wake {:?}", req);
            self.timers.push(req);
        }

        pub fn wait(&mut self) {
            eprintln!("QUEUELEN: {}", self.timers.len());

            if self.timers.is_empty() {
                return;
            }

            // Get smallest timeout
            let heap = self.timers.to_owned();
            let mut v = heap.into_sorted_vec();
            v.reverse();
            let timeout = v.pop().unwrap();
            self.timers = BinaryHeap::from(v);

            eprintln!("QUEUELEN: {}", self.timers.len());
            eprintln!("=====>>QUEUE: {:?}", self.timers);
            eprintln!(
                "WAITER: queue.len():{:?}, dur:{:?}",
                self.timers.len(),
                timeout
            );

            let now = Instant::now();
            let mut dur = Duration::from_millis(0);
            if now < timeout {
                dur = timeout - now;
            }

            self.poll.poll(&mut self.events, Some(dur)).unwrap();
        }
    }
}

mod time {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
        time::{Duration, Instant},
    };

    use crate::wait::WAITER;

    pub async fn sleep(duration: Duration) {
        let timer = Timer::new(duration);
        timer.await;
    }

    struct Timer {
        deadline: Instant,
    }

    impl Timer {
        fn new(duration: Duration) -> Self {
            let deadline = Instant::now() + duration;
            WAITER.with(|waiter| {
                waiter.borrow_mut().push_next_wake(deadline);
            });
            Timer { deadline }
        }
    }

    impl Future for Timer {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            if Instant::now() >= self.deadline {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }
}

mod run {
    use std::{
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };

    use crossbeam::channel;
    use futures::{channel::oneshot, task::ArcWake};

    use crate::wait::WAITER;

    type InnerFut = Pin<Box<dyn Future<Output = ()>>>;

    pub struct Task<T> {
        fut: Mutex<Pin<Box<dyn Future<Output = T> + Send>>>,
        // queue_tx: channel::Sender<InnerFut>,
    }

    /*
    impl<T> Task<T> {
        fn awake(&self) {
            self.queue_tx.send(self.fut).unwrap();
        }
    }
    */

    impl<T> Future for Task<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<T> {
            self.fut.lock().unwrap().as_mut().poll(cx)
        }
    }

    /*
    impl<T> ArcWake for Task<T> {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            arc_self.awake();
        }
    }
    */

    pub struct Runner {
        // tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
        waiting: Vec<Pin<Box<dyn Future<Output = ()>>>>,
        queue_rx: channel::Receiver<InnerFut>,
        queue_tx: channel::Sender<InnerFut>,
    }

    impl Runner {
        pub fn new() -> Self {
            let (queue_tx, queue_rx) = channel::unbounded();
            Runner {
                // tasks: Vec::new(),
                waiting: Vec::new(),
                queue_tx,
                queue_rx,
            }
        }

        pub fn spawn<T, F>(&mut self, f: F) -> Task<T>
        where
            T: Send + 'static,
            F: Future<Output = T> + Send + 'static,
        {
            let (tx, rx) = oneshot::channel();
            let fut = async move {
                let _ = tx.send(f.await);
            };

            // self.tasks.push(Box::pin(fut));
            self.queue_tx.send(Box::pin(fut)).unwrap();

            Task {
                fut: Mutex::new(Box::pin(async move { rx.await.unwrap() })),
                // queue_tx: self.queue_tx.clone(),
            }
        }

        pub fn run(mut self) {
            loop {
                eprintln!("RUN: about to drain");
                // for mut task in self.tasks.drain(..) {
                while let Some(mut task) = self.queue_rx.try_iter().next() {
                    unsafe {
                        let waker = Waker::from_raw(new_raw_waker());
                        let mut context = Context::from_waker(&waker);
                        match task.as_mut().poll(&mut context) {
                            //Poll::Pending => self.queue_tx.send(task).unwrap(),
                            Poll::Pending => self.waiting.push(task),
                            Poll::Ready(()) => {}
                        }
                    }
                }
                eprintln!("RUN: about to wait");
                WAITER.with(|waiter| {
                    eprintln!("RUN: about to wait INNER");
                    waiter.borrow_mut().wait();
                });
                /*
                if self.tasks.is_empty() && self.waiting.is_empty() {
                    break;
                } else {
                    std::mem::swap(&mut self.tasks, &mut self.waiting);
                }
                */
                if self.waiting.is_empty() {
                    break;
                } else {
                    for t in self.waiting.drain(..) {
                        self.queue_tx.send(t).unwrap();
                    }
                }
            }
        }
    }

    fn new_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn clone(_data: *const ()) -> RawWaker {
        eprintln!("**clone!");
        new_raw_waker()
    }

    unsafe fn wake(data: *const ()) {
        eprintln!("**wake!");
        wake_by_ref(data);
    }

    unsafe fn wake_by_ref(_data: *const ()) {
        eprintln!("**wake_by_ref!");
    }

    unsafe fn drop(_data: *const ()) {
        // eprintln!("**drop!");
    }
}
