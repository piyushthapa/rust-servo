use super::{Method, MethodError};
use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};
use std::str;
use std::str::Utf8Error;

pub struct Request {
    path: String,
    query_string: String,
    method: Option<Method>,
}



impl TryFrom<&[u8]> for Request {
    type Error = RequestError;

    fn try_from(buff: &[u8]) -> Result<Self, Self::Error> {
        // TODO:  parse Request 
        // GET /some_path HTTP/1.1\r\n...SOME_HEADERS...

        let request = str::from_utf8(buff)?;
        let (method, request) = fetch_next_word(request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (path, request) = fetch_next_word(request, ' ').ok_or(RequestError::InvalidRequest)?;
        let (protocol, _) = fetch_next_word(request, '\r').ok_or(RequestError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(RequestError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        unimplemented!()
    }
}

fn fetch_next_word(request: &str, pattern: char) -> Option<(&str, &str)>{
    for (i, c) in request.chars().enumerate() {
        if c == pattern {
            return Some((&request[..i], &request[i+1..]));
        }
    };

    None
}

pub enum RequestError {
    InvalidMethod,
    InvalidPath,
    InvalidEncodding, 
    InvalidProtocol,
    InvalidRequest,
}

impl RequestError {
    fn get_msg(&self) -> &str {
        match self {
            RequestError::InvalidEncodding => "Invalid Encoding",
            RequestError::InvalidPath => "Invalid Path",
            RequestError::InvalidMethod => "Invalid Method",
            RequestError::InvalidProtocol => "Invalid Protocol",
            RequestError::InvalidRequest => "Invalid Request",
        }
    }
}

impl Error for RequestError{}

impl From<MethodError> for RequestError {
    fn from(_: MethodError) -> Self{
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for RequestError {
    fn from(_: Utf8Error) -> Self{
        Self::InvalidEncodding
    }
}

impl Display for RequestError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_msg())
    }
}

impl Debug for RequestError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.get_msg())
    }
}
