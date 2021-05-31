use std::{process::Command, time::Duration};

use io::CmdReader;
use run::Runner;
use time::sleep;

fn main() {
    let mut rt = Runner::new();

    let f1 = async {
        for _ in 0..30 {
            println!("f1");
            sleep(Duration::from_millis(999)).await;
        }
        42
    };

    let f2 = async {
        for _ in 0..90 {
            println!("f2");
            sleep(Duration::from_millis(333)).await;
        }
        "47"
    };

    let f3 = async {
        let mut buf = vec![0u8; 1024];
        let mut cmd = CmdReader::new(Command::new("cat").arg("named_pipe"));

        while let Ok(len) = cmd.read(&mut buf).await {
            if len == 0 {
                break;
            }
            print!("{}", String::from_utf8(buf[0..len].to_vec()).unwrap());
        }
    };

    rt.spawn(async move {
        println!("{:?}", f1.await);
        println!("f1 done");
    });

    rt.spawn(async move {
        println!("{:?}", f2.await);
        println!("f2 done");
    });

    rt.spawn(async move {
        f3.await;
        println!("f3 done");
    });

    rt.run();
}

mod io {
    use std::{
        future::Future,
        io::{ErrorKind, Read, Result},
        os::unix::io::AsRawFd,
        pin::Pin,
        process::{ChildStdout, Command, Stdio},
        task::{Context, Poll},
    };

    use mio::{event::Source, unix::SourceFd, Interest, Registry, Token};

    use crate::wait::WAITER;

    pub struct CmdReader {
        stdout: ChildStdout,
        registered: bool,
    }

    impl CmdReader {
        pub fn new(cmd: &mut Command) -> Self {
            let mut command = cmd.stdout(Stdio::piped()).spawn().unwrap();
            let stdout = command.stdout.take().unwrap();
            let fd = stdout.as_raw_fd();

            unsafe {
                let mut flags = libc::fcntl(fd, libc::F_GETFL);
                flags |= libc::O_NONBLOCK;
                libc::fcntl(fd, libc::F_SETFL, flags);
            }

            CmdReader {
                stdout,
                registered: false,
            }
        }

        pub async fn read<'a>(&'a mut self, buf: &'a mut Vec<u8>) -> Result<usize> {
            Reader::new(self, buf).await
        }
    }

    impl Source for CmdReader {
        fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
        ) -> Result<()> {
            SourceFd(&self.stdout.as_raw_fd()).register(registry, token, interests)
        }

        fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
        ) -> Result<()> {
            SourceFd(&self.stdout.as_raw_fd()).reregister(registry, token, interests)
        }

        fn deregister(&mut self, registry: &Registry) -> Result<()> {
            SourceFd(&self.stdout.as_raw_fd()).deregister(registry)
        }
    }

    pub struct Reader<'a> {
        cmd_reader: &'a mut CmdReader,
        buf: &'a mut Vec<u8>,
        pub registered: bool,
    }

    impl<'a> Reader<'a> {
        fn new(cmd_reader: &'a mut CmdReader, buf: &'a mut Vec<u8>) -> Self {
            Reader {
                cmd_reader,
                buf,
                registered: false,
            }
        }
    }

    impl<'a> Future for Reader<'a> {
        type Output = Result<usize>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<usize>> {
            let me = &mut *self;

            match me.cmd_reader.stdout.read(me.buf) {
                Ok(len) => {
                    Poll::Ready(Ok(len))
                }
                Err(err) => {
                    if err.kind() == ErrorKind::WouldBlock {
                        WAITER.with(|waiter| {
                            let mut waiter = waiter.borrow_mut();
                            let token = Token(waiter.next_tok);

                            if me.cmd_reader.registered {
                                waiter
                                    .poll
                                    .registry()
                                    .reregister(me.cmd_reader, token, Interest::READABLE)
                                    .unwrap();
                            } else {
                                waiter
                                    .poll
                                    .registry()
                                    .register(me.cmd_reader, token, Interest::READABLE)
                                    .unwrap();
                                me.cmd_reader.registered = true;
                            }

                            waiter.push_io_waker(token, cx.waker().clone());
                            waiter.next_tok += 1;
                        });
                        Poll::Pending
                    } else {
                        Poll::Ready(Err(err))
                    }
                }
            }
        }
    }
}

mod wait {
    use std::{
        cell::RefCell,
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
        io,
        task::Waker,
        thread_local,
        time::Instant,
    };

    use mio::{Events, Poll, Token};

    use crate::time::TimerWaker;

    thread_local! {
        pub static WAITER: RefCell<Waiter> = RefCell::new(Waiter::new().unwrap());
    }

    #[derive(PartialEq)]
    pub enum WaitStatus {
        Running,
        Done,
    }

    pub struct Waiter {
        pub poll: Poll,
        events: Events,
        timers: BinaryHeap<Reverse<TimerWaker>>,
        pub next_tok: usize,
        readers: HashMap<Token, Waker>,
    }

    impl Waiter {
        pub fn new() -> io::Result<Self> {
            Ok(Waiter {
                poll: Poll::new()?,
                events: Events::with_capacity(8),
                timers: BinaryHeap::new(),
                next_tok: 0,
                readers: HashMap::new(),
            })
        }

        pub fn push_timer_waker(&mut self, timer_waker: TimerWaker) {
            self.timers.push(Reverse(timer_waker));
        }

        pub fn push_io_waker(&mut self, token: Token, waker: Waker) {
            self.readers.insert(token, waker);
        }

        pub fn wait(&mut self) -> WaitStatus {
            if self.timers.is_empty() && self.readers.is_empty() {
                return WaitStatus::Done;
            }

            let mut timer_waker = None;
            let mut timeout = None;

            if let Some(tw) = self.timers.pop() {
                timeout = Some(tw.0.deadline.saturating_duration_since(Instant::now()));
                timer_waker = Some(tw);
            }

            self.poll.poll(&mut self.events, timeout).unwrap();

            for event in self.events.iter() {
                if self.readers.contains_key(&event.token()) {
                    self.readers.get(&event.token()).unwrap().clone().wake();
                    self.readers.remove(&event.token()).unwrap();
                }
            }

            if let Some(tw) = timer_waker {
                if tw.0.deadline < Instant::now() {
                    tw.0.waker.wake();
                } else {
                    self.timers.push(tw);
                }
            }

            WaitStatus::Running
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

    #[derive(Clone, Debug)]
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
                Poll::Ready(())
            } else {
                WAITER.with(|waiter| {
                    waiter.borrow_mut().push_timer_waker(TimerWaker {
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

    use crate::wait::{WaitStatus, WAITER};

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
        queue_rx: channel::Receiver<Arc<Task>>,
        queue_tx: channel::Sender<Arc<Task>>,
    }

    impl Runner {
        pub fn new() -> Self {
            let (queue_tx, queue_rx) = channel::unbounded();
            Runner { queue_rx, queue_tx }
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
                while let Some(task) = self.queue_rx.try_iter().next() {
                    let waker = futures::task::waker(task.clone());
                    let mut context = Context::from_waker(&waker);
                    let mut fut = task.fut.lock().unwrap();

                    match fut.as_mut().poll(&mut context) {
                        Poll::Pending => {}
                        Poll::Ready(()) => {}
                    }
                }

                let mut status: WaitStatus = WaitStatus::Running;

                WAITER.with(|waiter| {
                    status = waiter.borrow_mut().wait();
                });

                if status == WaitStatus::Done {
                    break;
                }
            }
        }
    }
}
