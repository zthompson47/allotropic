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

    use crate::time::TimerWaker;

    thread_local! {
        pub static WAITER: RefCell<Waiter> = RefCell::new(Waiter::new().unwrap());
    }

    pub struct Waiter {
        poll: Poll,
        events: Events,
        timers: BinaryHeap<TimerWaker>,
    }

    impl Waiter {
        pub fn new() -> io::Result<Self> {
            Ok(Waiter {
                poll: Poll::new()?,
                events: Events::with_capacity(8),
                timers: BinaryHeap::new(),
            })
        }

        pub fn push_next_wake(&mut self, req: TimerWaker) {
            self.timers.push(req);
        }

        pub fn wait(&mut self) -> bool {
            if self.timers.is_empty() {
                return false;
            }

            // Get smallest timeout
            let heap = self.timers.to_owned();
            let mut v = heap.into_sorted_vec();
            v.reverse();
            let timeout = v.pop().unwrap();
            self.timers = BinaryHeap::from(v);

            let now = Instant::now();
            let mut dur = Duration::from_millis(0);
            if now < timeout.deadline {
                dur = timeout.deadline - now;
            }

            eprintln!("BEFORE: {:?}", dur);
            self.poll.poll(&mut self.events, Some(dur)).unwrap();
            timeout.waker.wake();
            eprintln!("AFTER: {:?}", dur);

            true
        }
    }
}

mod time {
    use std::{
        cmp::Ordering,
        future::Future,
        pin::Pin,
        task::{Context, Poll, Waker},
        time::{Duration, Instant},
    };

    use crate::wait::WAITER;

    pub async fn sleep(duration: Duration) {
        Timer::new(duration).await;
    }

    struct Timer {
        deadline: Instant,
    }

    #[derive(Clone)]
    pub struct TimerWaker {
        pub deadline: Instant,
        pub waker: Waker,
    }

    impl Eq for TimerWaker {}

    impl PartialEq for TimerWaker {
        fn eq(&self, other: &Self) -> bool {
            self.deadline == other.deadline
        }
    }

    impl Ord for TimerWaker {
        fn cmp(&self, other: &Self) -> Ordering {
            self.deadline.cmp(&other.deadline)
        }
    }

    impl PartialOrd for TimerWaker {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Timer {
        fn new(duration: Duration) -> Self {
            let deadline = Instant::now() + duration;
            Timer { deadline }
        }
    }

    impl Future for Timer {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            let now = Instant::now();
            if now >= self.deadline {
                eprintln!("TIMER__POLL__READY");
                Poll::Ready(())
            } else {
                eprintln!("TIMER__POLL__PENDING: {:?}", self.deadline - now);
                WAITER.with(|waiter| {
                    waiter.borrow_mut().push_next_wake(TimerWaker {
                        deadline: self.deadline,
                        waker: cx.waker().clone(),
                    });
                });
                Poll::Pending
            }
        }
    }
}

mod run {
    use std::{
        cell::RefCell,
        future::Future,
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll},
    };

    use crossbeam::channel;
    use futures::{channel::oneshot, task::ArcWake};

    use crate::wait::WAITER;

    thread_local! {
        pub static RUNTIME: Option<RefCell<Runner>> = None;
    }

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
        // waiting: Vec<Arc<Task>>,
        queue_rx: channel::Receiver<Arc<Task>>,
        queue_tx: channel::Sender<Arc<Task>>,
    }

    impl Runner {
        pub fn new() -> Self {
            let (queue_tx, queue_rx) = channel::unbounded();
            Runner {
                // waiting: Vec::new(),
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

        pub fn run(self) {
            loop {
                eprintln!("RUN: about to drain");
                while let Some(task) = self.queue_rx.try_iter().next() {
                    eprintln!("RUN: POLLING TASK");
                    let waker = futures::task::waker(task.clone());
                    let mut context = Context::from_waker(&waker);
                    let mut fut = task.fut.lock().unwrap();

                    match fut.as_mut().poll(&mut context) {
                        Poll::Pending => {
                            eprintln!("RUN: POLLED-->PENDING");
                            //self.waiting.push(task.clone());
                        }
                        Poll::Ready(()) => eprintln!("RUN: POLLED-->READY"),
                    }
                }

                let mut wait_result = false;
                WAITER.with(|waiter| {
                    eprintln!("RUN: about to wait INNER");
                    wait_result = waiter.borrow_mut().wait();
                });
                if !wait_result {
                    break;
                }

                /*if self.waiting.is_empty() {
                    eprintln!("_______>> self.waiting.is_empty() <<_____________");
                    break;
                } else {
                    for t in self.waiting.drain(..) {
                        self.queue_tx.send(t).unwrap();
                    }
                }*/
            }
        }
    }
}
