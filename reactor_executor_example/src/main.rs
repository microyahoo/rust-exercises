// use mio::net::{TcpListener, TcpStream};
// use mio::{Events, Interest, Poll, Token};

// https://github.com/cfsamson/examples-minimio
// https://cfsamsonbooks.gitbook.io/epoll-kqueue-iocp-explained/appendix-1/reactor-executor-pattern
use minimio::{Events, Interests, Poll, Registrator, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{io, io::Read, io::Write, thread};

use crossbeam::channel::{unbounded, Sender as crossSender};

const TEST_TOKEN: usize = 10;

fn main() {
    let (sender, receiver) = unbounded::<async_task::Task<()>>();
    dbg!(sender);
    dbg!(receiver);

    let (evt_sender, evt_receiver) = channel();
    let reactor = Reactor::new(evt_sender);
    let mut executor = Excutor::new(evt_receiver);

    let mut stream = TcpStream::connect("slowwly.robertomurray.co.uk:80").unwrap();
    let request = b"GET /delay/1000/url/http://www.google.com HTTP/1.1\r\nHost: slowwly.robertomurray.co.uk\r\nConnection: close\r\n\r\n";

    stream.write_all(request).expect("Stream write err.");
    reactor.register_stream_read_interest(&mut stream, TEST_TOKEN);

    executor.suspend(TEST_TOKEN, move || {
        let mut buffer = String::new();
        stream.read_to_string(&mut buffer).unwrap();
        println!("************{}", buffer);
        assert!(!buffer.is_empty(), "Got an empty buffer");
        reactor.stop_loop();
    });

    executor.block_on_all();
    // NB! Best practice is to make sure to join our child thread. We skip it here for brevity.
    println!("EXITING");
}

struct Reactor {
    handle: std::thread::JoinHandle<()>,
    registrator: Registrator,
}

impl Reactor {
    fn new(evt_sender: Sender<usize>) -> Reactor {
        let mut poll = Poll::new().unwrap();
        let registrator = poll.registrator();

        // Set up the epoll/IOCP event loop in a seperate thread
        let handle = thread::spawn(move || {
            let mut events = Events::with_capacity(1024);
            loop {
                println!("Waiting! {:?}", poll);
                match poll.poll(&mut events, Some(200)) {
                    Ok(..) => (),
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                        println!("INTERRUPTED: {}", e);
                        break;
                    }
                    Err(e) => panic!("Poll error: {:?}, {}", e.kind(), e),
                };
                for event in &events {
                    let event_token = event.id();
                    evt_sender.send(event_token).expect("send event_token err.");
                }
            }
        });

        Reactor {
            handle,
            registrator,
        }
    }

    fn register_stream_read_interest(&self, stream: &mut TcpStream, token: usize) {
        self.registrator
            .register(stream, token, Interests::READABLE)
            .expect("registration err.");
    }

    fn stop_loop(&self) {
        self.registrator.close_loop().expect("close loop err.");
    }
}

struct Excutor {
    events: Vec<(usize, Box<dyn FnMut()>)>,
    evt_receiver: Receiver<usize>,
}

impl Excutor {
    fn new(evt_receiver: Receiver<usize>) -> Self {
        Excutor {
            events: vec![],
            evt_receiver,
        }
    }
    fn suspend(&mut self, id: usize, f: impl FnMut() + 'static) {
        self.events.push((id, Box::new(f)));
    }
    fn resume(&mut self, event: usize) {
        println!("RESUMING TASK: {}", event);
        let (_, f) = self
            .events
            .iter_mut()
            .find(|(e, _)| *e == event)
            .expect("Couldn't find event.");
        f();
    }
    fn block_on_all(&mut self) {
        while let Ok(received_token) = self.evt_receiver.recv() {
            assert_eq!(TEST_TOKEN, received_token, "Non matching tokens.");
            println!("EVENT: {} is ready", received_token);
            self.resume(received_token);
        }
    }
}

// enum Forward<T> {
//     WaitingForReceive(ReceiveFuture<T>, Option<Sender<T>>),
//     WaitingForSend(SendFuture<T>, Option<Receiver<T>>),
// }

// impl<T> Future for Forward<T> {
//     type Output = (); // 2
//     fn poll(&mut self) -> Poll<Self::Output> {
//         match self {
//             Forward::WaitingForReceive(recv, tx) => {
//                 if let Poll::Ready((rx, v)) = recv.poll() {
//                     if let Some(v) = v {
//                         let tx = tx.take().unwrap(); //4
//                         *self = Forward::WaitingForSend(tx.send(v), Some(rx));
//                         // Try to make progress on sending.
//                         return self.poll(); //6
//                     } else {
//                         // No more items.
//                         Poll::Ready(())
//                     }
//                 } else {
//                     Poll::Pending
//                 }
//             }
//             Forward::WaitingForSend(send, rx) => {
//                 if let Poll::Ready(tx) = send.poll() {
//                     let rx = rx.take().unwrap();
//                     *self = Forward::WaitingForReceive(rx.receive(), Some(tx));
//                     // Try to make progress on receiving.
//                     return self.poll();
//                 } else {
//                     Poll::Pending
//                 }
//             }
//         }
//     }
// }
