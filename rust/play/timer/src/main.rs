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

    enum Event {
        Timer,
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
            eprintln!("--->>WAIT: push_next_wake {:?}", req - Instant::now());
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
            eprintln!("---GOT timeout:{:?}", timeout - Instant::now());
            self.timers = BinaryHeap::from(v);

            eprintln!("QUEUELEN_2: {}", self.timers.len());
            eprintln!(
                "=====>>QUEUE: {:?}",
                self.timers
                    .iter()
                    .map(|x| *x - Instant::now())
                    .collect::<Vec<Duration>>()
            );
            eprintln!(
                "WAITER: queue.len():{:?}, dur:{:?}",
                self.timers.len(),
                timeout - Instant::now()
            );

            let now = Instant::now();
            let mut dur = Duration::from_millis(0);
            if now < timeout {
                dur = timeout - now;
            }

            eprintln!("BEFORE: {:?}", dur);
            self.poll.poll(&mut self.events, Some(dur)).unwrap();
            eprintln!("AFTER: {:?}", dur);
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
        Timer::new(duration).await;
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
            let now = Instant::now();
            if now >= self.deadline {
                eprintln!("TIMER__POLL__READY");
                Poll::Ready(())
            } else {
                eprintln!("TIMER__POLL__PENDING: {:?}", self.deadline - now);
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
        task::{Context, Poll}, // , RawWaker, RawWakerVTable, Waker},
    };

    use crossbeam::channel;
    use futures::{channel::oneshot, task::ArcWake};

    use crate::wait::WAITER;

    struct Task {
        fut: Mutex<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
        queue_tx: channel::Sender<Arc<Task>>,
    }

    impl ArcWake for Task {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            arc_self.queue_tx.send(arc_self.clone()).unwrap();
        }

        fn wake(self: Arc<Self>) {
            ArcWake::wake_by_ref(&self);
        }
    }

    pub struct TaskHandle<T> {
        fut: Mutex<Pin<Box<dyn Future<Output = T> + Send>>>,
    }

    impl<T> Future for TaskHandle<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<T> {
            self.fut.lock().unwrap().as_mut().poll(cx)
        }
    }

    pub struct Runner {
        waiting: Vec<Arc<Task>>,
        queue_rx: channel::Receiver<Arc<Task>>,
        queue_tx: channel::Sender<Arc<Task>>,
    }

    impl Runner {
        pub fn new() -> Self {
            let (queue_tx, queue_rx) = channel::unbounded();
            Runner {
                waiting: Vec::new(),
                queue_tx,
                queue_rx,
            }
        }

        pub fn spawn<T, F>(&mut self, f: F) -> TaskHandle<T>
        where
            T: Send + 'static,
            F: Future<Output = T> + Send + Sync + 'static,
        {
            let (tx, rx) = oneshot::channel();
            let fut = async move {
                let _ = tx.send(f.await);
            };

            let task = Task {
                fut: Mutex::new(Box::pin(fut)),
                queue_tx: self.queue_tx.clone(),
            };

            self.queue_tx.send(Arc::new(task)).unwrap();

            TaskHandle {
                fut: Mutex::new(Box::pin(async move { rx.await.unwrap() })),
            }
        }

        pub fn run(mut self) {
            loop {
                eprintln!("RUN: about to drain");
                while let Some(task) = self.queue_rx.try_iter().next() {
                    eprintln!("RUN: POLLING TASK");
                    //unsafe {
                    {
                        // let waker = Waker::from_raw(new_raw_waker());
                        let waker = futures::task::waker(task.clone());
                        let mut context = Context::from_waker(&waker);
                        let mut fut = task.fut.lock().unwrap();
                        match fut.as_mut().poll(&mut context) {
                            Poll::Pending => {
                                eprintln!("RUN: POLLED-->PENDING");
                                self.waiting.push(task.clone());
                            }
                            Poll::Ready(()) => eprintln!("RUN: POLLED-->READY"),
                        }
                    }
                }

                WAITER.with(|waiter| {
                    eprintln!("RUN: about to wait INNER");
                    waiter.borrow_mut().wait();
                });

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

    /*
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
    */
}
