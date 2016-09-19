use std::iter;
use std::collections;

use super::lexer::{Token, Tokens};
use super::ast::{Type, CondType, Statement, CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery, Condition, CondArg, Value, ColumnTable, ValueSource, Constraint};

pub fn parse(tokens: Tokens) -> Result<Statement, String> {
    let mut iter = tokens.into_iter();
    match iter.next() {
        Some(Token::Create) => Ok(CreateTableQueryParser::new(iter).parse()),
        Some(Token::Delete) => Ok(DeleteQueryParser::new(iter).parse()),
        Some(Token::Insert) => Ok(InsertQueryParser::new(iter).parse()),
        Some(Token::Select) => Ok(SelectQueryParser::new(iter).parse()),
        _ => unimplemented!(),
    }
}

#[derive(Debug)]
struct CreateTableQueryParser<I: Iterator<Item = Token>> {
    tokens: I
}

impl<I: Iterator<Item = Token>> CreateTableQueryParser<I> {
    pub fn parse(mut self) -> Statement {
        if self.tokens.next() != Some(Token::Table) {
            unimplemented!();
        }

        let table_name = match self.tokens.next() {
            Some(Token::Ident(name)) => name,
            _ => unimplemented!(),
        };

        let mut columns = vec![];

        while let Some(token) = self.tokens.next() {
            match token {
                Token::LParent => {},
                Token::Semicolon => break,
                Token::Ident(name) => columns.push(self.parse_column(name)),
                t => panic!("unexpected token {:?}", t)
            }
        }

        Statement::Create(CreateTableQuery::new(table_name, columns))
    }

    fn new(tokens: I) -> CreateTableQueryParser<I> {
        CreateTableQueryParser {
            tokens: tokens
        }
    }

    fn parse_column(&mut self, column_name: String) -> ColumnTable {
        let column_type = match self.tokens.next() {
            Some(Token::Int) => Type::Integer,
            Some(Token::Character) => self.parse_var_char_type(),
            _ => unimplemented!(),
        };
        let mut column_constraints = collections::HashSet::new();
        let mut has_default = false;
        let mut is_primary_key = false;
        let mut is_foreign_key = false;
        while let Some(token) = self.tokens.next() {
            match token {
                Token::PrimaryKey => {
                    is_primary_key = true;
                    column_constraints.insert(Constraint::PrimaryKey);
                },
                Token::ForeignKey => {
                    if Some(Token::References) != self.tokens.next() {
                        unimplemented!();
                    }
                    match self.tokens.next() {
                        Some(Token::Ident(table_name)) => {
                            if Some(Token::LParent) != self.tokens.next() {
                                unimplemented!();
                            }
                            match self.tokens.next() {
                                Some(Token::Ident(col_name)) => {
                                    is_foreign_key = true;
                                    column_constraints.insert(Constraint::ForeignKey(table_name, col_name));
                                }
                                t => panic!("unexpected token {:?}", t)
                            }
                            if Some(Token::RParent) != self.tokens.next() {
                                unimplemented!();
                            }
                        }
                        t => panic!("unexpected token {:?}", t)
                    }
                }
                Token::Default => {
                    match self.tokens.next() {
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
                    match self.tokens.next() {
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
                    if !is_primary_key && !column_constraints.contains(&Constraint::Nullable(false)) {
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
        ColumnTable::new(column_name, column_type, column_constraints)
    }

    fn parse_var_char_type(&mut self) -> Type {
        if self.tokens.next() != Some(Token::LParent) {
            unimplemented!();
        }

        let size = self.parse_size();

        if self.tokens.next() != Some(Token::RParent) {
            unimplemented!();
        }

        Type::VarChar(size)
    }

    fn parse_size(&mut self) -> u8 {
        match self.tokens.next() {
            Some(Token::NumConst(num)) => match num.parse::<u8>() {
                Ok(size) => size,
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

struct InsertQueryParser<I: Iterator<Item = Token>> {
    tokens: I
}

impl<I: Iterator<Item = Token>> InsertQueryParser<I> {
    pub fn new(tokens: I) -> InsertQueryParser<I> {
        InsertQueryParser {
            tokens: tokens
        }
    }

    fn parse_columns(&mut self) -> Vec<String> {
        let mut columns = vec![];
        while let Some(token) = self.tokens.next() {
            match token {
                Token::Comma => {},
                Token::Ident(col) => { columns.push(col); },
                Token::RParent => break,
                unexpected => panic!("panic find unexpected token {:?}", unexpected),
            }
        }
        columns
    }

    fn parse_values(&mut self) -> Vec<Value> {
        if self.tokens.next() != Some(Token::LParent) {
            unimplemented!();
        }
        let mut values = vec![];
        while let Some(token) = self.tokens.next() {
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

    fn parse(mut self) -> Statement {
        if self.tokens.next() != Some(Token::Into) {
            unimplemented!();
        }
        let table_name = match self.tokens.next() {
            Some(Token::Ident(table_name)) => table_name,
            _ => unimplemented!(),
        };
        let mut columns = vec![];

        let mut sub_query = false;
        while let Some(token) = self.tokens.next() {
            match token {
                Token::LParent => columns = self.parse_columns(),
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

        let query = if sub_query {
            InsertQuery::new(table_name, columns, ValueSource::SubQuery(SelectQueryParser::new(self.tokens).parse_select()))
        } else {
            let query = InsertQuery::new(table_name, columns, ValueSource::Row(self.parse_values()));
            if self.tokens.next() != Some(Token::Semicolon) {
                unimplemented!();
            }
            query
        };
        Statement::Insert(query)
    }
}

struct DeleteQueryParser<I: Iterator<Item = Token>> {
    tokens: I
}

impl<I: Iterator<Item = Token>> DeleteQueryParser<I> {
    pub fn new(tokens: I) -> DeleteQueryParser<I> {
        DeleteQueryParser {
            tokens: tokens
        }
    }

    fn parse(mut self) -> Statement {
        if self.tokens.next() != Some(Token::From) {
            unimplemented!();
        }

        let table_name = match self.tokens.next() {
            Some(Token::Ident(name)) => name,
            _ => unimplemented!(),
        };

        Statement::Delete(DeleteQuery::new(table_name, PredicateParser::new(self.tokens).parse_where()))
    }
}

struct SelectQueryParser<I: Iterator<Item = Token>> {
    tokens: I
}

impl<I: Iterator<Item = Token>> SelectQueryParser<I> {
    pub fn new(tokens: I) -> SelectQueryParser<I> {
        SelectQueryParser {
            tokens: tokens
        }
    }

    fn parse_select(mut self) -> SelectQuery {
        let columns = self.parse_columns_list();

        let table_name = match self.tokens.next() {
            Some(Token::Ident(table_name)) => table_name,
            _ => unimplemented!()
        };

        SelectQuery::new(table_name, columns, PredicateParser::new(self.tokens).parse_where())
    }

    fn parse_columns_list(&mut self) -> Vec<String> {
        let mut columns = vec![];
        loop {
            match self.tokens.next() {
                Some(Token::From) => break, // skip 'FROM' keyword
                Some(Token::Ident(column_name)) => columns.push(column_name),
                Some(Token::Comma) => {},
                t => panic!("unexpected token {:?} - ", t),
            }
        }
        columns
    }

    fn parse(self) -> Statement {
        Statement::Select(self.parse_select())
    }
}

struct PredicateParser<I: Iterator<Item = Token>> {
    tokens: I
}

impl<I: Iterator<Item = Token>> PredicateParser<I> {
    fn new(tokens: I) -> PredicateParser<I> {
        PredicateParser {
            tokens: tokens
        }
    }

    fn parse_where(&mut self) -> Option<Condition> {
        match self.tokens.next() {
            Some(Token::Where) => {
                let left = self.parse_predicate_arguments();

                let cond_type = match self.tokens.next() {
                    Some(Token::EqualSign) => CondType::Eq,
                    Some(Token::NotEqualSign) => CondType::NotEq,
                    _ => unimplemented!(),
                };
                let right = self.parse_predicate_arguments();
                Some(Condition::new(left, right, cond_type))
            },
            Some(Token::Semicolon) => None,
            _ => unimplemented!(),
        }
    }

    fn parse_predicate_arguments(&mut self) -> CondArg {
        match self.tokens.next() {
            Some(Token::CharsConst(s)) => CondArg::StringConstant(s),
            Some(Token::NumConst(s)) => CondArg::NumConst(s),
            Some(Token::Limit) => CondArg::Limit,
            Some(Token::Ident(s)) => CondArg::ColumnName(s),
            c => panic!("unexpected token - {:?}", c),
        }
    }
}
