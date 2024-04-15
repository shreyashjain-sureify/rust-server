use crate::http::{Request, Response};
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler + Send + 'static) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        let handler = Arc::new(Mutex::new(handler));

        for stream in listener.incoming() {
            let handler = Arc::clone(&handler);
            match stream {
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_connection(stream, handler);
                    });
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}

fn handle_connection(stream: TcpStream, handler: Arc<Mutex<dyn Handler + Send>>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let request = match Request::try_from(&buffer[..]) {
                Ok(request) => request,
                Err(e) => {
                    let response = handler
                        .lock()
                        .unwrap()
                        .handle_bad_request(&e);
                    if let Err(e) = response.send(&stream) {
                        println!("Failed to send response: {}", e);
                    }
                    return;
                }
            };

            let response = handler.lock().unwrap().handle_request(&request);
            if let Err(e) = response.send(&stream) {
                println!("Failed to send response: {}", e);
            }
        }
        Err(e) => println!("Failed to read from connection: {}", e),
    }
}
