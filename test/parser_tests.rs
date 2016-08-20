pub use expectest::prelude::be_ok;

pub use sql::lexer::Token::{IdentT, NumberT, StringT, Semicolon, EqualSign, LeftParenthesis, RightParenthesis, Comma};
pub use sql::parser::Condition::{Eq};
pub use sql::parser::Parser;
pub use sql::parser::Node::{Delete, From, Where, Id, Const, Table, Values, Insert, Column};

describe! parser {

    describe! delete_statements {

        it "parses delete statement" {
            let tokens = vec![
                IdentT("delete".to_owned()),
                IdentT("from".to_owned()),
                IdentT("table_name".to_owned()), Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Delete(
                        Box::new(From("table_name".to_owned())),
                        Box::new(Where(None))
                    )
                ));
        }

        it "parses delete statement with predicate" {
            let tokens = vec![
                IdentT("delete".to_owned()),
                IdentT("from".to_owned()),
                IdentT("table_name".to_owned()),
                IdentT("where".to_owned()),
                IdentT("col".to_owned()),
                EqualSign,
                NumberT("5".to_owned()),
                Semicolon
            ];

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
            let tokens = vec![
                IdentT("insert".to_owned()),
                IdentT("into".to_owned()),
                IdentT("table_name".to_owned()),
                IdentT("values".to_owned()),
                LeftParenthesis,
                NumberT("10".to_owned()),
                Comma,
                StringT("string".to_owned()),
                RightParenthesis,
                Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Insert(
                        Box::new(Table("table_name".to_owned(), None)),
                        Box::new(Values(vec![Const("10".to_owned()), Const("string".to_owned())]))
                    )
                ));
        }

        it "parses insert statement with columns" {
            let tokens = vec![
                IdentT("insert".to_owned()),
                IdentT("into".to_owned()),
                IdentT("table_name".to_owned()),
                LeftParenthesis,
                IdentT("col1".to_owned()),
                Comma,
                IdentT("col2".to_owned()),
                RightParenthesis,
                IdentT("values".to_owned()),
                LeftParenthesis,
                NumberT("10".to_owned()),
                Comma,
                StringT("string".to_owned()),
                RightParenthesis,
                Semicolon
            ];

            expect!(tokens.parse())
                .to(be_ok().value(
                    Insert(
                        Box::new(Table("table_name".to_owned(), Some(vec![Column("col1".to_owned(), None), Column("col2".to_owned(), None)]))),
                        Box::new(Values(vec![Const("10".to_owned()), Const("string".to_owned())]))
                    )
                ));
        }
    }
}
