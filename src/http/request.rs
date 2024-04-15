use super::{Method, QueryString};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
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

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, mut request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, protocol) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::try_from(&path[i + 1..])?);
            path = &path[..i];
        }

        Ok(Request {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.splitn(2, char::is_whitespace);
    let word = iter.next()?;
    let rest = iter.next()?;
    Some((word, rest))
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod(MethodError),
    InvalidEncoding(Utf8Error),
    InvalidQueryString,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::InvalidRequest => write!(f, "Invalid Request"),
            Self::InvalidProtocol => write!(f, "Invalid Protocol"),
            Self::InvalidMethod(_) => write!(f, "Invalid Method"),
            Self::InvalidEncoding(_) => write!(f, "Invalid Encoding"),
            Self::InvalidQueryString => write!(f, "Invalid Query String"),
        }
    }
}

impl Error for ParseError {}

impl From<MethodError> for ParseError {
    fn from(err: MethodError) -> Self {
        Self::InvalidMethod(err)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(err: Utf8Error) -> Self {
        Self::InvalidEncoding(err)
    }
}
