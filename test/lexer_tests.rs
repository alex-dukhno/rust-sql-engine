pub use expectest::prelude::{be_ok, be_err};

pub use sql::lexer::Tokenizer;
pub use sql::lexer::Token::{IdentT, LeftParenthesis, RightParenthesis, Comma, SingleQuote, Semicolon, EqualSign, Asterisk, NumberT, StringT};

describe! lexer {

    describe! lexems {

        it "emits None when given an empty string" {
            expect!("".to_owned().tokenize())
                .to(be_ok().value(vec![]));
        }

        it "emits identifier token when given a single word string" {
            expect!("word".to_owned().tokenize())
                .to(be_ok().value(vec![IdentT("word".to_owned())]));
        }

        it "emits identifiers when given string of words" {
            expect!("this is a sentence".to_owned().tokenize())
                .to(be_ok().value(vec![IdentT("this".to_owned()), IdentT("is".to_owned()), IdentT("a".to_owned()), IdentT("sentence".to_owned())]));
        }

        it "emits number token when given number" {
            expect!("5".to_owned().tokenize())
                .to(be_ok().value(vec![NumberT("5".to_owned())]));
        }

        it "emits number token when given number with float point" {
            expect!("2.01".to_owned().tokenize())
                .to(be_ok().value(vec![NumberT("2.01".to_owned())]));
        }

        it "emits error when given number with two delimeters" {
            expect!("2.0.1".to_owned().tokenize())
                .to(be_err().value("Number format error"));
        }

        it "escapes single quote inside string token" {
            expect!("\'str\'\'str\'".to_owned().tokenize())
                .to(be_ok().value(vec![StringT("str\'str".to_owned())]));
        }
    }

    describe! sql_statements {

        it "tokenizes create table query" {
            expect!("create table table_name (col int);".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        IdentT("create".to_owned()),
                        IdentT("table".to_owned()),
                        IdentT("table_name".to_owned()),
                        LeftParenthesis,
                        IdentT("col".to_owned()),
                        IdentT("int".to_owned()),
                        RightParenthesis,
                        Semicolon
                    ]
                ));
        }

        it "tokenizes table creation with list of columns" {
            expect!("create table table_name (\n\tcol1 int,\n\tcol2 int,\n\tcol3 int);".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
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
                    ]
                ));
        }

        it "emits lexems of sql insert statement" {
            expect!("insert into table values(10, 'str');".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        IdentT("insert".to_owned()),
                        IdentT("into".to_owned()),
                        IdentT("table".to_owned()),
                        IdentT("values".to_owned()),
                        LeftParenthesis,
                        NumberT("10".to_owned()),
                        Comma,
                        StringT("str".to_owned()),
                        RightParenthesis,
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql delete statement" {
            expect!("delete from table_name where col_name = 'five';".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        IdentT("delete".to_owned()),
                        IdentT("from".to_owned()),
                        IdentT("table_name".to_owned()),
                        IdentT("where".to_owned()),
                        IdentT("col_name".to_owned()),
                        EqualSign,
                        StringT("five".to_owned()),
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql update statement" {
            expect!("update table_name set col_one=val1,col_two='val2' where col_three=3".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        IdentT("update".to_owned()),
                        IdentT("table_name".to_owned()),
                        IdentT("set".to_owned()),
                        IdentT("col_one".to_owned()),
                        EqualSign,
                        IdentT("val1".to_owned()),
                        Comma,
                        IdentT("col_two".to_owned()),
                        EqualSign,
                        StringT("val2".to_owned()),
                        IdentT("where".to_owned()),
                        IdentT("col_three".to_owned()),
                        EqualSign,
                        NumberT("3".to_owned())
                    ]
                ));
        }

        it "emits lexems of sql select statement" {
            expect!("select count(*),count(col1)from table_name".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        IdentT("select".to_owned()),
                        IdentT("count".to_owned()),
                        LeftParenthesis,
                        Asterisk,
                        RightParenthesis,
                        Comma,
                        IdentT("count".to_owned()),
                        LeftParenthesis,
                        IdentT("col1".to_owned()),
                        RightParenthesis,
                        IdentT("from".to_owned()),
                        IdentT("table_name".to_owned())
                    ]
                ));
        }
    }
}
