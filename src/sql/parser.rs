use std::string::String;

use std::option::Option;

use std::marker::Sized;

use super::error::SQLError;

use std::fmt;
use std::result;

pub type SQLResult = result::Result<SQLNode, SQLError>;

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
