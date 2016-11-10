use std::iter::{self, Peekable};
use std::error::Error;

use super::lexer::{Token, Tokens};
use super::ast::{Type, CondType, RawStatement, RawColumn, Condition, CondArg};
use super::ast::create_table::{CreateTableQuery, ColumnTable};
use super::ast::delete_query::DeleteQuery;
use super::ast::insert_query::{Value, ValueSource, InsertQuery};
use super::ast::select_query::SelectQuery;

pub fn parse(tokens: Tokens) -> Result<RawStatement, String> {
    let mut iter = tokens.into_iter();
    match iter.next() {
        Some(Token::Create) => Ok(RawStatement::Create(try!(parse_create_table(iter.by_ref())))),
        Some(Token::Delete) => Ok(RawStatement::Delete(parse_delete_query(iter.by_ref()))),
        Some(Token::Insert) => Ok(RawStatement::Insert(parse_insert_query(iter.by_ref()))),
        Some(Token::Select) => Ok(RawStatement::Select(parse_select_query(iter.by_ref()))),
        token => panic!("unimplemented parsing procedure for {:?}", token),
    }
}

fn parse_create_table<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<CreateTableQuery, String> {
    if tokens.next() != Some(Token::Table) {
        unimplemented!();
    }

    let table_name = match tokens.next() {
        Some(Token::Ident(name)) => name,
        _ => unimplemented!(),
    };

    let mut columns = vec![];

    let mut has_semicolon = false;
    while let Some(token) = tokens.next() {
        match token {
            Token::LParent => {},
            Token::Semicolon => {
                has_semicolon = true;
                break
            },
            Token::Ident(name) => columns.push(try!(parse_table_column(tokens.by_ref(), name))),
            token => panic!("unexpected token {:?}", token)
        }
    }
    if !has_semicolon {
        Err("missed ';' in the end of statement".into())
    } else {
        Ok(CreateTableQuery::new(table_name, columns))
    }
}

fn parse_table_column<I: Iterator<Item = Token>>(tokens: &mut I, column_name: String) -> Result<ColumnTable, String> {
    let mut tokens = tokens.peekable();
    let column_type = match tokens.next() {
        Some(Token::Int) => Type::Integer,
        Some(Token::Character) => try!(parse_char_type(tokens.by_ref())),
        token => panic!("Unexpected token - {:?}", token),
    };
    let mut is_primary_key = false;
    let mut foreign_key = None;
    let mut is_nullable = true;
    let mut default_value = None;
    while let Some(token) = tokens.next() {
        match token {
            Token::Primary => {
                if Some(Token::Key) != tokens.next() {
                    unimplemented!()
                }
                is_primary_key = true;
            },
            Token::Foreign => {
                if Some(Token::Key) != tokens.next() {
                    unimplemented!()
                }
                if Some(Token::References) != tokens.next() {
                    unimplemented!();
                }
                match tokens.next() {
                    Some(Token::Ident(table_name)) => {
                        if Some(Token::LParent) != tokens.next() {
                            unimplemented!();
                        }
                        match tokens.next() {
                            Some(Token::Ident(col_name)) => {
                                foreign_key = Some((table_name, col_name));
                            }
                            token => panic!("unexpected token {:?}", token)
                        }
                        if Some(Token::RParent) != tokens.next() {
                            unimplemented!();
                        }
                    }
                    token => panic!("unexpected token {:?}", token)
                }
            },
            Token::Default => {
                match tokens.next() {
                    Some(Token::NumConst(const_val)) |
                    Some(Token::CharsConst(const_val)) => { default_value = Option::from(const_val) },
                    token => panic!("unexpected token {:?}", token)
                }
            },
            Token::Not => {
                match tokens.next() {
                    Some(Token::Null) => {
                        is_nullable = false;
                        match column_type {
                            Type::Integer => {
                                default_value = Option::from(String::from("0"));
                            },
                            Type::Character(Some(len)) => {
                                default_value = Option::from(iter::repeat(" ").take(len as usize).collect::<String>());
                            },
                            Type::Character(None) => {}
                        }
                    },
                    t => panic!("unexpected token {:?}", t)
                }
            },
            Token::RParent | Token::Comma | Token::Semicolon => {
                if is_primary_key {
                    is_nullable = false;
                }
                break;
            },
            t => panic!("unexpected token {:?}", t)
        }
    };
    Ok(ColumnTable::new(column_name, column_type, is_primary_key, foreign_key, is_nullable, default_value))
}

fn parse_char_type<I: Iterator<Item = Token>>(tokens: &mut Peekable<I>) -> Result<Type, String> {
    match tokens.peek() {
        Some(&Token::LParent) => {}
        Some(&Token::RParent) | Some(&Token::Comma) => return Ok(Type::Character(None)),
        token => panic!("unexpected token {:?}", token)
    }

    tokens.next();
    let size = try!(parse_size(tokens.by_ref()));

    if tokens.next() != Some(Token::RParent) {
        unimplemented!();
    }

    Ok(Type::Character(Option::from(size)))
}

fn parse_size<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<u8, String> {
    match tokens.next() {
        Some(Token::NumConst(num)) => match num.parse::<u8>() {
            Ok(size) => Ok(size),
            Err(e) => Err(e.description().into())
        },
        Some(Token::Minus) => Err("invalid digit found in string".into()),
        token => panic!("unimplemented parsing procedure for {:?} token", token),
    }
}

fn parse_columns<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<RawColumn> {
    let mut columns = vec![];
    while let Some(token) = tokens.next() {
        match token {
            Token::Comma => {},
            Token::Ident(col) => { columns.push(RawColumn::new(col)); },
            Token::RParent => break,
            unexpected => panic!("panic find unexpected token {:?}", unexpected),
        }
    }
    columns
}

fn parse_values<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<Value> {
    if tokens.next() != Some(Token::LParent) {
        unimplemented!();
    }
    let mut values = vec![];
    while let Some(token) = tokens.next() {
        match token {
            Token::NumConst(s) => values.push(Value::new(s, Type::Integer)),
            Token::CharsConst(s) => {
                let size = s.len() as u8;
                values.push(Value::new(s, Type::Character(Option::from(size))));
            },
            Token::Comma => {},
            Token::RParent => break,
            unexpected => panic!("panic find unexpected token {:?}", unexpected),
        }
    }
    values
}

fn parse_insert_query<I: Iterator<Item = Token>>(tokens: &mut I) -> InsertQuery<RawColumn> {
    if tokens.next() != Some(Token::Into) {
        unimplemented!();
    }
    let table_name = match tokens.next() {
        Some(Token::Ident(table_name)) => table_name,
        _ => unimplemented!(),
    };
    let mut columns = vec![];

    let mut sub_query = false;
    while let Some(token) = tokens.next() {
        match token {
            Token::LParent => columns = parse_columns(tokens.by_ref()),
            Token::Values => {
                sub_query = false;
                break;
            },
            Token::Select => {
                sub_query = true;
                break;
            },
            Token::Semicolon => break,
            _ => unimplemented!(),
        }
    }

    if sub_query {
        InsertQuery::new(table_name, columns.into_iter().collect(), ValueSource::SubQuery(parse_select_query(tokens.by_ref())))
    } else {
        let query = InsertQuery::new(table_name, columns.into_iter().collect(), ValueSource::Row(parse_values(tokens.by_ref())));
        if tokens.next() != Some(Token::Semicolon) {
            unimplemented!();
        }
        query
    }
}

fn parse_delete_query<I: Iterator<Item = Token>>(tokens: &mut I) -> DeleteQuery {
    if tokens.next() != Some(Token::From) {
        unimplemented!();
    }

    let table_name = match tokens.next() {
        Some(Token::Ident(name)) => name,
        _ => unimplemented!(),
    };

    DeleteQuery::new(table_name, parse_where(tokens.by_ref()))
}

fn parse_select_query<I: Iterator<Item = Token>>(tokens: &mut I) -> SelectQuery<RawColumn> {
    let columns = parse_columns_list(tokens.by_ref());

    let table_name = match tokens.next() {
        Some(Token::Ident(table_name)) => table_name,
        _ => unimplemented!()
    };

    SelectQuery::new(table_name, columns, parse_where(tokens.by_ref()))
}

fn parse_columns_list<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<RawColumn> {
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Token::From) => break, // skip 'FROM' keyword
            Some(Token::Ident(column_name)) => columns.push(RawColumn::new(column_name)),
            Some(Token::Comma) => {},
            t => panic!("unexpected token {:?} - ", t),
        }
    }
    columns
}

fn parse_where<I: Iterator<Item = Token>>(tokens: &mut I) -> Option<Condition> {
    match tokens.next() {
        Some(Token::Where) => {
            let left = parse_predicate_arguments(tokens.by_ref());

            let cond_type = match tokens.next() {
                Some(Token::EqualSign) => CondType::Eq,
                Some(Token::NotEqualSign) => CondType::NotEq,
                _ => unimplemented!(),
            };
            let right = parse_predicate_arguments(tokens.by_ref());
            Some(Condition::new(left, right, cond_type))
        },
        Some(Token::Semicolon) => None,
        _ => unimplemented!(),
    }
}

fn parse_predicate_arguments<I: Iterator<Item = Token>>(tokens: &mut I) -> CondArg {
    match tokens.next() {
        Some(Token::CharsConst(s)) => CondArg::StringConstant(s),
        Some(Token::NumConst(s)) => CondArg::NumConst(s),
        Some(Token::Limit) => CondArg::Limit,
        Some(Token::Ident(s)) => CondArg::ColumnName(s),
        c => panic!("unexpected token - {:?}", c),
    }
}
