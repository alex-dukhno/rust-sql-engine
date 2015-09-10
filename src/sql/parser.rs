use std::string::String;
use std::result::Result;

use std::boxed::Box;

use super::error::SQLError;

use super::ast::SQLAbstractTreeWalker;
use super::ast::SQLNode;

pub type SQLResult = Result<SQLAbstractTreeWalker, SQLError>;

pub fn parse_query(query: String) -> SQLResult {
    Result::Ok(SQLAbstractTreeWalker::new(query))
}
