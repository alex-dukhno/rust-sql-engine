use std::string::String;

use std::option::Option;

use std::marker::Sized;

use std::error::Error;

use std::fmt;
use std::result;

pub type SQLResult = result::Result<SQLNode, SQLParseError>;

pub struct SQLQueryParser {
    name: String,
}

impl SQLQueryParser {

    pub fn new(name: String) -> SQLQueryParser {
        SQLQueryParser { name: name }
    }

    pub fn parse_query(&self, query: String) -> SQLResult {
        Result::Ok(SQLNode::new(query))
    }
}

pub struct SQLNode {
    value: String,
}

impl SQLNode {

    pub fn new(value: String) -> SQLNode {
        SQLNode { value: value }
    }
}

#[derive(Debug)]
pub struct SQLParseError {
    query: String,
    msg: String,
    code: i32,
}

impl fmt::Display for SQLParseError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error code {}: {} during parsing query {}", self.code, self.msg, self.query)
    }
}

impl Error for SQLParseError {

    fn description(&self) -> &str {
        &(self.msg)
    }

    fn cause(&self) -> Option<&Error> {
        Option::None
    }
}
