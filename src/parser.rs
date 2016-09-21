use std::iter;
use std::collections;
use std::error::Error;

use super::lexer::{Token, Tokens};
use super::ast::{Type, CondType, Statement, CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery, Condition, CondArg, Value, ColumnTable, ValueSource, Constraint};

pub fn parse(tokens: Tokens) -> Result<Statement, String> {
    let mut iter = tokens.into_iter();
    match iter.next() {
        Some(Token::Create) => Ok(Statement::Create(try!(parse_create_table(iter.by_ref())))),
        Some(Token::Delete) => Ok(Statement::Delete(parse_delete_query(iter.by_ref()))),
        Some(Token::Insert) => Ok(Statement::Insert(parse_insert_query(iter.by_ref()))),
        Some(Token::Select) => Ok(Statement::Select(parse_select_query(iter.by_ref()))),
        _ => unimplemented!(),
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

    while let Some(token) = tokens.next() {
        match token {
            Token::LParent => {},
            Token::Semicolon => break,
            Token::Ident(name) => columns.push(try!(parse_table_column(tokens.by_ref(), name))),
            t => panic!("unexpected token {:?}", t)
        }
    }

    Ok(CreateTableQuery::new(table_name, columns))
}

fn parse_table_column<I: Iterator<Item = Token>>(tokens: &mut I, column_name: String) -> Result<ColumnTable, String> {
    let column_type = match tokens.next() {
        Some(Token::Int) => Type::Integer,
        Some(Token::Character) => try!(parse_var_char_type(tokens.by_ref())),
        _ => unimplemented!(),
    };
    let mut column_constraints = collections::HashSet::new();
    let mut has_default = false;
    let mut is_primary_key = false;
    let mut is_foreign_key = false;
    while let Some(token) = tokens.next() {
        match token {
            Token::Primary => {
                if Some(Token::Key) != tokens.next() {
                    unimplemented!()
                }
                is_primary_key = true;
                column_constraints.insert(Constraint::PrimaryKey);
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
                                is_foreign_key = true;
                                column_constraints.insert(Constraint::ForeignKey(table_name, col_name));
                            }
                            t => panic!("unexpected token {:?}", t)
                        }
                        if Some(Token::RParent) != tokens.next() {
                            unimplemented!();
                        }
                    }
                    t => panic!("unexpected token {:?}", t)
                }
            }
            Token::Default => {
                match tokens.next() {
                    Some(Token::NumConst(const_val))
                    | Some(Token::CharsConst(const_val)) => {
                        if !is_primary_key {
                            has_default = true;
                            column_constraints.insert(Constraint::DefaultValue(Some(const_val)));
                        }
                    },
                    t => panic!("unexpected token {:?}", t)
                }
            },
            Token::Not => {
                match tokens.next() {
                    Some(Token::Null) => {
                        column_constraints.insert(Constraint::Nullable(false));
                        match column_type {
                            Type::Integer => {
                                column_constraints.insert(Constraint::DefaultValue(Some("0".to_owned())));
                            },
                            Type::VarChar(len) => {
                                column_constraints.insert(Constraint::DefaultValue(Some(iter::repeat(" ").take(len as usize).collect::<String>())));
                            }
                        }
                    },
                    t => panic!("unexpected token {:?}", t)
                }
            }
            Token::RParent | Token::Comma => {
                if !is_primary_key && is_foreign_key && !column_constraints.contains(&Constraint::Nullable(false)) {
                    column_constraints.insert(Constraint::Nullable(true));
                } else {
                    column_constraints.insert(Constraint::Nullable(false));
                }
                if !has_default {
                    column_constraints.insert(Constraint::DefaultValue(None));
                }
                break;
            },
            t => panic!("unexpected token {:?}", t)
        }
    };
    Ok(ColumnTable::new(column_name, column_type, column_constraints))
}

fn parse_var_char_type<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Type, String> {
    match tokens.next() {
        Some(Token::LParent) => {},
        Some(token) => return Err(format!("expected token <{}> but was found <{}>", Token::LParent, token)),
        None => unimplemented!()
    }

    let size = try!(parse_size(tokens.by_ref()));

    if tokens.next() != Some(Token::RParent) {
        unimplemented!();
    }

    Ok(Type::VarChar(size))
}

fn parse_size<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<u8, String> {
    match tokens.next() {
        Some(Token::NumConst(num)) => match num.parse::<u8>() {
            Ok(size) => Ok(size),
            Err(e) => Err(e.description().into())
        },
        _ => unimplemented!(),
    }
}

fn parse_columns<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<String> {
    let mut columns = vec![];
    while let Some(token) = tokens.next() {
        match token {
            Token::Comma => {},
            Token::Ident(col) => { columns.push(col); },
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
            Token::NumConst(s) => values.push(Value::NumConst(s)),
            Token::CharsConst(s) => values.push(Value::StrConst(s)),
            Token::Comma => {},
            Token::RParent => break,
            unexpected => panic!("panic find unexpected token {:?}", unexpected),
        }
    }
    values
}

fn parse_insert_query<I: Iterator<Item = Token>>(tokens: &mut I) -> InsertQuery {
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
        InsertQuery::new(table_name, columns, ValueSource::SubQuery(parse_select_query(tokens.by_ref())))
    } else {
        let query = InsertQuery::new(table_name, columns, ValueSource::Row(parse_values(tokens.by_ref())));
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

fn parse_select_query<I: Iterator<Item = Token>>(tokens: &mut I) -> SelectQuery {
    let columns = parse_columns_list(tokens.by_ref());

    let table_name = match tokens.next() {
        Some(Token::Ident(table_name)) => table_name,
        _ => unimplemented!()
    };

    SelectQuery::new(table_name, columns, parse_where(tokens.by_ref()))
}

fn parse_columns_list<I: Iterator<Item = Token>>(tokens: &mut I) -> Vec<String> {
    let mut columns = vec![];
    loop {
        match tokens.next() {
            Some(Token::From) => break, // skip 'FROM' keyword
            Some(Token::Ident(column_name)) => columns.push(column_name),
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
