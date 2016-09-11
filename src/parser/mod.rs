pub mod ast;

use std::iter::Peekable;

use super::lexer::Token;
use self::ast::{Type, CondType, Statement, CreateTableQuery, DeleteQuery, InsertQuery, SelectQuery, Condition, CondArg, Value};
use self::ast::table::Column;

pub enum Parser<I: Iterator<Item = Token>> {
    Create(CreateTableQueryParser<I>),
    Insert(InsertQueryParser<I>),
    Delete(DeleteQueryParser<I>),
    Select(SelectQueryParser<I>)
}

impl<I: Iterator<Item = Token>> QueryParser for Parser<I> {
    fn parse(self) -> Statement {
        match self {
            Parser::Create(q) => q.parse(),
            Parser::Insert(q) => q.parse(),
            Parser::Delete(q) => q.parse(),
            Parser::Select(q) => q.parse(),
        }
    }
}

pub trait QueryParser {
    fn parse(mut self) -> Statement;
}

pub trait IntoQueryParser<I: Iterator<Item = Token>> {
    fn into_parser(self) -> Parser<I>;
}

impl<I: Iterator<Item = Token>, II: IntoIterator<Item = Token, IntoIter = I>> IntoQueryParser<I> for II {
    fn into_parser(self) -> Parser<I> {
        let mut iter = self.into_iter().peekable();
        if let Some(Token::Ident(statement)) = iter.next() {
            match statement.as_str() {
                "create" => Parser::Create(CreateTableQueryParser::new(iter)),
                "delete" => Parser::Delete(DeleteQueryParser::new(iter)),
                "insert" => Parser::Insert(InsertQueryParser::new(iter)),
                "select" => Parser::Select(SelectQueryParser::new(iter)),
                _ => unimplemented!(),
            }
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct CreateTableQueryParser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>
}

impl<I: Iterator<Item = Token>> QueryParser for CreateTableQueryParser<I> {
    fn parse(mut self) -> Statement {
        Statement::Create(self.parse_create())
    }
}

impl<I: Iterator<Item = Token>> CreateTableQueryParser<I> {
    fn new(tokens: Peekable<I>) -> CreateTableQueryParser<I> {
        CreateTableQueryParser {
            tokens: tokens
        }
    }

    fn parse_create(&mut self) -> CreateTableQuery {
        self.tokens.next(); //skip 'TABLE' keyword
        match self.tokens.next() {
            Some(Token::Ident(name)) => CreateTableQuery::new(name, self.parse_table_columns()),
            _ => unimplemented!(),
        }
    }

    fn parse_table_columns(&mut self) -> Vec<Column> {
        if self.tokens.next() != Some(Token::LParent) {
            unimplemented!();
        }

        let mut columns = vec![];

        loop {
            let col_name = match self.tokens.next() {
                Some(Token::Ident(name)) => name,
                _ => unimplemented!(),
            };
            let col_type = match self.tokens.next() {
                Some(Token::Ident(column_type)) =>
                    match column_type.as_str() {
                        "int" => Type::Int,
                        "varchar" => {
                            self.tokens.next(); //skip '('
                            let size = match self.tokens.next() {
                                Some(Token::NumConst(s)) => match s.parse::<u8>() {
                                    Ok(s) => s,
                                    Err(e) => panic!(e),
                                },
                                _ => unimplemented!(),
                            };
                            self.tokens.next(); // skip ')'
                            Type::VarChar(size)
                        },
                        _ => unimplemented!(),
                    },
                _ => unimplemented!(),
            };

            columns.push(Column::new(col_name, col_type));

            match self.tokens.next() {
                Some(Token::Comma) => {}, //skip ','
                Some(Token::RParent) => break,
                _ => unimplemented!()
            }
        }

        //    tokens.next(); //skip ')'
        match self.tokens.peek() {
            Some(&Token::Semicolon) => { self.tokens.next(); } // skip ';'
            _ => unimplemented!()
        }

        columns
    }
}

pub struct DeleteQueryParser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>
}

impl<I: Iterator<Item = Token>> DeleteQueryParser<I> {
    pub fn new(tokens: Peekable<I>) -> DeleteQueryParser<I> {
        DeleteQueryParser {
            tokens: tokens
        }
    }

    fn parse_delete(&mut self) -> DeleteQuery {
        DeleteQuery::new(self.parse_from(), self.parse_where())
    }

    fn parse_from(&mut self) -> String {
        self.tokens.next(); //skip 'FROM' keyword
        match self.tokens.next() {
            Some(Token::Ident(table_name)) => table_name,
            _ => unimplemented!(),
        }
    }

    fn parse_where(&mut self) -> Option<Condition> {
        if let Some(Token::Ident(_)) = self.tokens.next() {
            //skip 'WHERE' keyword
            let left = self.parse_predicate_arguments();

            let cond_type = match self.tokens.next() {
                Some(Token::EqualSign) => CondType::Eq,
                Some(Token::NotEqualSign) => CondType::NotEq,
                _ => unimplemented!(),
            };
            let right = self.parse_predicate_arguments();
            Some(Condition::new(left, right, cond_type))
        } else {
            None
        }
    }

    fn parse_predicate_arguments(&mut self) -> CondArg {
        match self.tokens.next() {
            Some(Token::CharsConst(s)) => CondArg::StringConstant(s),
            Some(Token::NumConst(s)) => CondArg::NumConst(s),
            Some(Token::Ident(s)) => if s == "limit" {
                CondArg::Limit
            } else {
                CondArg::ColumnName(s)
            },
            c => panic!("unexpected token - {:?}", c),
        }
    }
}

impl<I: Iterator<Item = Token>> QueryParser for DeleteQueryParser<I> {
    fn parse(mut self) -> Statement {
        Statement::Delete(self.parse_delete())
    }
}

pub struct InsertQueryParser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>
}

impl<I: Iterator<Item = Token>> InsertQueryParser<I> {
    pub fn new(tokens: Peekable<I>) -> InsertQueryParser<I> {
        InsertQueryParser {
            tokens: tokens
        }
    }

    fn parse_insert(&mut self) -> InsertQuery {
        self.tokens.next(); //skip 'INTO' keyword
        if let Some(Token::Ident(table_name)) = self.tokens.next() {
            InsertQuery::new(table_name, self.parse_columns(), self.parse_values())
        } else {
            unimplemented!()
        }
    }

    fn parse_columns(&mut self) -> Vec<String> {
        match self.tokens.peek() {
            Some(&Token::LParent) => { self.tokens.next(); }, //skip '('
            _ => return vec![],
        }
        let mut columns = vec![];
        loop {
            match self.tokens.next() {
                Some(Token::Comma) => {},
                Some(Token::Ident(col)) => { columns.push(col); },
                Some(Token::RParent) => break,
                _ => unimplemented!(),
            }
        }
        columns
    }

    fn parse_values(&mut self) -> Vec<Value> {
        self.tokens.next(); //skip 'VALUES' keyword
        self.tokens.next(); //skip '('
        let mut values = vec![];
        loop {
            match self.tokens.next() {
                Some(Token::NumConst(s)) => values.push(Value::NumConst(s)),
                Some(Token::CharsConst(s)) => values.push(Value::StrConst(s)),
                Some(Token::Comma) => {},
                Some(Token::RParent) => break,
                c => panic!("panic find {:?}", c),
            }
        }
        values
    }
}

impl<I: Iterator<Item = Token>> QueryParser for InsertQueryParser<I> {
    fn parse(mut self) -> Statement {
        Statement::Insert(self.parse_insert())
    }
}

pub struct SelectQueryParser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>
}

impl<I: Iterator<Item = Token>> SelectQueryParser<I> {
    pub fn new(tokens: Peekable<I>) -> SelectQueryParser<I> {
        SelectQueryParser {
            tokens: tokens
        }
    }

    fn parse_from(&mut self) -> String {
        self.tokens.next(); //skip 'FROM' keyword
        match self.tokens.next() {
            Some(Token::Ident(table_name)) => table_name,
            _ => unimplemented!(),
        }
    }

    fn parse_where(&mut self) -> Option<Condition> {
        if let Some(Token::Ident(_)) = self.tokens.next() {
            //skip 'WHERE' keyword
            let left = self.parse_predicate_arguments();

            let cond_type = match self.tokens.next() {
                Some(Token::EqualSign) => CondType::Eq,
                Some(Token::NotEqualSign) => CondType::NotEq,
                _ => unimplemented!(),
            };
            let right = self.parse_predicate_arguments();
            Some(Condition::new(left, right, cond_type))
        } else {
            None
        }
    }

    fn parse_predicate_arguments(&mut self) -> CondArg {
        match self.tokens.next() {
            Some(Token::CharsConst(s)) => CondArg::StringConstant(s),
            Some(Token::NumConst(s)) => CondArg::NumConst(s),
            Some(Token::Ident(s)) => if s == "limit" {
                CondArg::Limit
            } else {
                CondArg::ColumnName(s)
            },
            c => panic!("unexpected token - {:?}", c),
        }
    }

    fn parse_select(&mut self) -> SelectQuery {
        let mut columns = vec![];

        loop {
            match self.tokens.next() {
                Some(Token::Ident(v)) => if v == "from" {
                    break; // skip 'FROM' keyword
                } else {
                    columns.push(v)
                },
                Some(Token::Comma) => {},
                _ => unimplemented!()
            }
        }

        let table_name = match self.tokens.next() {
            Some(Token::Ident(table_name)) => table_name,
            _ => unimplemented!()
        };

        SelectQuery::new(table_name, columns, self.parse_where())
    }
}

impl<I: Iterator<Item = Token>> QueryParser for SelectQueryParser<I> {
    fn parse(mut self) -> Statement {
        Statement::Select(self.parse_select())
    }
}
