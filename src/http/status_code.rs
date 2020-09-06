use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone)]
pub enum StatusCode {
    Ok = 200,
    NotFound = 404,
    BadRequest = 400,
    UnAuthorized = 401
}

impl StatusCode {
    pub fn get_reason(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::NotFound => "Not Found",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::UnAuthorized => "UnAuthorized"
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}