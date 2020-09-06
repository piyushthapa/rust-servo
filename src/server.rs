use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(addr: &str) -> Self {
        Self {
            address: addr.to_string(),
        }
    }

    pub fn run(&self) {
        println!(" listining on: {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _client_addr)) => {
                    let mut bytes = [0; 2046];
                    let response = match stream.read(&mut bytes) {
                        Ok(_) => match Request::try_from(&bytes[..]) {
                            Ok(_request) => Response::new(
                                StatusCode::NotFound,
                                Some("<h1> It Works</h1>".to_string()),
                            ),
                            Err(_e) => Response::new(StatusCode::BadRequest, Some("".to_string())),
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
