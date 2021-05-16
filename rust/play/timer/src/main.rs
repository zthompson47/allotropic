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

    eprintln!("before spawn f1");
    rt.spawn(async move {
        println!("{:?}", f1.await);
    });
    eprintln!("after spawn f1");

    eprintln!("before spawn f2");
    rt.spawn(async move {
        println!("{:?}", f2.await);
    });
    eprintln!("after spawn f2");

    eprintln!("before run");
    rt.run();
    eprintln!("after run");
}

mod wait {
    use std::{cell::RefCell, collections::BinaryHeap, io, thread_local, time::Duration};

    use mio::{Events, Poll};

    thread_local! {
        pub static THREAD_WAITER: RefCell<Waiter> = RefCell::new(Waiter::new().unwrap());
    }

    pub struct Waiter {
        poll: Poll,
        events: Events,
        queue: BinaryHeap<Duration>,
    }

    impl Waiter {
        pub fn new() -> io::Result<Self> {
            Ok(Waiter {
                poll: Poll::new()?,
                events: Events::with_capacity(8),
                queue: BinaryHeap::new(),
            })
        }

        pub fn push_next_wake(&mut self, req: Duration) {
            eprintln!("--->>WAIT: push_next_wake {:?}", req);
            self.queue.push(req);
        }

        pub fn wait(&mut self) {
            eprintln!("QUEUELEN: {}", self.queue.len());
            let heap = self.queue.to_owned();
            let mut v = heap.into_sorted_vec();
            v.reverse();
            let dur = v.pop();
            self.queue = BinaryHeap::from(v);
            eprintln!("QUEUELEN: {}", self.queue.len());
            eprintln!("=====>>QUEUE: {:?}", self.queue);
            eprintln!("WAITER: queue.len():{:?}, dur:{:?}", self.queue.len(), dur);
            if self.queue.is_empty() {
                return;
            }
            self.poll.poll(&mut self.events, dur).unwrap();
            self.queue = BinaryHeap::new(); //TODO?
            eprintln!("WAITER: after poll");
        }
    }
}

mod run {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    };

    use crate::wait::THREAD_WAITER;

    pub struct Task<T> {
        fut: Pin<Box<dyn Future<Output = T>>>,
    }

    impl<T> Future for Task<T> {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<T> {
            self.fut.as_mut().poll(cx)
        }
    }

    pub struct Runner {
        tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
        waiting: Vec<Pin<Box<dyn Future<Output = ()>>>>,
    }

    impl Runner {
        pub fn new() -> Self {
            Runner {
                tasks: Vec::new(),
                waiting: Vec::new(),
            }
        }

        pub fn spawn<F: Future + 'static>(&mut self, f: F) -> Task<F::Output> {
            let (tx, rx) = tokio::sync::oneshot::channel();
            let fut = async move {
                let _ = tx.send(f.await);
            };
            self.tasks.push(Box::pin(fut));
            Task {
                fut: Box::pin(async move { rx.await.unwrap() }),
            }
        }

        pub fn run(mut self) {
            loop {
                eprintln!("RUN: about to drain");
                for mut task in self.tasks.drain(..) {
                    unsafe {
                        let waker = Waker::from_raw(new_raw_waker());
                        let mut context = Context::from_waker(&waker);
                        match task.as_mut().poll(&mut context) {
                            Poll::Pending => self.waiting.push(task),
                            Poll::Ready(()) => {}
                        }
                    }
                }
                eprintln!("RUN: about to wait");
                THREAD_WAITER.with(|waiter| {
                    eprintln!("RUN: about to wait INNER");
                    waiter.borrow_mut().wait();
                });
                if self.tasks.is_empty() && self.waiting.is_empty() {
                    break;
                } else {
                    std::mem::swap(&mut self.tasks, &mut self.waiting);
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

mod time {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
        time::{Duration, Instant},
    };

    use crate::wait::THREAD_WAITER;

    pub async fn sleep(duration: Duration) {
        let timer = Timer::new(duration);
        timer.await;
    }

    struct Timer {
        deadline: Instant,
    }

    impl Timer {
        fn new(duration: Duration) -> Self {
            Timer {
                deadline: Instant::now() + duration,
            }
        }
    }

    impl Future for Timer {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            let now = Instant::now();
            if now >= self.deadline {
                Poll::Ready(())
            } else {
                THREAD_WAITER.with(|waiter| {
                    waiter.borrow_mut().push_next_wake(self.deadline - now);
                });
                Poll::Pending
            }
        }
    }
}
