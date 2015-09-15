use std::string::String;
use std::result::Result;
use std::iter::Iterator;

use super::lexer::Lexer;

pub fn parse_query(lexer: &Lexer) {
    let first_node = (*lexer).next();
    let first_match = first_node.matches("insert")
}
