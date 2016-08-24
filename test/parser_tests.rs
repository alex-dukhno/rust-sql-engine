pub use expectest::prelude::{be_ok, be_err};

pub use sql::lexer::Token::{self, IdentT, NumberT, StringT, Semicolon, EqualSign, LeftParenthesis, RightParenthesis, Comma};
pub use sql::parser::Condition::{Eq};
pub use sql::parser::Parser;
pub use sql::parser::Node::{self, Delete, From, Where, Id, Const, Table, Values, Insert, Column, TableColumn};
pub use sql::parser::Type::{self, Int};

describe! parser {

    describe! create_table_statements {

        it "parses create table statement" {
            let tokens = vec![
                IdentT("create".to_owned()),
                IdentT("table".to_owned()),
                IdentT("table_name".to_owned()),
                LeftParenthesis,
                IdentT("col".to_owned()),
                IdentT("int".to_owned()),
                RightParenthesis,
                Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Node::Create(
                        Box::new(Table(
                            "table_name".to_owned(),
                            vec![TableColumn("col".to_owned(), Some(Int), None)]
                        ))
                    )
                ));
        }

        it "parses create table with list of columns statement" {
            let tokens = vec![
                IdentT("create".to_owned()),
                IdentT("table".to_owned()),
                IdentT("table_name".to_owned()),
                LeftParenthesis,
                IdentT("col1".to_owned()),
                IdentT("int".to_owned()),
                Comma,
                IdentT("col2".to_owned()),
                IdentT("int".to_owned()),
                Comma,
                IdentT("col3".to_owned()),
                IdentT("int".to_owned()),
                RightParenthesis,
                Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Node::Create(
                        Box::new(Table(
                            "table_name".to_owned(),
                            vec![
                                TableColumn("col1".to_owned(), Some(Int), None),
                                TableColumn("col2".to_owned(), Some(Int), None),
                                TableColumn("col3".to_owned(), Some(Int), None)
                            ]
                        ))
                    )
                ));
        }

        it "parses create table without comma in column list" {
            let tokens = vec![
                IdentT("create".to_owned()),
                IdentT("table".to_owned()),
                IdentT("table_name".to_owned()),
                LeftParenthesis,
                IdentT("col1".to_owned()),
                IdentT("int".to_owned()),
                IdentT("col2".to_owned()),
                IdentT("int".to_owned()),
                RightParenthesis,
                Semicolon
            ];


            expect!(tokens.parse())
                .to(be_err().value("parsing error missing ','".to_owned()));
        }
    }

    describe! delete_statements {

        it "parses delete statement" {
            let tokens = vec![delete(), from(), table_name(), Semicolon];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Delete(
                        Box::new(From("table_name".to_owned())),
                        Box::new(Where(None))
                    )
                ));
        }

        it "parses delete statement with predicate" {
            let tokens = vec![delete(), from(), table_name(), where_t(), column_name(), EqualSign, five_int(), Semicolon];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Delete(
                        Box::new(From("table_name".to_owned())),
                        Box::new(Where(Some(
                            Eq(
                                Box::new(Id("col".to_owned())),
                                Box::new(Const("5".to_owned()))
                            )
                        )))
                    )
                ));
        }
    }

    describe! insert_statements {

        it "parses insert statement" {
            let tokens = vec![insert(), into(), table_name(), values(), LeftParenthesis, ten_int(), Comma, string(), RightParenthesis, Semicolon];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Insert(
                        Box::new(Table("table_name".to_owned(), vec![])),
                        Box::new(Values(vec![Const("10".to_owned()), Const("string".to_owned())]))
                    )
                ));
        }

        it "parses insert statement with columns" {
            let tokens = vec![
                insert(), into(), table_name(), LeftParenthesis, column_1_name(), Comma, column_2_name(), RightParenthesis,
                                    values(), LeftParenthesis, ten_int(), Comma, string(), RightParenthesis, Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Insert(
                        Box::new(Table("table_name".to_owned(), vec![Column("col1".to_owned()), Column("col2".to_owned())])),
                        Box::new(Values(vec![Const("10".to_owned()), Const("string".to_owned())]))
                    )
                ));
        }
    }
}

pub fn delete() -> Token {
    IdentT("delete".to_owned())
}

pub fn from() -> Token {
    IdentT("from".to_owned())
}

pub fn where_t() -> Token {
    IdentT("where".to_owned())
}

pub fn column_name() -> Token {
    IdentT("col".to_owned())
}

pub fn five_int() -> Token {
    NumberT("5".to_owned())
}

pub fn insert() -> Token {
    IdentT("insert".to_owned())
}

pub fn into() -> Token {
    IdentT("into".to_owned())
}

pub fn table_name() -> Token {
    IdentT("table_name".to_owned())
}

pub fn column_1_name() -> Token {
    IdentT("col1".to_owned())
}

pub fn column_2_name() -> Token {
    IdentT("col2".to_owned())
}

pub fn values() -> Token {
    IdentT("values".to_owned())
}

pub fn ten_int() -> Token {
    NumberT("10".to_owned())
}

pub fn string() -> Token {
    StringT("string".to_owned())
}
