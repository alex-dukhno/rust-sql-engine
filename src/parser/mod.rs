pub mod ast;

use std::iter::Peekable;

use super::lexer::Token;
use self::ast::{Node, Type, Condition};

pub trait Parser {
    fn parse(self) -> Node;
}

impl<T: IntoIterator<Item = Token>> Parser for T {
    fn parse(self) -> Node {
        let mut iter = self.into_iter().peekable();
        if let Some(Token::Ident(statement)) = iter.next() {
            match statement.as_str() {
                "create" => Node::Create(Box::new(parse_create(&mut iter.by_ref()))),
                "delete" => Node::Delete(Box::new(parse_from(&mut iter.by_ref())), Box::new(parse_where(&mut iter.by_ref()))),
                "insert" => Node::Insert(Box::new(parse_table(&mut iter.by_ref())), Box::new(Node::Values(parse_values(&mut iter.by_ref())))),
                "select" => parse_select(&mut iter.by_ref()),
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    }
}

fn parse_create<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Node {
    tokens.next(); //skip 'TABLE' keyword
    if let Some(Token::Ident(name)) = tokens.next() {
        Node::Table(name, parse_table_columns(&mut tokens.by_ref()))
    } else {
        unimplemented!()
    }
}

fn parse_table_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Node> {
    match tokens.next() {
        Some(Token::LeftParenthesis) => {} //skip '('
        _ => unimplemented!()
    }

    let mut columns = vec![];

    loop {
        let col_name = match tokens.next() {
            Some(Token::Ident(name)) => name,
            _ => unimplemented!(),
        };
        let col_type = match tokens.next() {
            Some(Token::Ident(_)) => Type::Int,
            _ => unimplemented!(),
        };

        columns.push(Node::TableColumn(col_name, col_type, None));

        match tokens.next() {
            Some(Token::Comma) => {}, //skip ','
            Some(Token::RightParenthesis) => break,
            _ => unimplemented!()
        }
    }

    //    tokens.next(); //skip ')'
    match tokens.peek() {
        Some(&Token::Semicolon) => { tokens.next(); } // skip ';'
        _ => unimplemented!()
    }

    columns
}

fn parse_from<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Node {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(Token::Ident(table_name)) => Node::From(table_name),
        _ => unimplemented!(),
    }
}

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut I) -> Node {
    tokens.next(); //skip 'WHERE' keyword
    match tokens.next() {
        Some(_) => Node::Where(Some(Condition::Eq(Box::new(Node::Id("col".to_owned())), Box::new(Node::Numeric("5".to_owned()))))),
        _ => Node::Where(None),
    }
}

fn parse_table<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Node {
    tokens.next(); //skip 'INTO' keyword
    tokens.next(); //skip table name
    Node::Table("table_name".to_owned(), parse_columns(&mut tokens.by_ref()))
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

fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Node {
    if let Some(&Token::Ident(ref v)) = tokens.peek() {
        if v == "from" {
            unimplemented!()
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
            _ => unimplemented!()
        }
    }

    let table = match tokens.next() {
        Some(Token::Ident(table_name)) => Node::Table(table_name, vec![]),
        _ => unimplemented!()
    };

    Node::Select(Box::new(table), columns)
}
