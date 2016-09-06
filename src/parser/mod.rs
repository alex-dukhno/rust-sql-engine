pub mod ast;

use std::iter::Peekable;

use super::lexer::Token;
use self::ast::{Type, Condition, Statement, CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery, PredicateArgument, ValueParameter};
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

        columns.push(Column::new(col_name, col_type));

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

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut I) -> Option<Condition> {
    if let Some(Token::Ident(_)) = tokens.next() { //skip 'WHERE' keyword
        let left_hand_arg = parse_predicate_arguments(tokens.by_ref());

        if let Some(Token::EqualSign) = tokens.next() {
            let right_hand_arg = parse_predicate_arguments(tokens.by_ref());
            Some(Condition::Eq(left_hand_arg, right_hand_arg))
        } else {
            unimplemented!()
        }
    } else {
        None
    }
}

fn parse_predicate_arguments<I: Iterator<Item = Token>>(tokens: &mut I) -> PredicateArgument {
    match tokens.next() {
        Some(Token::CharactersConstant(s)) => PredicateArgument::StringConstant(s),
        Some(Token::NumericConstant(s)) => PredicateArgument::NumberConstant(s),
        Some(Token::Ident(s)) => PredicateArgument::ColumnName(s),
        _ => unimplemented!(),
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
        Some(&Token::LeftParenthesis) => { tokens.next(); }, //skip '('
        _ => return vec![],
    }
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Token::Comma) => {},
            Some(Token::Ident(col)) => { columns.push(col); },
            Some(Token::RightParenthesis) => break,
            _ => unimplemented!(),
        }
    }
    columns
}

fn parse_values<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<ValueParameter> {
    tokens.next(); //skip 'VALUES' keyword
    tokens.next(); //skip '('
    let mut values = vec![];
    loop {
        match tokens.next() {
            Some(Token::NumericConstant(s)) => values.push(ValueParameter::NumberConst(s)),
            Some(Token::CharactersConstant(s)) => values.push(ValueParameter::StringConst(s)),
            Some(Token::Comma) => {},
            Some(Token::RightParenthesis) => break,
            c => panic!("panic find {:?}", c),
        }
    }
    values
}

//fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> SelectQuery {
//    SelectQuery::new("table_name_1", vec!["col_1"], None)
//}

fn parse_select<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> SelectQuery {
    //    if let Some(&Token::Ident(ref v)) = tokens.peek() {
    //        if v == "from" {
    //            unimplemented!()
    //        }
    //    }

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
