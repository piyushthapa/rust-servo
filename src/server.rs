use crate::http::{Request, RequestError, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &RequestError) -> Response {
        println!("[error:] {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.to_string(),
        }
    }

    pub fn run(&self, mut handler: impl Handler) {
        println!(" listining on: {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _client_addr)) => {
                    let mut bytes = [0; 2046];
                    let response = match stream.read(&mut bytes) {
                        Ok(_) => match Request::try_from(&bytes[..]) {
                            Ok(request) => handler.handle_request(&request),
                            Err(e) => handler.handle_bad_request(&e),
                        },
                        Err(_e) => Response::new(StatusCode::BadRequest, Some("".to_string())),
                    };

                    response.send(&mut stream);
                }
                Err(e) => println!(" Error: {}", e),
            };
        }
    }
}
