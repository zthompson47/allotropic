#![allow(dead_code)]
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn main() {
    let timer = Timer::new(4);
    let func = async {
        let _ = testit("asdf");
        testit("hello").await;
        timer.await;
        let data = Read::new().await;
        println!("******************GOT DATA*******{:?}", data);
        if data.is_empty() {
            42
        } else {
            data.len()
        }
    };

    println!("---- Run func - async block --------");
    let result = run(func);
    println!("====================!!>> GOT___RESULT_____:{:?}", result);
    /*
    println!("---- Run func - async fn -----------");
    let result = run(some_async());
    println!("====================!!>> GOT___RESULT22_____:{:?}", result);
    */
}

struct Timer {
    duration: usize,
    elapsed: usize,
}

impl Timer {
    fn new(duration: usize) -> Self {
        Timer {
            duration,
            elapsed: 0,
        }
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.elapsed >= self.duration {
            println!(">>>>>>>GETTIN PLLED<<<<<<<READY!!<<<<<<<");
            Poll::Ready(())
        } else {
            println!(">>>>>>>GETTIN PLLED<<<<PEND<<<<<<<<<<");
            self.elapsed += 1;
            Poll::Pending
        }
    }
}

fn run<F, T>(fut: F) -> T
where
    F: Future<Output = T> + 'static,
    T: std::fmt::Debug + 'static,
{
    let mut ex: Runner<T> = Runner::new();
    ex.spawn(Box::pin(fut));
    ex.run()
}

struct Waiter {
    poll: mio::Poll,
    events: mio::Events,
}

impl Waiter {
    fn new() -> Self {
        Waiter {
            poll: mio::Poll::new().unwrap(),
            events: mio::Events::with_capacity(1024),
        }
    }
}

struct Read {
    cursor: usize,
    buf: Vec<u8>,
}

impl Read {
    fn new() -> Self {
        Read {
            cursor: 0,
            buf: vec![9, 9, 5, 4, 6, 2, 1],
        }
    }
}

impl Future for Read {
    type Output = Vec<u8>;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Vec<u8>> {
        println!(">>>>>>>>GETTIN READ<<<<<<<<<<<<<<");
        if self.cursor == 7 {
            Poll::Ready(self.buf.clone())
        } else {
            self.cursor += 1;
            Poll::Pending
        }
    }
}

struct Runner<T> {
    ready: Vec<Pin<Box<dyn Future<Output = T>>>>,
    waiting: Vec<Pin<Box<dyn Future<Output = T>>>>,
}

impl<T> Runner<T>
where
    T: std::fmt::Debug + 'static,
{
    fn new() -> Self {
        Runner {
            ready: Vec::new(),
            waiting: Vec::new(),
        }
    }

    fn spawn(&mut self, task: Pin<Box<dyn Future<Output = T>>>) {
        self.ready.push(task);
    }

    fn run(mut self) -> T {
        loop {
            println!(
                "___________ready:{}_______waiting:{}_____________",
                self.ready.len(),
                self.waiting.len()
            );
            for mut fut in self.ready.drain(..) {
                unsafe {
                    let waker = Waker::from_raw(new_raw_waker());
                    let mut context = Context::from_waker(&waker);
                    // let mut pinned_future = Box::pin(fut);
                    match fut.as_mut().poll(&mut context) {
                        Poll::Pending => {
                            self.waiting.push(Box::pin(fut));
                            println!("-->Pending");
                        }
                        Poll::Ready(result) => {
                            println!("-->Ready({:?})", result);
                            return result;
                        }
                    }
                }
            }
            if self.ready.is_empty() {
                if self.waiting.is_empty() {
                    panic!();
                } else {
                    println!("PARKING>>>");
                    std::thread::park_timeout(std::time::Duration::from_millis(100));
                    std::mem::swap(&mut self.ready, &mut self.waiting);
                    println!("UN-PARKING>>>");
                }
            }
        }
    }
}

async fn testit(s: &str) {
    println!(">>>>> testit, {}!!!!!!", s);
}

async fn some_async() -> u32 {
    println!(">>>>> Some async function");
    47
}

fn new_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

unsafe fn clone(_data: *const ()) -> RawWaker {
    println!("**clone!");
    new_raw_waker()
}

unsafe fn wake(data: *const ()) {
    println!("**wake!");
    wake_by_ref(data);
}

unsafe fn wake_by_ref(_data: *const ()) {
    println!("**wake_by_ref!");
}

unsafe fn drop(_data: *const ()) {
    println!("**drop!");
}
