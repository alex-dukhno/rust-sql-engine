pub use expectest::prelude::be_equal_to;

pub use sql::lexer::Tokenizer;
pub use sql::lexer::Token::{self, Identifier, LeftParenthesis, RightParenthesis, Comma, SingleQuote, Semicolon, EqualSign, Asterisk};

describe! lexer {

    describe! lexems {


        it "emits None when given an empty string" {
            expect!("".to_owned().tokenize())
                .to(be_equal_to(vec![]));
        }

        it "emits identifier token when given a single word string" {
            expect!("word".to_owned().tokenize())
                .to(be_equal_to(vec![Identifier("word".to_owned())]));
        }

        it "emits identifiers when given string of words" {
            expect!("This is a sentence".to_owned().tokenize())
                .to(be_equal_to(vec![Identifier("This".to_owned()), Identifier("is".to_owned()), Identifier("a".to_owned()), Identifier("sentence".to_owned())]));
        }
    }

    describe! sql_lexems {

        it "emits lexems of sql insert statement" {
            expect!("insert into table1 values(val1, 'val2');".to_owned().tokenize())
                .to(be_equal_to(
                    vec![
                        Identifier("insert".to_owned()),
                        Identifier("into".to_owned()),
                        Identifier("table1".to_owned()),
                        Identifier("values".to_owned()),
                        LeftParenthesis,
                        Identifier("val1".to_owned()),
                        Comma,
                        SingleQuote,
                        Identifier("val2".to_owned()),
                        SingleQuote,
                        RightParenthesis,
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql delete statement" {
            expect!("delete from table_name where col_name = 'five';".to_owned().tokenize())
                .to(be_equal_to(
                    vec![
                        Identifier("delete".to_owned()),
                        Identifier("from".to_owned()),
                        Identifier("table_name".to_owned()),
                        Identifier("where".to_owned()),
                        Identifier("col_name".to_owned()),
                        EqualSign,
                        SingleQuote,
                        Identifier("five".to_owned()),
                        SingleQuote,
                        Semicolon
                    ]
                ));
        }

        it "emits lexems of sql update statement" {
            expect!("update table_name set col1=val1,col2='val2' where col3=val3".to_owned().tokenize())
                .to(be_equal_to(
                    vec![
                        Identifier("update".to_owned()),
                        Identifier("table_name".to_owned()),
                        Identifier("set".to_owned()),
                        Identifier("col1".to_owned()),
                        EqualSign,
                        Identifier("val1".to_owned()),
                        Comma,
                        Identifier("col2".to_owned()),
                        EqualSign,
                        SingleQuote,
                        Identifier("val2".to_owned()),
                        SingleQuote,
                        Identifier("where".to_owned()),
                        Identifier("col3".to_owned()),
                        EqualSign,
                        Identifier("val3".to_owned())
                    ]
                ));
        }

        it "emits lexems of sql select statement" {
            expect!("select count(*),count(col1)from table_name".to_owned().tokenize())
                .to(be_equal_to(
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
