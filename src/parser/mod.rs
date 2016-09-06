pub mod ast;

use std::iter::Peekable;

use super::lexer::Token;
use self::ast::{Type, CondType, Statement, CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery, Condition, CondArg, Value};
use self::ast::table::Column;

pub trait Parser {
    fn parse(self) -> Statement;
}

impl<T: IntoIterator<Item = Token>> Parser for T {
    fn parse(self) -> Statement {
        let mut iter = self.into_iter().peekable();
        if let Some(Token::Ident(statement)) = iter.next() {
            match statement.as_str() {
                "create" => Statement::Create(parse_create(iter.by_ref())),
                "delete" => Statement::Delete(parse_delete(iter.by_ref())),
                "insert" => Statement::Insert(parse_insert(iter.by_ref())),
                "select" => Statement::Select(parse_select(iter.by_ref())),
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    }
}

fn parse_create<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> CreateTableQuery {
    tokens.next(); //skip 'TABLE' keyword
    if let Some(Token::Ident(name)) = tokens.next() {
        CreateTableQuery::new(name, parse_table_columns(tokens.by_ref()))
    } else {
        unimplemented!()
    }
}

fn parse_table_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<Column> {
    match tokens.next() {
        Some(Token::LParent) => {} //skip '('
        _ => unimplemented!()
    }

    let mut columns = vec![];

    loop {
        let col_name = match tokens.next() {
            Some(Token::Ident(name)) => name,
            _ => unimplemented!(),
        };
        let col_type = match tokens.next() {
            Some(Token::Ident(t)) => if t == "int" { Type::Int } else {
                tokens.next(); //skip '('
                let size = match tokens.next() {
                    Some(Token::NumConst(s)) => match s.parse::<u8>() {
                        Ok(s) => s,
                        Err(e) => panic!(e),
                    },
                    _ => unimplemented!(),
                };
                tokens.next(); // skip ')'
                Type::VarChar(size)
            },
            _ => unimplemented!(),
        };

        columns.push(Column::new(col_name, col_type));

        match tokens.next() {
            Some(Token::Comma) => {}, //skip ','
            Some(Token::RParent) => break,
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

fn parse_delete<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> DeleteQuery {
    DeleteQuery::new(parse_from(tokens.by_ref()), parse_where(tokens.by_ref()))
}

fn parse_from<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> String {
    tokens.next(); //skip 'FROM' keyword
    match tokens.next() {
        Some(Token::Ident(table_name)) => table_name,
        _ => unimplemented!(),
    }
}

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Option<Condition> {
    if let Some(Token::Ident(_)) = tokens.next() { //skip 'WHERE' keyword
        let left = parse_predicate_arguments(tokens.by_ref());

        let cond_type = match tokens.next() {
            Some(Token::EqualSign) => CondType::Eq,
            Some(Token::LAngle) => {
                match tokens.peek() {
                    Some(&Token::RAngle) => {
                        tokens.next();
                        CondType::NotEq
                    },
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        };
        let right = parse_predicate_arguments(tokens.by_ref());
        Some(Condition::new(left, right, cond_type))
    } else {
        None
    }
}

fn parse_predicate_arguments<I: Iterator<Item = Token>>(tokens: &mut I) -> CondArg {
    match tokens.next() {
        Some(Token::CharsConst(s)) => CondArg::StringConstant(s),
        Some(Token::NumConst(s)) => CondArg::NumConst(s),
        Some(Token::Ident(s)) => if s == "limit" {
            CondArg::Limit
        }
        else {
            CondArg::ColumnName(s)
        },
        c => panic!("unexpected token - {:?}", c),
    }
}

fn parse_insert<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> InsertQuery {
    tokens.next(); //skip 'INTO' keyword
    if let Some(Token::Ident(table_name)) = tokens.next() {
        InsertQuery::new(table_name, parse_columns(tokens.by_ref()), parse_values(tokens.by_ref()))
    } else {
        unimplemented!()
    }
}

fn parse_columns<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Vec<String> {
    match tokens.peek() {
        Some(&Token::LParent) => { tokens.next(); }, //skip '('
        _ => return vec![],
    }
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Token::Comma) => {},
            Some(Token::Ident(col)) => { columns.push(col); },
            Some(Token::RParent) => break,
            _ => unimplemented!(),
        }
    }
    columns
}

fn parse_values<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<Value> {
    tokens.next(); //skip 'VALUES' keyword
    tokens.next(); //skip '('
    let mut values = vec![];
    loop {
        match tokens.next() {
            Some(Token::NumConst(s)) => values.push(Value::NumConst(s)),
            Some(Token::CharsConst(s)) => values.push(Value::StrConst(s)),
            Some(Token::Comma) => {},
            Some(Token::RParent) => break,
            c => panic!("panic find {:?}", c),
        }
    }
    values
}

fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> SelectQuery {
    let mut columns = vec![];

    loop {
        match tokens.next() {
            Some(Token::Ident(v)) => if v == "from" {
                break; // skip 'FROM' keyword
            } else {
                columns.push(v)
            },
            Some(Token::Comma) => {},
            _ => unimplemented!()
        }
    }

    let table_name = match tokens.next() {
        Some(Token::Ident(table_name)) => table_name,
        _ => unimplemented!()
    };

    SelectQuery::new(table_name, columns, parse_where(tokens.by_ref()))
}
