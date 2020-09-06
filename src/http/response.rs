use super::StatusCode;
use std::io::{Result as IoResult, Write};

pub struct Response {
    body: Option<String>,
    status_code: StatusCode,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.get_reason(),
            body
        )
    }
}
