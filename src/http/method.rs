use std::str::FromStr;
use strum_macros::EnumString;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, EnumString)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}
