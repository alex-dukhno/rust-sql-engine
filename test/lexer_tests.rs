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
                .to(be_ok().value(
                    vec![
                        IdentT("this".to_owned()),
                        IdentT("is".to_owned()),
                        IdentT("a".to_owned()),
                        IdentT("sentence".to_owned())
                    ]
                ));
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

        it "escapes new line chars" {
            expect!("\nword".to_owned().tokenize())
                .to(be_ok().value(vec![IdentT("word".to_owned())]));
        }

        it "escapes tabs" {
            expect!("\tword".to_owned().tokenize())
                .to(be_ok().value(vec![IdentT("word".to_owned())]));
        }

        it "emits error when string token is not closed" {
            expect!("\'str".to_owned().tokenize())
                .to(be_err().value("string const should be closed by \'".to_owned()));
        }

        it "case insensitive" {
            expect!("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned().tokenize())
                .to(be_ok().value(vec![IdentT("abcdefghijklmnopqrstuvwxyz".to_owned())]));
        }
    }
}
