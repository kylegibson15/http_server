use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::{Request};
use std::convert::{TryInto, TryFrom};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        print!("Application Running\n");
        print!("Listening on {}...\n", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    // let response = 
                                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n");
                                }
                                Err(e) => print!("Failed to parse a request: {}", e),
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => print!("Failed to read from connection: {}", e)
                    }
                }
                Err(e) => print!("Failed to establish a connection: {}", e)
            }
        }
    }
}
