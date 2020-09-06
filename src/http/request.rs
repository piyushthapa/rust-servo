use super::{Method, MethodError};
use crate::http::QueryString;
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'a> {
    path: &'a str,
    query_string: Option<QueryString<'a>>,
    method: Method,
}

impl<'a> Request<'a> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = RequestError;

    fn try_from(buff: &'a [u8]) -> Result<Request<'a>, Self::Error> {
        // TODO:  parse Request
        // GET /some_path HTTP/1.1\r\n...SOME_HEADERS...

        let request = str::from_utf8(buff)?;
        let (method, request) =
            fetch_next_word(request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (path, request) = fetch_next_word(request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = fetch_next_word(request, '\r').ok_or(RequestError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(RequestError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some((q, _)) = fetch_next_word(path, '?') {
            query_string = Some(QueryString::from(q));
        }

        Ok(Request {
            path,
            query_string,
            method,
        })
    }
}

fn fetch_next_word(request: &str, pattern: char) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == pattern {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum RequestError {
    InvalidMethod,
    InvalidEncodding,
    InvalidProtocol,
    InvalidRequest,
}

impl RequestError {
    fn get_msg(&self) -> &str {
        match self {
            RequestError::InvalidEncodding => "Invalid Encoding",
            RequestError::InvalidMethod => "Invalid Method",
            RequestError::InvalidProtocol => "Invalid Protocol",
            RequestError::InvalidRequest => "Invalid Request",
        }
    }
}

impl Error for RequestError {}

impl From<MethodError> for RequestError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncodding
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_msg())
    }
}

impl Debug for RequestError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_msg())
    }
}
