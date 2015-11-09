extern crate sql;

use sql::lexer::Lexer;
use sql::lexer::Token;

static DELETE: &'static str = "delete";
const FROM: &'static str = "from";

#[test]
fn test_parse_empty_string() {
    let mut lexer = Lexer::new("");

    assert_eq!(lexer.next(), None);
}

#[test]
fn test_parse_newline() {
    let mut lexer = Lexer::new("\n");

    assert_eq!(lexer.next(), Some(Token::NewLine));
}

#[test]
fn test_parse_spaces() {
    let mut lexer = Lexer::new(" ");

    assert_eq!(lexer.next(), Some(Token::WhiteSpace));
}

#[test]
fn test_parse_tabulation() {
    let mut lexer = Lexer::new("\t");

    assert_eq!(lexer.next(), Some(Token::WhiteSpace));
}

#[test]
fn test_parse_many_spaces() {
    let mut lexer = Lexer::new(" \t \t ");

    assert_eq!(lexer.next(), Some(Token::WhiteSpace));
    assert_eq!(lexer.is_empty(), true);
}

#[test]
fn test_simple_delete_query() {
    let mut lexer = Lexer::new("delete from tab1");

    assert_eq!(lexer.next(), Some(Token::KeyWord(DELETE.to_string())));
    assert_eq!(lexer.next(), Some(Token::WhiteSpace));
    assert_eq!(lexer.next(), Some(Token::KeyWord(FROM.to_string())));
    assert_eq!(lexer.next(), Some(Token::WhiteSpace));
    assert_eq!(lexer.next(), Some(Token::Table("tab1".to_string())));
}
