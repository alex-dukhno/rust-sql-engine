use super::lexer::Token::{self, IdentT, LeftParenthesis, RightParenthesis, Comma};
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
                    iter.next(); //skip 'INTO' keyword
                    Ok(Insert(Box::new(try!(parse_table(&mut iter.by_ref()))), Box::new(Values(vec![Const("10".to_owned()), Const("string".to_owned())]))))
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
    tokens.next(); //skip '('

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
            _ => break,
        }

    }

    match tokens.next() {
         Some(RightParenthesis) => {}, //skip ')'
         _ => return Err("parsing error missing ','".to_owned()),
    }
    tokens.next(); // skip ';'

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
    tokens.next(); //skip table name
    Ok(Table("table_name".to_owned(), parse_columns(&mut tokens.by_ref())))
}

fn parse_columns<I: Iterator<Item=Token>>(tokens: &mut I) -> Vec<Node> {
    let mut peekable = tokens.peekable();
    match peekable.peek() {
        Some(&LeftParenthesis) => { peekable.next(); },
        _ => return vec![],
    }
    let mut columns = vec![];
    loop {
        match peekable.peek() {
            Some(&RightParenthesis) | None => break,
            Some(&Comma) => { peekable.next(); },
            Some(_) => {
                if let Some(IdentT(col)) = peekable.next() {
                    columns.push(Column(col));
                }
            },
        }
    }
    columns
}
