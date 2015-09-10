use std::string::String;
use std::result::Result;
use std::iter::Iterator;

use super::error::SQLError;

const WHITE_SPACES = vec![' ', '\n', '\r', '\t', '\v', '\f'];

pub type InsertParserResult = Result<ASTInsertStatementNode, SQLError>;

pub fn parse_insert_query(query: String) -> InsertParserResult {
    let mut chars = query.chars();
    skip_white_spaces(chars);
    let insert_keyword = match (
            chars.next(), chars.next(), chars.next(), chars.next(), chars.next(), chars.next()) {
        val @ ('i'|'I', 'n'|'N', 's'|'S', 'e'|'E', 'r'|'R', 't'|'T') => {val},
        _ => panic!("Keyword 'INSERT' is not found"),
    }
    {
        let before = chars.count();
        skip_white_spaces(chars);
        let after = chars.count();
        if after == before {
            panic!("Error in query string at {} position", after);
        }
    }
    let into_keyword = match(
            chars.next(), chars.next(), chars.next(), chars.next()) {
        val @ ('i'|'I', 'n'|'N', 't'|'T', 'o'|O) => {val},
        _ => panic!("Keyword 'INTO' is not found"),
    }
    {
        let before = chars.count();
        skip_white_spaces(chars);
        let after = chars.count();
        if after == before {
            panic!("Error in query string at {} position", after);
        }
    }
}

fn skip_white_spaces(chars: &mut Iterator) {
    chars.skip_while(
        |&chars| {
            WHITE_SPACES.contains(*chars)
        }
    );
}
