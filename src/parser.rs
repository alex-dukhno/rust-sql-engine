use std::fmt::Debug;

use super::lexer::Token::{self, IdentT, LeftParenthesis, RightParenthesis, Comma, Semicolon, NumberT, StringT};
use self::Node::{Delete, From, Where, Id, Const, Insert, Table, Values, Column, TableColumn, Create};
use self::Condition::{Eq};

#[derive(Debug, PartialEq)]
pub enum Node {
    Delete(Box<Node>, Box<Node>),
    From(String),
    Where(Option<Condition>),
    Id(String),
    Const(String),

    Insert(Box<Node>, Box<Node>),
    Table(String, Vec<Node>),
    Values(Vec<Node>),
    Column(String),

    Create(Box<Node>),
    TableColumn(String, Option<Type>, Option<Flag>)
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Int
}

#[derive(Debug, PartialEq)]
pub enum Flag {
    PrimeryKey,
    ForeignKey(String),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Eq(Box<Node>, Box<Node>)
}

pub trait Parser {

    fn parse(self) -> Result<Node, String>;
}

impl Parser for Vec<Token> {

    fn parse(self) -> Result<Node, String> {

        let mut iter = self.into_iter();
        match iter.next() {
            Some(IdentT(statement)) => {
                if statement == "create" {
                    Ok(Create(Box::new(try!(parse_create(&mut iter.by_ref())))))
                }
                else if statement == "delete" {
                    Ok(Delete(Box::new(try!(parse_from(&mut iter.by_ref()))), Box::new(try!(parse_where(&mut iter.by_ref())))))
                }
                else {
                    Ok(Insert(Box::new(try!(parse_table(&mut iter.by_ref()))), Box::new(Values(parse_values(&mut iter.by_ref())))))
                }
            },
            _ => Err("".to_owned()),
        }
    }
}

fn parse_create<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'TABLE' keyword
    let table_name = match tokens.next() {
        Some(IdentT(name)) => name,
        _ => return Err("".to_owned()),
    };
    Ok(Table(table_name, try!(parse_table_columns(&mut tokens.by_ref()))))
}

fn parse_table_columns<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Vec<Node>, String> {
    let mut tokens = tokens.peekable();
    match tokens.peek() {
        Some(&LeftParenthesis) => { tokens.next(); } //skip '('
        _ => return Err("parsing error missing '('".to_owned()),
    }

    let mut columns = vec![];

    loop {
        let col_name = match tokens.next() {
            Some(IdentT(name)) => name,
            _ => return Err("".to_owned()),
        };
        let col_type = match tokens.next() {
            Some(IdentT(_)) => Type::Int,
            _ => return Err("".to_owned()),
        };

        columns.push(TableColumn(col_name, Some(col_type), None));

        match tokens.peek() {
            Some(&Comma) => { tokens.next(); }, //skip ','
            Some(&RightParenthesis) => break,
            Some(&Semicolon) => return Err("parsing error missing ')'".to_owned()),
            _ => return Err("parsing error missing ','".to_owned()),
        }

    }

    tokens.next(); //skip ')'
    match tokens.peek() {
        Some(&Semicolon) => { tokens.next(); } // skip ';'
        _ => return Err("parsing error missing ';'".to_owned()),
    }

    Ok(columns)
}

fn parse_from<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(IdentT(table_name)) => Ok(From(table_name)),
        _ => Err("".to_owned()),
    }
}

fn parse_where<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'WHERE' keyword
    match tokens.next() {
        Some(_) => Ok(Where(Some(Eq(Box::new(Id("col".to_owned())), Box::new(Const("5".to_owned())))))),
        _ => Ok(Where(None)),
    }
}

fn parse_table<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, String> {
    tokens.next(); //skip 'INTO' keyword
    tokens.next(); //skip table name
    Ok(Table("table_name".to_owned(), parse_columns(&mut tokens.by_ref())))
}

fn parse_columns<I: Iterator<Item=Token>>(tokens: &mut I) -> Vec<Node> {
    match tokens.next() {
        Some(LeftParenthesis) => { }, //skip '('
        _ => return vec![],
    }
    println!("columns exist");
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Comma) => {},
            Some(IdentT(col)) => { columns.push(Column(col)); },
            _ => break,
        }
    }
    columns
}

fn parse_values<I: Debug + Iterator<Item=Token>>(tokens: &mut I) -> Vec<Node> {
    // tokens.next(); //skip 'VALUES' keyword
    println!("left tokens - {:?}", tokens);
    tokens.next(); //skip '('
    let mut values = vec![];
    loop {
        match tokens.next() {
            Some(NumberT(val)) | Some(StringT(val)) => values.push(Const(val)),
            Some(Comma) => {},
            _ => break,
        }
    }

    tokens.next(); // skip ';'
    values
}
