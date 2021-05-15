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

    rt.spawn(f1);
    rt.spawn(f2);
    rt.run();
}

mod wait {
    use std::{io, task::Waker, thread_local, time::Duration};

    use mio::{Events, Poll};

    thread_local! {
        pub static THREAD_WAITER: Waiter<'static> = Waiter::new().unwrap();
    }

    pub struct Waiter<'a> {
        poll: Poll,
        events: Events,
        queue: Vec<(&'static str, Duration, &'a Waker)>,
    }

    impl Waiter<'_> {
        pub fn new() -> io::Result<Self> {
            Ok(Waiter {
                poll: Poll::new()?,
                events: Events::with_capacity(8),
                queue: Vec::new(),
            })
        }

        pub fn push(&self, req: (&str, Duration, &Waker)) {

        }

        pub fn wait(&mut self) {
            self.poll.poll(&mut self.events, Some(Duration::from_millis(1477)));
        }
    }
}

mod run {
    use std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

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
        tasks: Vec<Box<dyn Future<Output = ()>>>,
    }

    impl Runner {
        pub fn new() -> Self {
            Runner { tasks: Vec::new() }
        }

        pub fn spawn<F: Future>(&mut self, f: F) -> Task<F::Output>
        {
            self.tasks.push(Box::new(f));
        }

        pub fn run(&self) {}
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

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if Instant::now() >= self.deadline {
                Poll::Ready(())
            } else {
                let waker = cx.waker();
                THREAD_WAITER.with(|waiter| {
                    waiter.push(("timer", self.deadline - Instant::now(), waker));
                });
                Poll::Pending
            }
        }
    }
}
