pub use expectest::prelude::{be_ok, be_err};

pub use sql::lexer::Tokenizer;
pub use sql::lexer::Token::{self, Identifier, LeftParenthesis, RightParenthesis, Comma, SingleQuote, Semicolon, EqualSign, Asterisk, NumberValue, StringValue};

describe! lexer {

    describe! lexems {

        it "emits None when given an empty string" {
            expect!("".to_owned().tokenize())
                .to(be_ok().value(vec![]));
        }

        it "emits identifier token when given a single word string" {
            expect!("word".to_owned().tokenize())
                .to(be_ok().value(vec![Identifier("word".to_owned())]));
        }

        it "emits identifiers when given string of words" {
            expect!("this is a sentence".to_owned().tokenize())
                .to(be_ok().value(vec![Identifier("this".to_owned()), Identifier("is".to_owned()), Identifier("a".to_owned()), Identifier("sentence".to_owned())]));
        }

        it "emits number token when given number" {
            expect!("5".to_owned().tokenize())
                .to(be_ok().value(vec![NumberValue("5".to_owned())]));
        }

        it "emits number token when given number with float point" {
            expect!("2.01".to_owned().tokenize())
                .to(be_ok().value(vec![NumberValue("2.01".to_owned())]));
        }

        it "emits error when given number with two delimeters" {
            expect!("2.0.1".to_owned().tokenize())
                .to(be_err().value("Number format error"));
        }

        it "escapes single quote inside string token" {
            expect!("\'str\'\'str\'".to_owned().tokenize())
                .to(be_ok().value(vec![StringValue("str\'str".to_owned())]));
        }
    }

    describe! sql_statements {

        it "emits lexems of sql insert statement" {
            expect!("insert into table values(10, 'str');".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        Identifier("insert".to_owned()),
                        Identifier("into".to_owned()),
                        Identifier("table".to_owned()),
                        Identifier("values".to_owned()),
                        LeftParenthesis,
                        NumberValue("10".to_owned()),
                        Comma,
                        StringValue("str".to_owned()),
                        RightParenthesis,
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql delete statement" {
            expect!("delete from table_name where col_name = 'five';".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        Identifier("delete".to_owned()),
                        Identifier("from".to_owned()),
                        Identifier("table_name".to_owned()),
                        Identifier("where".to_owned()),
                        Identifier("col_name".to_owned()),
                        EqualSign,
                        StringValue("five".to_owned()),
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql update statement" {
            expect!("update table_name set col_one=val1,col_two='val2' where col_three=3".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        Identifier("update".to_owned()),
                        Identifier("table_name".to_owned()),
                        Identifier("set".to_owned()),
                        Identifier("col_one".to_owned()),
                        EqualSign,
                        Identifier("val1".to_owned()),
                        Comma,
                        Identifier("col_two".to_owned()),
                        EqualSign,
                        StringValue("val2".to_owned()),
                        Identifier("where".to_owned()),
                        Identifier("col_three".to_owned()),
                        EqualSign,
                        NumberValue("3".to_owned())
                    ]
                ));
        }

        it "emits lexems of sql select statement" {
            expect!("select count(*),count(col1)from table_name".to_owned().tokenize())
                .to(be_ok().value(
                    vec![
                        Identifier("select".to_owned()),
                        Identifier("count".to_owned()),
                        LeftParenthesis,
                        Asterisk,
                        RightParenthesis,
                        Comma,
                        Identifier("count".to_owned()),
                        LeftParenthesis,
                        Identifier("col1".to_owned()),
                        RightParenthesis,
                        Identifier("from".to_owned()),
                        Identifier("table_name".to_owned())
                    ]
                ));
        }
    }
}
