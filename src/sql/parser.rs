use std::string::String;
use std::result::Result;
use std::iter::Iterator;

use super::lexer::Lexer;
// use super::error::SQLError;

pub struct Parser<'a> {
    lexer: &'a Lexer<'a>,
}

impl<'a> Parser<'a> {

    pub fn new(lexer: &'a Lexer) -> Parser<'a> {
        Parser { lexer: lexer }
    }
}
