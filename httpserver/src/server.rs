use std::{io::Read, net::TcpListener};



pub struct Server<'a> {
    socket_addr: &'a str,
}

impl <'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self {
            socket_addr
        }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();
            let req = String::from_utf8(read_buffer)

        }
    }
}