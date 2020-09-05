use std::str::FromStr;

pub enum Method {
    GET,
    HEAD,
    POST,
    DELETE,
    PUT,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}


impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "DELETE" => Ok(Self::DELETE),
            "PUT" => Ok(Self::PUT),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ =>  Err(MethodError)
        }
    }
}

pub struct MethodError;

