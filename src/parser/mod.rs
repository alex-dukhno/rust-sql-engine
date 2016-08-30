pub mod ast;

use std::iter::Peekable;
use std::marker::Sized;
use std::fmt;

use super::lexer::Token;
use self::ast::{Node, Type, Condition};

pub trait Parser {
    fn parse(self) -> Result<Node, String>;
}

impl<T: IntoIterator<Item = Token>> Parser for T {
    fn parse(self) -> Result<Node, String> {
        let mut iter = self.into_iter().peekable();
        match iter.next() {
            Some(Token::Ident(statement)) => {
                match statement.as_str() {
                    "create" => Ok(Node::Create(Box::new(try!(parse_create(&mut iter.by_ref()))))),
                    "delete" => Ok(Node::Delete(Box::new(try!(parse_from(&mut iter.by_ref()))), Box::new(try!(parse_where(&mut iter.by_ref()))))),
                    "insert" => Ok(Node::Insert(Box::new(try!(parse_table(&mut iter.by_ref()))), Box::new(Node::Values(parse_values(&mut iter.by_ref()))))),
                    "select" => Ok(try!(parse_select(&mut iter.by_ref()))),
                    _ => Err("undefined query type".to_owned()),
                }
            }
            _ => Err("".to_owned()),
        }
    }
}

fn parse_create<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'TABLE' keyword
    let table_name = match tokens.next() {
        Some(Token::Ident(name)) => name,
        Some(token) => return Err(format_unexpected_token("table name", Some(&token))),
        _ => return Err("".to_owned()),
    };
    Ok(Node::Table(table_name, try!(parse_table_columns(&mut tokens.by_ref()))))
}

fn parse_table_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Vec<Node>, String> {
    match tokens.next() {
        Some(Token::LeftParenthesis) => {} //skip '('
        token => return Err(format_unexpected_token(Token::LeftParenthesis, token.as_ref())),
    }

    let mut columns = vec![];

    loop {
        let col_name = match tokens.next() {
            Some(Token::Ident(name)) => name,
            _ => return Err("".to_owned()),
        };
        let col_type = match tokens.next() {
            Some(Token::Ident(_)) => Type::Int,
            _ => return Err("".to_owned()),
        };

        columns.push(Node::TableColumn(col_name, col_type, None));

        match tokens.next() {
            Some(Token::Comma) => {}, //skip ','
            Some(Token::RightParenthesis) => break,
            Some(Token::Ident(id)) => return Err(format_unexpected_token(Token::Comma, Some(&Token::Ident(id)))),
            Some(token) => return Err(format_unexpected_token(Token::RightParenthesis, Some(&token))),
            None => return Err("parsing error missing ','".to_owned()),
        }
    }

    //    tokens.next(); //skip ')'
    match tokens.peek() {
        Some(&Token::Semicolon) => { tokens.next(); } // skip ';'
        _ => return Err(format_unexpected_token(Token::Semicolon, None)),
    }

    Ok(columns)
}

fn format_unexpected_token<D: fmt::Display + Sized>(expected: D, found: Option<&Token>) -> String {
    match found {
        Some(token) => format!("error: expected <{}> found <{}>", expected, token),
        None => format!("error: expected <{}>", expected)
    }
}

fn parse_from<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(Token::Ident(table_name)) => Ok(Node::From(table_name)),
        _ => Err("".to_owned()),
    }
}

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'WHERE' keyword
    match tokens.next() {
        Some(_) => Ok(Node::Where(Some(Condition::Eq(Box::new(Node::Id("col".to_owned())), Box::new(Node::Numeric("5".to_owned())))))),
        _ => Ok(Node::Where(None)),
    }
}

fn parse_table<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    tokens.next(); //skip 'INTO' keyword
    tokens.next(); //skip table name
    Ok(Node::Table("table_name".to_owned(), parse_columns(&mut tokens.by_ref())))
}

fn parse_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Node> {
    match tokens.peek() {
        Some(&Token::LeftParenthesis) => { tokens.next(); }, //skip '('
        _ => return vec![],
    }
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Token::Comma) => {},
            Some(Token::Ident(col)) => { columns.push(Node::Column(col)); },
            _ => break,
        }
    }
    columns
}

fn parse_values<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Node> {
    tokens.next(); //skip 'VALUES' keyword
    tokens.next(); //skip '('
    let mut values = vec![];
    loop {
        match tokens.next() {
            Some(Token::NumericConstant(val)) => values.push(Node::Numeric(val)),
            Some(Token::CharactersConstant(val)) => values.push(Node::CharSequence(val)),
            Some(Token::Comma) => {},
            _ => break,
        }
    }

    tokens.next(); // skip ';'
    values
}

fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Node, String> {
    if let Some(&Token::Ident(ref v)) = tokens.peek() {
        if v == "from" {
            return Err("parsing error".to_owned());
        }
    }

    let mut columns = vec![];

    loop {
        match tokens.next() {
            Some(Token::Ident(v)) => if v == "from" {
                break; // skip 'FROM' keyword
            } else {
                columns.push(Node::Column(v))
            },
            Some(Token::Comma) => {},
            _ => return Err("parsing error".to_owned()),
        }
    }

    let table = match tokens.next() {
        Some(Token::Ident(table_name)) => Node::Table(table_name, vec![]),
        _ => return Err("parsing error".to_owned()),
    };

    Ok(Node::Select(Box::new(table), columns))
}
