use std::net::TcpListener;
use std::io::Read;
use crate::http::Request;
use std::convert::TryFrom;

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
                    match stream.read(&mut bytes) {
                        Ok(_) => {
                            match Request::try_from(&bytes[..]) {
                                Ok(request) => {
                                    
                                },
                                Err(e) => {
                                    println!("[Error: ]  {}", e)
                                }
                            }
                            // create Request here   
                        },
                        Err(e) => println!("{}", e)
                    }
                },
                Err(e) => println!(" Error: {}", e)
            }
        };
    }
}