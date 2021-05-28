//#![allow(unused_imports)]
//#![allow(dead_code)]
use std::{process::Command, time::Duration};

use io::CmdReader;
use run::Runner;
use time::sleep;

fn main() {
    let mut rt = Runner::new();

    let f1 = async {
        for _ in 0..3000 {
            println!("f11");
            sleep(Duration::from_millis(999)).await;
        }
        42
    };

    let f2 = async {
        for _ in 0..9000 {
            println!("f2");
            sleep(Duration::from_millis(333)).await;
        }
        "47"
    };

    let f3 = async {
        let mut buf = vec![0u8; 1024];
        let mut cmd = CmdReader::new(Command::new("cat").arg("named_pipe"));
        // println!("about to read");
        while let Ok(len) = cmd.read(&mut buf).await {
            if len == 0 {
                break;
            }
            print!("{}", String::from_utf8(buf[0..len].to_vec()).unwrap());
        }
        /*
        match cmd.read(&mut buf).await {
            Ok(len) => {
                // println!("got OK");
                if len == 0 {
                    // println!("len 00000");
                }
                // println!("--->>{}", len);
                print!("{}", String::from_utf8(buf[0..len].to_vec()).unwrap());
            }
            Err(e) => println!("{:?}", e),
        }
        */
        // println!("AFTER about to read");
    };

    rt.spawn(async move {
        println!("{:?}", f1.await);
        // println!("f1 done");
    });

    rt.spawn(async move {
        println!("{:?}", f2.await);
        // println!("f2 done");
    });

    rt.spawn(async move {
        // println!("f3 starting");
        f3.await;
        // println!("f3 done");
    });

    rt.run();
}

mod io {
    use std::{
        future::Future,
        io::{ErrorKind, Read, Result},
        os::unix::io::AsRawFd,
        pin::Pin,
        process::{Child, ChildStdout, Command, Stdio},
        task::{Context, Poll},
    };

    use mio::{event::Source, unix::SourceFd, Interest, Registry, Token};

    use crate::wait::WAITER;

    pub struct CmdReader {
        #[allow(dead_code)]
        command: Child,
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

            let mut cmd_reader = CmdReader {
                command,
                stdout,
                registered: false,
            };

            WAITER.with(|waiter| {
                println!("++++++++++++++REGISTER++++++++++++++++++++");
                let waiter = waiter.borrow_mut();
                waiter
                    .poll
                    .registry()
                    .register(&mut cmd_reader, Token(waiter.next_tok), Interest::READABLE)
                    .unwrap();
                cmd_reader.registered = true;
            });

            cmd_reader
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

    /*
    impl Source for Reader<'_> {
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
    */

    impl<'a> Future for Reader<'a> {
        type Output = Result<usize>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<usize>> {
            let me = &mut *self;

            match me.cmd_reader.stdout.read(me.buf) {
                Ok(len) => {
                    println!("---READER--len:{}--READY", len);
                    Poll::Ready(Ok(len))
                }
                Err(err) => {
                    if err.kind() == ErrorKind::WouldBlock {
                        // println!("---READER---WouldBlock");
                        WAITER.with(|waiter| {
                            let mut waiter = waiter.borrow_mut();

                            if me.cmd_reader.registered {
                                println!("++++++++++++++REREGISTER!!++++++++++++++++++++");
                                waiter
                                    .poll
                                    .registry()
                                    .reregister(
                                        me.cmd_reader,
                                        Token(waiter.next_tok),
                                        Interest::READABLE,
                                    )
                                    .unwrap();
                            } else {
                                println!("++++++++++++++REGISTER++++++++++++++++++++");
                                waiter
                                    .poll
                                    .registry()
                                    .register(
                                        me.cmd_reader,
                                        Token(waiter.next_tok),
                                        Interest::READABLE,
                                    )
                                    .unwrap();
                                me.cmd_reader.registered = true;
                            }

                            waiter.push_reader(me, Interest::READABLE, cx.waker().clone());
                        });
                        Poll::Pending
                    } else {
                        println!("---READER---err{:?}", err);
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
        collections::{BinaryHeap, HashMap},
        io,
        task::Waker,
        thread_local,
        time::{Duration, Instant},
    };

    use mio::{event::Source, Events, Interest, Poll, Token};

    use crate::io::Reader;
    use crate::time::TimerWaker;

    thread_local! {
        pub static WAITER: RefCell<Waiter> = RefCell::new(Waiter::new().unwrap());
    }

    pub struct Waiter {
        pub poll: Poll,
        events: Events,
        timers: BinaryHeap<TimerWaker>,
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

        pub fn push_next_wake(&mut self, req: TimerWaker) {
            self.timers.push(req);
        }

        //pub fn push_reader(&mut self, source: &mut impl Source, interest: Interest, waker: Waker) {
        pub fn push_reader(&mut self, reader: &mut Reader, interest: Interest, waker: Waker) {
            println!("PUSH_READER..readers:{:?}", self.readers);
            println!("PUSH_READER..events:{:?}", self.events);

            let token = Token(self.next_tok);

            /*
            if reader.registered {
                println!("++++++++++++++REREGISTER!!++++++++++++++++++++");
                self.poll
                    .registry()
                    .reregister(reader, Token(self.next_tok), interest)
                    .unwrap();
            } else {
                println!("++++++++++++++REGISTER++++++++++++++++++++");
                self.poll
                    .registry()
                    .register(reader, Token(self.next_tok), interest)
                    .unwrap();
                reader.registered = true;
            }
            */

            self.readers.insert(token, waker);

            self.next_tok += 1;
        }

        pub fn wait(&mut self) -> bool {
            if self.timers.is_empty() && self.readers.is_empty() {
                return false;
            }

            // println!("[[[[TIMERS]]]]>>>>>{:?}", self.timers);

            /*
            let now = Instant::now();
            println!(
                "__timers__{:?}",
                self.timers
                    .iter()
                    .map(|x| x.deadline - now)
                    .collect::<Vec<Duration>>()
            );
            */

            // Get smallest timeout
            let mut timer_waker = None;
            let mut timeout = None;

            if !self.timers.is_empty() {
                let heap = self.timers.to_owned();
                let mut v = heap.into_sorted_vec();
                v.reverse();
                let timer_waker_inner = v.pop().unwrap();
                self.timers = BinaryHeap::from(v);

                let now = Instant::now();
                timeout = Some(Duration::from_millis(0));
                if now < timer_waker_inner.deadline {
                    timeout = Some(timer_waker_inner.deadline - now);
                    // println!("------{:?}-------->>>> TIMER_ON", dur);
                }
                timer_waker = Some(timer_waker_inner);
            }

            /*
            println!(
                "timers:{:?}",
                self.timers
                    .iter()
                    .map(|x| x.deadline - now)
                    .collect::<Vec<Duration>>()
            );
            */

            self.poll.poll(&mut self.events, timeout).unwrap();
            // println!("--EVENTS-->>{:?}", self.events);
            // println!("--READERS-->>{:?}", self.readers);
            for event in self.events.iter() {
                // println!("--EVENT-->>{:?}", event);
                if self.readers.contains_key(&event.token()) {
                    // println!("-BEFORE-GOT_KEY-->>{:?}", event);
                    self.readers.get(&event.token()).unwrap().clone().wake();
                    // println!("-AFTER-GOT_KEY-->>{:?}", event);
                    self.readers.remove(&event.token()).unwrap();
                }
            }

            if let Some(tw) = timer_waker {
                // TODO: only wake if needed..
                tw.waker.wake();
            }

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
            // println!("!!!!!!!!!!!{:?}", duration);
            let deadline = Instant::now() + duration;
            Timer { deadline }
        }
    }

    impl Future for Timer {
        type Output = ();

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            let now = Instant::now();
            if now >= self.deadline {
                // println!(">>>>>>>>TIMER READY");
                Poll::Ready(())
            } else {
                WAITER.with(|waiter| {
                    waiter.borrow_mut().push_next_wake(TimerWaker {
                        deadline: self.deadline,
                        waker: cx.waker().clone(),
                    });
                });
                // println!(">>>>>>>>TIMER PENDING");
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

                let mut wait_result = false;

                WAITER.with(|waiter| {
                    wait_result = waiter.borrow_mut().wait();
                });

                if !wait_result {
                    break;
                }
            }
        }
    }
}
