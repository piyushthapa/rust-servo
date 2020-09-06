use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;
pub struct WebsiteHandler {
    base_path: String,
}

impl WebsiteHandler {
    pub fn new(path: &str) -> Self {
        Self {
            base_path: format!("{}/{}", path, "public"),
        }
    }

    fn read_file(&self, name: &str) -> Option<String> {
        let path = format!("{}{}", self.base_path, name);
        match fs::canonicalize(path) {
            Ok(path) => fs::read_to_string(path).ok(),
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("/index.html")),
                "/contact" => Response::new(StatusCode::Ok, self.read_file("/contact.html")),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
