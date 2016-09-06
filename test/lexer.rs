use expectest::prelude::be_equal_to;

use sql::lexer::Tokenizer;
use sql::lexer::Token::{Ident, NumericConstant, CharactersConstant};

#[test]
fn emits_none_when_given_an_empty_string() {
    expect!(Tokenizer::from("").tokenize())
        .to(be_equal_to(vec![]));
}

#[test]
fn emits_identifier_token_when_given_a_single_word_string() {
    expect!(Tokenizer::from("word").tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_identifiers_when_given_string_of_words() {
    expect!(Tokenizer::from("this is a sentence").tokenize())
        .to(be_equal_to(
            vec![
                Ident("this".to_owned()),
                Ident("is".to_owned()),
                Ident("a".to_owned()),
                Ident("sentence".to_owned())
            ]
        ));
}

#[test]
fn emits_number_token_when_given_number() {
    expect!(Tokenizer::from("5").tokenize())
        .to(be_equal_to(vec![NumericConstant("5".to_owned())]));
}

#[test]
fn escapes_single_quote_inside_string_token() {
    expect!(Tokenizer::from("\'str\'\'str\'").tokenize())
        .to(be_equal_to(vec![CharactersConstant("str\'str".to_owned())]));
}

#[test]
fn escapes_new_line_chars() {
    expect!(Tokenizer::from("\nword").tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn escapes_tabs() {
    expect!(Tokenizer::from("\tword").tokenize())
        .to(be_equal_to(vec![Ident("word".to_owned())]));
}

#[test]
fn emits_string_when_only_open_signle_quote() {
    expect!(Tokenizer::from("\'str").tokenize())
        .to(be_equal_to(vec![CharactersConstant("str".to_owned())]));
}

#[test]
fn case_insensitive() {
    expect!(Tokenizer::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ").tokenize())
        .to(be_equal_to(vec![Ident("abcdefghijklmnopqrstuvwxyz".to_owned())]));
}

#[cfg(test)]
mod sql_query {

    use expectest::prelude::be_equal_to;

    use sql::lexer::Tokenizer;
    use sql::lexer::Token::{Ident, LeftParenthesis, NumericConstant, CharactersConstant, RightParenthesis, Semicolon};

    #[test]
    fn tokenize_insert_query_numeric_value() {
        expect!(Tokenizer::from("insert into table_name values(1);").tokenize())
            .to(be_equal_to(vec![Ident("insert".to_owned()), Ident("into".to_owned()), Ident("table_name".to_owned()), Ident("values".to_owned()), LeftParenthesis, NumericConstant("1".to_owned()), RightParenthesis, Semicolon]));
    }

    #[test]
    fn tokenize_insert_query_varchar_value() {
        expect!(Tokenizer::from("insert into table_name values('string');").tokenize())
            .to(be_equal_to(vec![Ident("insert".to_owned()), Ident("into".to_owned()), Ident("table_name".to_owned()), Ident("values".to_owned()), LeftParenthesis, CharactersConstant("string".to_owned()), RightParenthesis, Semicolon]));
    }
}
