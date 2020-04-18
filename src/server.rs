use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};

const BUFSIZE: usize = 4096;
const MAXEVENTS: usize = 1024;

pub struct Client {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Client {
    pub fn new(socket: TcpStream) -> Client {
        Client {
            stream: socket,
            buffer: Vec::new(),
        }
    }
    pub fn dump_buffer(&mut self, buffer: &[u8; BUFSIZE], n: usize) {
        for b in &buffer[0..n] {
            self.buffer.push(*b);
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn send(&mut self) -> Result<(), Error> {
        return self.stream.write_all(&self.buffer);
    }

    pub fn register_read(&mut self, poll: &mut Poll, token: Token) {
        poll.registry()
            .register(&mut self.stream, token, Interest::READABLE)
            .unwrap();
    }

    pub fn register_write(&mut self, poll: &mut Poll, token: Token) {
        poll.registry()
            .register(&mut self.stream, token, Interest::WRITABLE)
            .unwrap();
    }
}

pub struct Server {
    addr: String,
    port: i32,
    connections: HashMap<Token, Client>,
}

impl Server {
    pub fn new(addr: String, port: i32) -> Server {
        Server {
            addr,
            port,
            connections: HashMap::new(),
        }
    }

    fn to_addr(&self) -> std::net::SocketAddr {
        return format!("{}:{}", self.addr, self.port).parse().unwrap();
    }

    pub fn run(&mut self) -> Result<(), Error> {
        let mut counter = 0;
        let mut buffer = [0 as u8; BUFSIZE];
        let mut listener = TcpListener::bind(self.to_addr()).unwrap();
        // Poll interface will take care of choosing the right IO multiplexing implementation found
        // on the host
        let mut poll = Poll::new().unwrap();
        // Register the listener socket for read events
        poll.registry()
            .register(&mut listener, Token(0), Interest::READABLE)
            .unwrap();
        let mut events = Events::with_capacity(MAXEVENTS);
        loop {
            // Blocking call, wait for kernel to notify sockets to be ready for read/write
            poll.poll(&mut events, None)?;
            for event in events.iter() {
                match event.token() {
                    Token(0) => loop {
                        // A new connection (possibly more than one) arrived, we accept it and
                        // track it inserting it into the server hashmap
                        match listener.accept() {
                            Ok((mut socket, _)) => {
                                counter += 1;
                                let token = Token(counter);
                                let mut client = Client::new(socket);
                                client.register_read(&mut poll, token);
                                self.connections.insert(token, client);
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                            Err(_) => break,
                        }
                    },
                    token if event.is_readable() => loop {
                        // Some data arrived to be read from the socket, we drain the kernel queue
                        // into the buffer till we're signaled with an EAGAIN/EWOULDBLOCK error or
                        // a 0 return (which imply client closed the connection)
                        let read = self
                            .connections
                            .get_mut(&token)
                            .unwrap()
                            .stream
                            .read(&mut buffer);
                        match read {
                            // Connection closed
                            Ok(0) => {
                                self.connections.remove(&token);
                                break;
                            }
                            // We copy n read bytes into the client buffer
                            Ok(n) => {
                                let client = self.connections.get_mut(&token).unwrap();
                                client.dump_buffer(&buffer, n);
                            }
                            Err(ref e) if e.kind() == ErrorKind::WouldBlock => break,
                            Err(_) => break,
                        }
                    },
                    token if event.is_writable() => {
                        let client = self.connections.get_mut(&token).unwrap();
                        client.send().unwrap();
                        // Re-use existing connection, switch back to reading wait
                        client.register_read(&mut poll, token);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
