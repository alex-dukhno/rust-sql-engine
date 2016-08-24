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
    Table(String, Option<Vec<Node>>),
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

    fn parse(self) -> Result<Node, ()>;
}

impl Parser for Vec<Token> {

    fn parse(self) -> Result<Node, ()> {

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
            _ => Err(()),
        }
    }
}

fn parse_create<I: DoubleEndedIterator<Item=Token>>(tokens: &mut I) -> Result<Node, ()> {
    tokens.next(); //skip 'TABLE' keyword
    let mut reverse = tokens.rev();
    reverse.next(); // skip ';'
    reverse.next(); //skip ')'
    let col_type = match reverse.next() {
        Some(IdentT(_)) => Type::Int,
        _ => return Err(()),
    };
    let col_name = match reverse.next() {
        Some(IdentT(name)) => name,
        _ => return Err(()),
    };
    reverse.next(); //skip '('
    let table = match reverse.next() {
        Some(IdentT(table_name)) => Table(table_name, Some(vec![TableColumn(col_name, Some(col_type), None)])),
        _ => return Err(()),
    };
    Ok(table)
}

fn parse_from<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, ()> {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(IdentT(table_name)) => Ok(From(table_name)),
        _ => Err(()),
    }
}

fn parse_where<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, ()> {
    tokens.next(); //skip 'WHERE' keyword
    match tokens.next() {
        Some(_) => Ok(Where(Some(Eq(Box::new(Id("col".to_owned())), Box::new(Const("5".to_owned())))))),
        _ => Ok(Where(None)),
    }
}

fn parse_table<I: Iterator<Item=Token>>(tokens: &mut I) -> Result<Node, ()> {
    tokens.next(); //skip table name
    Ok(Table("table_name".to_owned(), parse_columns(&mut tokens.by_ref())))
}

fn parse_columns<I: Iterator<Item=Token>>(tokens: &mut I) -> Option<Vec<Node>> {
    let mut peekable = tokens.peekable();
    match peekable.peek() {
        Some(&LeftParenthesis) => { peekable.next(); },
        _ => return None,
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
    Some(columns)
}
