use std::string::String;
use std::result::Result;
use std::marker::Sized;
use std::error::Error;

pub struct SQLQueryParser {
    name: String,
}

impl SQLQueryParser {

    pub fn new(name: String) -> SQLQueryParser {
        SQLQueryParser { name: name }
    }

    // pub fn parse_query(query: String) -> Result<SQLNode, SQLParseError> {
        // Result::Ok(SQLNode::new(query))
    // }
}

pub struct SQLNode {
    value: String,
}

impl SQLNode {

    pub fn new(value: String) -> SQLNode {
        SQLNode { value: value }
    }
}

// #[derive(Debug)]
// pub struct SQLParseError {
    // msg: String,
// }

// impl Error for SQLParseError {

// }
