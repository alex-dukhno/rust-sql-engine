use std::error::Error;

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct SQLError {
    query: String,
    msg: String,
    code: i32,
}

impl Display for SQLError {

    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Error code {}: {} during parsing query {}", self.code, self.msg, self.query)
    }
}

impl Error for SQLError {

    fn description(&self) -> &str {
        &(self.msg)
    }

    fn cause(&self) -> Option<&Error> {
        Option::None
    }
}
